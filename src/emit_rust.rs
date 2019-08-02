use super::*;
use class_file_visitor::*;
use gather_java::*;

use std::collections::*;
use std::error::Error;
use std::fmt::Write;
use std::io;

struct KnownDocsUrl {
    label:  String,
    url:    String,
}

impl KnownDocsUrl {
    pub fn from(java_class: &Class) -> Option<KnownDocsUrl> {
        let java_name = java_class.this_class().name();

        //let prefix = if java_name.starts_with("android/") {
        //    "https://developer.android.com/reference/kotlin/"
        //} else if java_name.starts_with("java/") {
        //    "https://docs.oracle.com/javase/7/docs/api/index.html?"
        //} else {
        //    return None;
        //};

        let prefix = "https://developer.android.com/reference/kotlin/";

        for ch in java_name.chars() {
            match ch {
                'a'..='z' => {},
                'A'..='Z' => {},
                '0'..='9' => {},
                '_' | '$' | '/' => {},
                _ch => return None,
            }
        }

        let last_slash = java_name.rfind(|ch| ch == '/');
        let no_namespace = if let Some(last_slash) = last_slash {
            &java_name[(last_slash+1)..]
        } else {
            &java_name[..]
        };

        Some(KnownDocsUrl{
            label:  no_namespace.to_owned().replace("$","."),
            url:    format!("{}{}.html", prefix, java_name.replace("$",".")),
        })
    }
}

const DEREF_FN : &'static str = "fn deref(&self) -> &Self::Target { unsafe { &*(self as *const Self as *const Self::Target) } }";

#[derive(Debug, Default)]
pub struct Struct {
    pub rust_mod_prefix:    String,
    pub rust_struct_name:   String,
    pub java_class:         Class,
}

impl Struct {
    pub fn write(&self, context: &Context, indent: &str, out: &mut impl io::Write) -> io::Result<()> {
        if let Some(url) = KnownDocsUrl::from(&self.java_class) {
            writeln!(out, "{}/// [{}]({})", indent, url.label, url.url)?;
        }
        //self.write_java_doc_comment(context, indent, out)?;
        //writeln!(out, "{}/// * access_flags: {:?}", indent, self.java_class.access_flags() )?;
        self.write_rust_struct(context, indent, out)?;
        Ok(())
    }

    fn write_java_doc_comment(&self, _context: &Context, indent: &str, out: &mut impl io::Write) -> io::Result<()> {
        // Emit an initial doc comment in the form of a proto-Java class definition.
        writeln!(out, "{}/// ```java", indent)?;
        write!(out, "{}/// ", indent)?;
        // Ignored access_flags: SUPER, SYNTHETIC, ANNOTATION
        if self.java_class.access_flags().contains(ClassAccessFlags::PUBLIC  ) { write!(out, "public ")? }
        if self.java_class.access_flags().contains(ClassAccessFlags::STATIC  ) { write!(out, "static ")? }
        if self.java_class.access_flags().contains(ClassAccessFlags::ABSTRACT) { write!(out, "abstract ")? }
        if self.java_class.access_flags().contains(ClassAccessFlags::FINAL   ) { write!(out, "final ")? }
        let keyword = if self.java_class.access_flags().contains(ClassAccessFlags::INTERFACE) {
            "interface"
        } else if self.java_class.access_flags().contains(ClassAccessFlags::ENUM) {
            "enum"
        } else {
            "class"
        };
        write!(out, "{} {}", keyword, self.java_class.this_class().name().replace("/", "."))?;
        if let Some(super_class) = self.java_class.super_class() {
            if super_class.name() != "java/lang/Object" {
                write!(out, " extends {}", super_class.name().replace("/", "."))?;
            }
        }
        let mut write_implements = false;
        for interface in self.java_class.interfaces() {
            if !write_implements {
                write!(out, " implements ")?;
                write_implements = true;
            } else {
                write!(out, ", ")?;
            }
            write!(out, "{}", interface.name().replace("/", "."))?;
        }
        writeln!(out, " {{ ... }}")?;
        writeln!(out, "{}/// ```", indent)?;
        Ok(())
    }

    fn write_rust_struct(&self, context: &Context, indent: &str, out: &mut impl io::Write) -> io::Result<()> {
        let struct_access = if self.java_class.access_flags().contains(ClassAccessFlags::PUBLIC) {
            "pub "
        } else {
            ""
        };

        let struct_body = if self.java_class.access_flags().contains(ClassAccessFlags::STATIC) {
            ""
        } else {
            "(::jni_sys::jobject)"
        };

        writeln!(out, "{}#[repr(transparent)] {}struct {}{};", indent, struct_access, &self.rust_struct_name, struct_body)?;
        if let Some(super_class) = self.java_class.super_class() {
            let rust_super_class_name = context.jni_type_to_rust_type.get(super_class.name()).unwrap();
            //let rust_super_name = "::java::lang::Object"; // XXX
            writeln!(out, "{}impl ::std::ops::Deref for {} {{ type Target = {}; {} }}", indent, &self.rust_struct_name, rust_super_class_name, DEREF_FN)?;
        }
        writeln!(out, "{}impl {} {{", indent, &self.rust_struct_name)?;
        writeln!(out, "{}    // ...", indent)?;
        writeln!(out, "{}}}", indent)?;
        Ok(())
    }
}



#[derive(Debug, Default)]
pub struct Module {
    // For consistent diffs / printing order, these should *not* be HashMaps
    pub structs: BTreeMap<String, Struct>,
    pub modules: BTreeMap<String, Module>,
}

impl Module {
    pub fn write(&self, context: &Context, indent: &str, out: &mut impl io::Write) -> io::Result<()> {
        let next_indent = format!("{}    ", indent);

        for (name, module) in self.modules.iter() {
            if indent.is_empty() {
                writeln!(out, "#[allow(non_camel_case_types)] // We map Java inner classes to Outer_Inner")?;
                writeln!(out, "#[allow(dead_code)] // We generate structs for private Java types too, just in case.")?;
            }
            writeln!(out, "{}pub mod {} {{", indent, name)?;
            module.write(context, &next_indent[..], out)?;
            writeln!(out, "{}}}", indent)?;
            writeln!(out, "")?;
        }

        for (name, structure) in self.structs.iter() {
            if indent.is_empty() {
                if name.contains("_") { writeln!(out, "#[allow(non_camel_case_types)] // We map Java inner classes to Outer_Inner")?; }
                if !structure.java_class.access_flags().contains(ClassAccessFlags::PUBLIC) { writeln!(out, "#[allow(dead_code)] // We generate structs for private Java types too, just in case.")?; }
            }
            structure.write(context, indent, out)?;
            writeln!(out, "")?;
        }

        Ok(())
    }
}



#[derive(Debug, Default)]
pub struct Context {
    module:                 Module,
    jni_type_to_rust_type:  HashMap<String, String>,
}

impl Context {
    pub fn new() -> Self { Default::default() }

    pub fn add_struct(&mut self, java_class: Class) -> Result<(), Box<dyn Error>> {
        let mut rust_mod : &mut Module = &mut self.module;
        let mut rust_mod_prefix     = String::from("crate::");
        let mut rust_struct_name    = String::new();

        for component in JniPathIter::new(java_class.this_class().name()) {
            match component {
                JniIdentifier::Namespace(id) => {
                    let id = match RustIdentifier::from_str(id) {
                        RustIdentifier::Identifier(id) => id,
                        RustIdentifier::KeywordRawSafe(id) => id,
                        RustIdentifier::KeywordUnderscorePostfix(id) => id,
                        RustIdentifier::NonIdentifier(id) => Err(format!("Unable to add_struct(): parent java namespace name {:?} has no rust equivalent", id))?,
                    };

                    write!(&mut rust_mod_prefix, "{}::", id)?;
                    rust_mod = rust_mod.modules.entry(id.to_owned()).or_insert(Default::default());
                },
                JniIdentifier::ContainingClass(id) => {
                    let id = match RustIdentifier::from_str(id) {
                        RustIdentifier::Identifier(id) => id,
                        RustIdentifier::KeywordRawSafe(id) => id,
                        RustIdentifier::KeywordUnderscorePostfix(id) => id,
                        RustIdentifier::NonIdentifier(id) => Err(format!("Unable to add_struct(): parent java class name {:?} has no rust equivalent", id))?,
                    };

                    write!(&mut rust_struct_name, "{}_", id)?;
                },
                JniIdentifier::LeafClass(id) => {
                    let id = match RustIdentifier::from_str(id) {
                        RustIdentifier::Identifier(id) => id,
                        RustIdentifier::KeywordRawSafe(id) => id,
                        RustIdentifier::KeywordUnderscorePostfix(id) => id,
                        RustIdentifier::NonIdentifier(id) => Err(format!("Unable to add_struct(): java class name {:?} has no rust equivalent", id))?,
                    };

                    write!(&mut rust_struct_name, "{}", id)?;
                    let id = &rust_struct_name[..];

                    if rust_mod.structs.contains_key(id) {
                        Err(format!("Unable to add_struct(): java class name {:?} was already added", id))?
                    }

                    self.jni_type_to_rust_type.insert(java_class.this_class().name().clone(), format!("{}{}", rust_mod_prefix, &rust_struct_name));

                    rust_mod.structs.insert(rust_struct_name.clone(), Struct {
                        rust_mod_prefix,
                        rust_struct_name,
                        java_class,
                    });

                    return Ok(());
                },
            }
        }

        Err(format!("Failed to find LeafClass in {:?}", java_class.this_class().name()))?
    }

    pub fn write(&self, out: &mut impl io::Write) -> io::Result<()> {
        self.module.write(self, "", out)
    }
}
