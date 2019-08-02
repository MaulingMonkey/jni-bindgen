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

const DEREF_FN : &'static str = "fn deref(&self) -> &Self::Target { unsafe { &*(self as *const Self as *const Self::Target) } }";
