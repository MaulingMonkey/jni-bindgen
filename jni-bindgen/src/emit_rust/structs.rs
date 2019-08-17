use super::*;

use java::class;

use std::collections::*;
use std::error::Error;
use std::fmt::Write;
use std::io;
use std::path::PathBuf;

#[derive(Debug, Default)]
pub(crate) struct StructPaths {
    pub mod_prefix:         String,
    pub struct_name:        String,
    pub feature_name:       String,
    pub sharded_path:       PathBuf,
}

impl StructPaths {
    pub(crate) fn new<'ctx>(context: &'ctx Context, class: class::Id) -> Result<Self, Box<dyn Error>> {
        Ok(Self{
            mod_prefix:     Struct::mod_for(context, class)? + "::",
            struct_name:    Struct::name_for(context, class)?,
            feature_name:   Struct::feature_for(context, class)?,
            sharded_path:   Struct::sharded_path_for(context, class)?,
        })
    }

    pub(crate) fn local_scope(&self) -> Option<impl Iterator<Item = &str>> {
        let mut iter = self.mod_prefix.split("::");
        if iter.next() == Some("crate") {
            Some(iter.filter(|s| !s.is_empty()))
        } else {
            None
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct Struct {
    pub rust:   StructPaths,
    pub java:   java::Class,
}

fn rust_id<'a>(id: &str) -> Result<&str, Box<dyn Error>> {
    Ok(match RustIdentifier::from_str(id) {
        RustIdentifier::Identifier(id) => id,
        RustIdentifier::KeywordRawSafe(id) => id,
        RustIdentifier::KeywordUnderscorePostfix(id) => id,
        RustIdentifier::NonIdentifier(id) => io_data_err!("Unable to add_struct(): java identifier {:?} has no rust equivalent (yet?)", id)?,
    })
}

fn feature_id<'a>(id: &str) -> Result<&str, Box<dyn Error>> {
    Ok(match RustIdentifier::from_str(id) {
        RustIdentifier::Identifier(id) => id,
        RustIdentifier::KeywordRawSafe(_) => id,
        RustIdentifier::KeywordUnderscorePostfix(_) => id,
        RustIdentifier::NonIdentifier(id) => io_data_err!("Unable to add_struct(): java identifier {:?} has no rust equivalent (yet?)", id)?,
    })
}

impl Struct {
    pub(crate) fn feature_for(_context: &Context, class: class::Id) -> Result<String, Box<dyn Error>> {
        let mut buf = String::new();
        for component in class.iter() {
            match component {
                class::IdPart::Namespace(id)        => write!(&mut buf, "{}-",  feature_id(id)?)?,
                class::IdPart::ContainingClass(id)  => write!(&mut buf, "{}_",  feature_id(id)?)?,
                class::IdPart::LeafClass(id)        => write!(&mut buf, "{}",   feature_id(id)?)?,
            }
        }
        Ok(buf)
    }

    pub(crate) fn mod_for(_context: &Context, class: class::Id) -> Result<String, Box<dyn Error>> {
        let mut buf = String::from("crate");
        for component in class.iter() {
            match component {
                class::IdPart::Namespace(id)        => write!(&mut buf, "::{}", rust_id(id)?)?,
                class::IdPart::ContainingClass(_)   => {},
                class::IdPart::LeafClass(_)         => {},
            }
        }
        Ok(buf)
    }

    pub(crate) fn name_for(_context: &Context, class: class::Id) -> Result<String, Box<dyn Error>> {
        let mut buf = String::new();
        for component in class.iter() {
            match component {
                class::IdPart::Namespace(_)         => {},
                class::IdPart::ContainingClass(id)  => write!(&mut buf, "{}_",  rust_id(id)?)?,
                class::IdPart::LeafClass(id)        => write!(&mut buf, "{}",   rust_id(id)?)?,
            }
        }
        Ok(buf)
    }

    pub(crate) fn sharded_path_for(context: &Context, class: class::Id) -> Result<PathBuf, Box<dyn Error>> {
        let mut buf = String::new();

        if let Some(name) = context.config.output_path.file_stem() {
            write!(&mut buf, "{}/", name.to_string_lossy())?;
        }

        for component in class.iter() {
            match component {
                class::IdPart::Namespace(id)        => write!(&mut buf, "{}/",    rust_id(id)?)?,
                class::IdPart::ContainingClass(id)  => write!(&mut buf, "{}_",    rust_id(id)?)?,
                class::IdPart::LeafClass(id)        => write!(&mut buf, "{}.rs",  rust_id(id)?)?,
            }
        }

        Ok(PathBuf::from(buf))
    }

    pub(crate) fn new<'ctx>(context: &'ctx mut Context, java: java::Class) -> Result<Self, Box<dyn Error>> {
        let rust = StructPaths::new(context, java.path.as_id())?;

        return Ok(Self {
            rust,
            java,
        });
    }

    pub(crate) fn write(&self, context: &Context, indent: &str, out: &mut impl io::Write) -> io::Result<()> {
        writeln!(out, "")?;

        // Ignored access_flags: SUPER, SYNTHETIC, ANNOTATION, ABSTRACT

        let keyword = if self.java.is_interface() {
            "interface"
        } else if self.java.is_enum() {
            "enum"
        } else if self.java.is_static() {
            "static java"
        } else if self.java.is_final() {
            "final class"
        } else {
            "class"
        };

        let visibility = if self.java.is_public() {
            "public"
        } else {
            "private"
        };

        let attributes = format!("{}",
            if self.java.deprecated { "#[deprecated] " } else { "" }
        );

        let super_path = if let Some(super_path) = self.java.super_path.as_ref() {
            context.java_to_rust_path(super_path.as_id()).unwrap()
        } else {
            "()".to_owned() // This might only happen for java.lang.Object
        };

        if let Ok(required_feature) = Struct::feature_for(context, self.java.path.as_id()) {
            writeln!(out, "{}#[cfg(any(feature = \"all\", feature = {:?}))]", indent, required_feature)?;
        }
        writeln!(out, "{}__jni_bindgen! {{", indent)?;
        if let Some(url) = KnownDocsUrl::from_class(context, self.java.path.as_id()) {
            writeln!(out, "{}    /// {} {} {}", indent, visibility, keyword, url)?;
        } else {
            writeln!(out, "{}    /// {} {} {}", indent, visibility, keyword, self.java.path.as_str())?;
        }
        if let Ok(required_feature) = Struct::feature_for(context, self.java.path.as_id()) {
            writeln!(out, "{}    ///", indent)?;
            writeln!(out, "{}    /// Required feature: {}", indent, required_feature)?;
        }
        write!(out, "{}    {}{} {} {} ({:?}) extends {}", indent, attributes, visibility, keyword, &self.rust.struct_name, self.java.path.as_str(), super_path)?;
        let mut implements = false;
        for interface in &self.java.interfaces {
            write!(out, ", ")?;
            if !implements {
                write!(out, "implements ")?;
                implements = true;
            }
            write!(out, "{}", &context.java_to_rust_path(interface.as_id()).unwrap())?;
        }
        writeln!(out, " {{")?;

        let mut id_repeats = HashMap::new();

        let mut methods : Vec<Method> = self.java.methods.iter().map(|m| Method::new(context, &self.java, m)).collect();
        let mut fields  : Vec<Field > = self.java.fields.iter().map(|f| Field::new(context, &self.java, f)).collect();

        for method in &methods {
            if !method.java.is_public() { continue; } // Skip private/protected methods
            if let Some(name) = method.rust_name() {
                *id_repeats.entry(name.to_owned()).or_insert(0) += 1;
            }
        }

        for field in &fields {
            if !field.java.is_public() { continue; } // Skip private/protected fields
            match field.rust_names.as_ref() {
                Ok(FieldMangling::ConstValue(name, _)) => { *id_repeats.entry(name.to_owned()).or_insert(0) += 1; },
                Ok(FieldMangling::GetSet(get, set)) => {
                    *id_repeats.entry(get.to_owned()).or_insert(0) += 1;
                    *id_repeats.entry(set.to_owned()).or_insert(0) += 1;
                },
                Err(_) => {},
            }
        }

        for method in &mut methods {
            if let Some(name) = method.rust_name() {
                let repeats = *id_repeats.get(name).unwrap_or(&0);
                let overloaded = repeats > 1;
                if overloaded {
                    method.set_mangling_style(context.config.codegen.method_naming_style_collision);
                }
            }

            method.emit(context, indent, out)?;
        }

        for field in &mut fields {
            field.emit(context, indent, out)?;
        }

        writeln!(out, "{}    }}", indent)?;
        writeln!(out, "{}}}", indent)?;
        Ok(())
    }
}

fn emit_cstr(s: &str) -> String {
    let mut s = format!("{:?}", s); // XXX
    s.insert_str(s.len() - 1, "\\0");
    s
}
