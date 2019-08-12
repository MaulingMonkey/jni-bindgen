use super::*;

use java::class;
use java::field;

use std::io;

pub struct Field<'a> {
    pub class:          &'a java::Class,
    pub java:           &'a java::Field,
    rust_name:          Option<String>,
    rust_const_value:   Option<String>,
}

impl<'a> Field<'a> {
    pub fn new(context: &Context, class: &'a java::Class, java: &'a java::Field) -> Self {
        let mut result = Self {
            class,
            java,
            rust_name:          None,
            rust_const_value:   None,
        };
        if context.config.codegen.field_naming_style.const_finals {
            result.rust_const_value = java.constant.as_ref().map(|c| match c {
                field::Constant::Double (value) => format!("{}", value),
                field::Constant::Float  (value) => format!("{}", value),
                field::Constant::Integer(value) => format!("{}", value),
                field::Constant::Long   (value) => format!("{}", value),
                field::Constant::String (value) => format!("{:?}", value),
            });
        }
        result.set_mangling_style(&context.config.codegen.field_naming_style); // rust_name + mangling_style
        result
    }

    pub fn rust_name(&self) -> Option<&str> {
        self.rust_name.as_ref().map(|s| s.as_str())
    }

    pub fn set_mangling_style(&mut self, style: &FieldManglingStyle) {
        // XXX: Implement
    }

    pub fn emit(&self, context: &Context, indent: &str, out: &mut impl io::Write) -> io::Result<()> {
        let mut emit_reject_reasons = Vec::new();

        if !self.java.is_public() { emit_reject_reasons.push("Non-public method"); }

        // TODO: context.config.codegen.field_naming_style.rustify_names
        let rust_name = if let Some(rust_name) = self.rust_name.as_ref() {
            rust_name
        } else {
            emit_reject_reasons.push("Failed to mangle field name");
            &self.java.name
        };

        let descriptor = self.java.descriptor();
        let rust_type = match descriptor {
            field::Descriptor::Single(field::BasicType::Boolean) => "bool",
            field::Descriptor::Single(field::BasicType::Byte)    => "i8",
            field::Descriptor::Single(field::BasicType::Char)    => "__jni_bindgen::jchar",
            field::Descriptor::Single(field::BasicType::Double)  => "f64",
            field::Descriptor::Single(field::BasicType::Float)   => "f32",
            field::Descriptor::Single(field::BasicType::Int)     => "i32",
            field::Descriptor::Single(field::BasicType::Long)    => "i64",
            field::Descriptor::Single(field::BasicType::Short)   => "i16",
            field::Descriptor::Single(field::BasicType::Class(class::Id("java/lang/String"))) if self.rust_const_value.is_some() => "&'static str",
            field::Descriptor::Single(field::BasicType::Void) => {
                emit_reject_reasons.push("void is not a valid field type");
                "()"
            },
            field::Descriptor::Single(field::BasicType::Class(class)) => {
                emit_reject_reasons.push("Haven't yet implemented object field types");
                class.as_str()
            },
            field::Descriptor::Array { .. } => {
                emit_reject_reasons.push("Haven't yet implemented array field types");
                "???"
            },
        };

        let rust_getter_name = context.config.codegen.field_naming_style.getter_pattern.replace("{NAME}", rust_name);
        let rust_setter_name = if self.rust_const_value.is_some() || self.java.is_final() {
            None
        } else {
            Some(context.config.codegen.field_naming_style.setter_pattern.replace("{NAME}", rust_name))
        };

        let emit_reject_reasons = emit_reject_reasons; // Freeze
        let indent = if emit_reject_reasons.is_empty() {
            format!("{}        ", indent)
        } else {
            format!("{}        // ", indent)
        };

        let keywords = format!("{}{}{}{}",
            self.java.access().unwrap_or("???"),
            if self.java.is_static()     { " static"     } else { "" },
            if self.java.is_final()      { " final"      } else { "" },
            if self.java.is_volatile()   { " volatile"   } else { "" }
        );

        let attributes = format!("{}",
            if self.java.deprecated { "#[deprecated] " } else { "" },
        );

        writeln!(out, "")?;
        for reason in &emit_reject_reasons {
            writeln!(out, "{}// Not emitting: {}", indent, reason)?;
        }
        writeln!(out, "{}/// {} {}", indent, &keywords, &self.java.name)?; // TODO: Field doc links

        if let Some(rust_const_value) = self.rust_const_value.as_ref() {
            match descriptor {
                field::Descriptor::Single(field::BasicType::Char)       => writeln!(out, "{}{}pub const {} : {} = {}({});", indent, &attributes, rust_name, rust_type, rust_type, rust_const_value)?,
                field::Descriptor::Single(field::BasicType::Boolean)    => writeln!(out, "{}{}pub const {} : {} = {};", indent, &attributes, rust_name, rust_type, if rust_const_value == "0" { "false" } else { "true" })?,
                _                                                       => writeln!(out, "{}{}pub const {} : {} = {};", indent, &attributes, rust_name, rust_type, rust_const_value)?,
            }
        } else if self.java.is_static() {
            writeln!(out, "{}{}pub fn {}<'env>(env: &'env Env) -> {} {{ ... }}", indent, &attributes, rust_getter_name, rust_type)?;
        } else {
            writeln!(out, "{}{}pub fn {}<'env>(&'env self) -> {} {{ ... }}", indent, &attributes, rust_getter_name, rust_type)?;
        }

        if let Some(rust_setter_name) = rust_setter_name {
            writeln!(out, "{}/// {} {}", indent, &keywords, &self.java.name)?; // TODO: Field doc links
            if self.java.is_static() {
                writeln!(out, "{}{}pub fn {}<'env>(env: &'env Env) -> {} {{ ... }}", indent, &attributes, rust_setter_name, rust_type)?;
            } else {
                writeln!(out, "{}{}pub fn {}<'env>(&'env self) -> {} {{ ... }}", indent, &attributes, rust_setter_name, rust_type)?;
            }
        }

        Ok(())
    }
}
