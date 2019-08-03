use super::*;
use class_file_visitor::method::*;

use std::collections::*;
use std::error::Error;
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
            context.java_to_rust_path(super_class).unwrap()
        } else {
            "()".to_owned() // This might only happen for java.lang.Object
        };

        // TODO:  Eventually move macro codegen into the mod so multiple classes can be collapsed.
        writeln!(out, "{}__bindgen_jni! {{", indent)?;
        if let Some(url) = KnownDocsUrl::from(&self.java_class) {
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
            write!(out, "{}", &context.java_to_rust_path(interface).unwrap())?;
        }
        writeln!(out, " {{")?;

        let mut id_repeats = HashMap::new();

        for method in self.java_class.methods() {
            if !method.access_flags().contains(MethodAccessFlags::PUBLIC) { continue; } // Skip private/protected methods
            let method_name = if let Ok(name) = mangle_method_name(method.name()) { name } else { continue };
            *id_repeats.entry(method_name).or_insert(0) += 1;
        }

        // TODO: fields

        for method in self.java_class.methods() {
            let mut actually_emit = true;

            let constructor = method.name() == "<init>";
            let public      = method.access_flags().contains(MethodAccessFlags::PUBLIC);
            let protected   = method.access_flags().contains(MethodAccessFlags::PROTECTED);
            let static_     = method.access_flags().contains(MethodAccessFlags::STATIC);
            let _private    = !public && !protected;
            let _access     = if public { "public" } else if protected { "protected" } else { "private" };

            let descriptor = method.descriptor();

            if !public { actually_emit = false; } // Skip private/protected methods

            let method_name = if let Ok(name) = mangle_method_name(method.name()) {
                name
            } else {
                writeln!(out, "{}        // {:?} fn ??? (could not mangle {:?}) = {:?}", indent, method.access_flags(), method.name(), method.descriptor())?;
                continue;
            };

            let repeats = *id_repeats.get(&method_name).unwrap_or(&0);

            let method_name = if repeats <= 1 {
                method_name
            } else {
                actually_emit = false; // NYI: Overload mangling
                format!("{}_OVERLOADED", method_name)
            };

            if actually_emit {
                let access = if public { "pub " } else { "" };
                let self_param = if static_ || constructor { "" } else if descriptor.starts_with("()") { "&self" } else { "&self, " };
                write!(out, "{}        // {}fn {}({}", indent, access, method_name, self_param)?;
                // TODO: Params
                if !descriptor.starts_with("()") { write!(out, "???")?; }
                // TODO: Return type
                writeln!(out, ") -> ! {{ unimplemented!(); }}")?;
            } else {
                writeln!(out, "{}        // {:?} fn {} = {:?}", indent, method.access_flags(), method_name, method.descriptor())?;
            }
        }

        // TODO: fields

        writeln!(out, "{}    }}", indent)?;
        writeln!(out, "{}}}", indent)?;
        Ok(())
    }
}

fn mangle_method_name(name: &String) -> Result<String, Box<dyn Error>> {
    if name == "<init>" { // Java Constructor
        Ok("new".to_owned()) // Traditional rust method
    } else if name == "<clinit>" {
        return Err("Java static constructors are not mapped to rust names")?;
    } else if name == "" {
        return Err("Unexpected empty string for method name")?;
    } else {
        let mut chars = name.chars();
        let mut buffer = String::new();

        // First character
        if let Some(ch) = chars.next() {
            match ch {
                'a'..='z'   => buffer.push(ch),
                'A'..='Z'   => buffer.push(ch.to_ascii_lowercase()),
                '_'         => buffer.push(ch),
                _           => Err(format!("Unexpected first character in method name: {}", ch))?,
            }
        }

        // Subsequent characters
        for ch in chars {
            if ch.is_ascii_uppercase() {
                buffer.push('_');
                buffer.push(ch.to_ascii_lowercase());
            } else if ch.is_ascii_alphanumeric() {
                buffer.push(ch);
            } else if ch == '_' {
                buffer.push(ch);
            } else {
                return Err(format!("Unexpected character in method name: {}", ch))?;
            }
        }

        if buffer == "_" {
            Ok(String::from("__"))
        } else {
            Ok(buffer)
        }
    }
}
