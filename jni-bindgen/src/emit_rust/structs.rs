use super::*;

use std::collections::*;
use std::io;

#[derive(Debug, Default)]
pub(crate) struct Struct {
    pub rust_mod_prefix:    String,
    pub rust_struct_name:   String,
    pub java:               java::Class,
}

impl Struct {
    pub(crate) fn write(&self, context: &Context, indent: &str, out: &mut impl io::Write) -> io::Result<()> {
        writeln!(out, "")?;
        self.write_rust_struct(context, indent, out)?;
        Ok(())
    }

    fn write_rust_struct(&self, context: &Context, indent: &str, out: &mut impl io::Write) -> io::Result<()> {
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

        writeln!(out, "{}__jni_bindgen! {{", indent)?;
        if let Some(url) = KnownDocsUrl::from_class(context, self.java.path.as_id()) {
            writeln!(out, "{}    /// {} {} {}", indent, visibility, keyword, url)?;
        }
        write!(out, "{}    {}{} {} {} extends {}", indent, attributes, visibility, keyword, &self.rust_struct_name, super_path)?;
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
