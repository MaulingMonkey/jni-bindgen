use super::*;

use std::collections::*;
use std::io;

#[derive(Debug, Default)]
pub(crate) struct Struct {
    pub rust_mod_prefix:    String,
    pub rust_struct_name:   String,
    pub java_class:         Class,
}

impl Struct {
    pub(crate) fn write(&self, context: &Context, indent: &str, out: &mut impl io::Write) -> io::Result<()> {
        writeln!(out, "")?;
        self.write_rust_struct(context, indent, out)?;
        Ok(())
    }

    fn write_rust_struct(&self, context: &Context, indent: &str, out: &mut impl io::Write) -> io::Result<()> {
        // Ignored access_flags: SUPER, SYNTHETIC, ANNOTATION, ABSTRACT

        let keyword = if self.java_class.access_flags().contains(ClassAccessFlags::INTERFACE) {
            "interface"
        } else if self.java_class.access_flags().contains(ClassAccessFlags::ENUM) {
            "enum"
        } else if self.java_class.access_flags().contains(ClassAccessFlags::STATIC) {
            "static class"
        } else if self.java_class.access_flags().contains(ClassAccessFlags::FINAL) {
            "final class"
        } else {
            "class"
        };

        let visibility = if self.java_class.access_flags().contains(ClassAccessFlags::PUBLIC) {
            "public"
        } else {
            "private"
        };

        let super_class = if let Some(super_class) = self.java_class.super_class() {
            context.java_to_rust_path(super_class.name()).unwrap()
        } else {
            "()".to_owned() // This might only happen for java.lang.Object
        };

        writeln!(out, "{}__jni_bindgen! {{", indent)?;
        if let Some(url) = KnownDocsUrl::from_class(context, &self.java_class.this_class().name()) {
            writeln!(out, "{}    /// {} {} [{}]({})", indent, visibility, keyword, url.label, url.url)?;
        }
        write!(out, "{}    {} {} {} extends {}", indent, visibility, keyword, &self.rust_struct_name, super_class)?;
        let mut implements = false;
        for interface in self.java_class.interfaces() {
            write!(out, ", ")?;
            if !implements {
                write!(out, "implements ")?;
                implements = true;
            }
            write!(out, "{}", &context.java_to_rust_path(interface.name()).unwrap())?;
        }
        writeln!(out, " {{")?;

        let mut id_repeats = HashMap::new();

        let mut methods : Vec<Method> = self.java_class.methods().map(|m| Method::new(context, &self.java_class, m)).collect();
        let mut fields  : Vec<Field > = self.java_class.fields().map(|f| Field::new(context, &self.java_class, f)).collect();

        for method in &methods {
            if !method.is_public() { continue; } // Skip private/protected methods
            if let Some(name) = method.rust_name() {
                *id_repeats.entry(name.to_owned()).or_insert(0) += 1;
            }
        }

        for field in &fields {
            if !field.is_public() { continue; } // Skip private/protected fields
            if let Some(name) = field.rust_name() {
                *id_repeats.entry(name.to_owned()).or_insert(0) += 1;
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
            //if let Some(name) = field.rust_name() {
            //    let repeats = *id_repeats.get(name).unwrap_or(&0);
            //    let overloaded = repeats > 1;
            //    if overloaded {
            //        field.set_mangling_style(context.config.codegen.field_naming_style_collision);
            //    }
            //}

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
