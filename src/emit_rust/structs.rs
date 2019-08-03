use super::*;
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
        // Ignored access_flags: SUPER, SYNTHETIC, ANNOTATION, ABSTRACT, FINAL

        let keyword = if self.java_class.access_flags().contains(ClassAccessFlags::INTERFACE) {
            "interface"
        } else if self.java_class.access_flags().contains(ClassAccessFlags::ENUM) {
            "enum"
        } else {
            "class"
        };

        let visibility = if self.java_class.access_flags().contains(ClassAccessFlags::PUBLIC) {
            "public"
        } else {
            "private"
        };

        let _struct_body = if self.java_class.access_flags().contains(ClassAccessFlags::STATIC) {
            ""
        } else {
            "(__bindgen_jni_jni_sys::jobject)"
        };

        let _super_class = if let Some(super_class) = self.java_class.super_class() {
            &context.jni_type_to_rust_type.get(super_class.name()).unwrap()[..]
        } else {
            "()"
        };

        writeln!(out, "{}__bindgen_jni! {{", indent)?;
        if let Some(url) = KnownDocsUrl::from(&self.java_class) {
            writeln!(out, "{}    /// [{}]({})", indent, url.label, url.url)?;
        }
        write!(out, "{}    {} {} {} extends {}", indent, visibility, keyword, &self.rust_struct_name, _super_class)?;
        let mut implements = false;
        for interface in self.java_class.interfaces() {
            write!(out, ", ")?;
            if !implements {
                write!(out, "implements ")?;
                implements = true;
            }
            write!(out, "{}", &context.jni_type_to_rust_type.get(interface.name()).unwrap())?;
        }
        writeln!(out, " {{")?;
        writeln!(out, "{}    }}", indent)?;
        writeln!(out, "{}}}", indent)?;
        Ok(())
    }
}

const DEREF_FN : &'static str = "fn deref(&self) -> &Self::Target { unsafe { &*(self as *const Self as *const Self::Target) } }";
