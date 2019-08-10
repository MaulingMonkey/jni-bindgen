use super::*;
use class_file_visitor::field::*;

use std::io;

pub struct Field {
    access_flags:       FieldAccessFlags,
    java_class:         String,
    java_name:          String,
    java_desc:          String,
    rust_name:          Option<String>,
    rust_const_value:   Option<String>,
    rust_deprecated:    bool,
}

impl Field {
    pub fn new(context: &Context, class: &Class, field: FieldRef) -> Self {
        let mut result = Self {
            access_flags:       field.access_flags(),
            java_class:         class.this_class().name().to_owned(),
            java_name:          field.name().to_owned(),
            java_desc:          field.descriptor().to_owned(),
            rust_name:          None,
            rust_const_value:   None,
            rust_deprecated:    field.field.deprecated,
        };
        if context.config.codegen.field_naming_style.const_finals {
            result.rust_const_value = field.field.rust_const_value.clone();
        }
        result.set_mangling_style(&context.config.codegen.field_naming_style); // rust_name + mangling_style
        result
    }

    pub fn rust_name(&self) -> Option<&str> {
        self.rust_name.as_ref().map(|s| s.as_str())
    }

    pub fn is_public(&self)     -> bool { self.access_flags.contains(FieldAccessFlags::PUBLIC)      }
    pub fn is_private(&self)    -> bool { self.access_flags.contains(FieldAccessFlags::PRIVATE)     }
    pub fn is_protected(&self)  -> bool { self.access_flags.contains(FieldAccessFlags::PROTECTED)   }
    pub fn is_static(&self)     -> bool { self.access_flags.contains(FieldAccessFlags::STATIC)      }
    pub fn is_final(&self)      -> bool { self.access_flags.contains(FieldAccessFlags::FINAL)       }
    pub fn is_volatile(&self)   -> bool { self.access_flags.contains(FieldAccessFlags::VOLATILE)    }
    pub fn is_transient(&self)  -> bool { self.access_flags.contains(FieldAccessFlags::TRANSIENT)   }
    pub fn is_synthetic(&self)  -> bool { self.access_flags.contains(FieldAccessFlags::SYNTHETIC)   }
    pub fn is_enum(&self)       -> bool { self.access_flags.contains(FieldAccessFlags::ENUM)        }

    pub fn access(&self) -> &'static str {
        if      self.is_public()    { "public" }
        else if self.is_protected() { "protected" }
        else if self.is_private()   { "private" }
        else                        { "???" }
    }

    pub fn set_mangling_style(&mut self, style: &FieldManglingStyle) {
        // XXX: Implement
    }

    pub fn emit(&self, context: &Context, indent: &str, out: &mut impl io::Write) -> io::Result<()> {
        let mut emit_reject_reasons = Vec::new();

        if !self.is_public() { emit_reject_reasons.push("Non-public method"); }

        // TODO: context.config.codegen.field_naming_style.rustify_names
        let rust_name = if let Some(rust_name) = self.rust_name.as_ref() {
            rust_name
        } else {
            emit_reject_reasons.push("Failed to mangle field name");
            &self.java_name
        };

        let rust_type = match JniField::from_str(self.java_desc.as_str()) {
            Ok(JniField::Single(JniBasicType::Boolean)) => "bool",
            Ok(JniField::Single(JniBasicType::Byte))    => "i8",
            Ok(JniField::Single(JniBasicType::Char))    => "__jni_bindgen::jchar",
            Ok(JniField::Single(JniBasicType::Double))  => "f64",
            Ok(JniField::Single(JniBasicType::Float))   => "f32",
            Ok(JniField::Single(JniBasicType::Int))     => "i32",
            Ok(JniField::Single(JniBasicType::Long))    => "i64",
            Ok(JniField::Single(JniBasicType::Short))   => "i16",
            Ok(JniField::Single(JniBasicType::Class("java/lang/String"))) if self.rust_const_value.is_some() => "&'static str",
            Ok(JniField::Single(JniBasicType::Void)) => {
                emit_reject_reasons.push("void is not a valid field type");
                "()"
            },
            Ok(JniField::Single(JniBasicType::Class(class))) => {
                emit_reject_reasons.push("Haven't yet implemented object field types");
                class
            },
            Ok(JniField::Array { .. }) => {
                emit_reject_reasons.push("Haven't yet implemented array field types");
                "???"
            },
            Err(_) => {
                emit_reject_reasons.push("Failed to parse field type");
                "???"
            },
        };

        let rust_getter_name = context.config.codegen.field_naming_style.getter_pattern.replace("{NAME}", rust_name);
        let rust_setter_name = if self.rust_const_value.is_some() || self.is_final() {
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
            self.access(),
            if self.is_static()     { " static"     } else { "" },
            if self.is_final()      { " final"      } else { "" },
            if self.is_volatile()   { " volatile"   } else { "" }
        );

        let attributes = format!("{}",
            if self.rust_deprecated { "#[deprecated] " } else { "" },
        );

        writeln!(out, "")?;
        for reason in &emit_reject_reasons {
            writeln!(out, "{}// Not emitting: {}", indent, reason)?;
        }
        writeln!(out, "{}/// {} {}", indent, &keywords, &self.java_name)?; // TODO: Field doc links

        if let Some(rust_const_value) = self.rust_const_value.as_ref() {
            match rust_type {
                "__jni_bindgen::jchar"  => writeln!(out, "{}{}pub const {} : {} = {}({});", indent, &attributes, rust_name, rust_type, rust_type, rust_const_value)?,
                "bool"                  => writeln!(out, "{}{}pub const {} : {} = {};", indent, &attributes, rust_name, rust_type, if rust_const_value == "0" { "false" } else { "true" })?,
                _                       => writeln!(out, "{}{}pub const {} : {} = {};", indent, &attributes, rust_name, rust_type, rust_const_value)?,
            }
        } else if self.is_static() {
            writeln!(out, "{}{}pub fn {}<'env>(env: &'env Env) -> {} {{ ... }}", indent, &attributes, rust_getter_name, rust_type)?;
        } else {
            writeln!(out, "{}{}pub fn {}<'env>(&'env self) -> {} {{ ... }}", indent, &attributes, rust_getter_name, rust_type)?;
        }

        if let Some(rust_setter_name) = rust_setter_name {
            writeln!(out, "{}/// {} {}", indent, &keywords, &self.java_name)?; // TODO: Field doc links
            if self.is_static() {
                writeln!(out, "{}{}pub fn {}<'env>(env: &'env Env) -> {} {{ ... }}", indent, &attributes, rust_setter_name, rust_type)?;
            } else {
                writeln!(out, "{}{}pub fn {}<'env>(&'env self) -> {} {{ ... }}", indent, &attributes, rust_setter_name, rust_type)?;
            }
        }

        Ok(())
    }
}
