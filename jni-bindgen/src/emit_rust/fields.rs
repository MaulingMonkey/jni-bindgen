use crate::emit_rust::*;
use crate::identifiers::*;

use jreflection::class;
use jreflection::field;

use std::io;

pub struct Field<'a> {
    pub class:      &'a jreflection::Class,
    pub java:       &'a jreflection::Field,
    pub rust_names: Result<FieldMangling<'a>, IdentifierManglingError>,
    pub ignored:    bool,
}

impl<'a> Field<'a> {
    pub fn new(context: &Context, class: &'a jreflection::Class, java: &'a jreflection::Field) -> Self {
        let java_class_field = format!("{}\x1f{}", class.path.as_str(), &java.name);
        let ignored = context.config.ignore_class_fields.contains(&java_class_field);
        let renamed_to = context.config.rename_class_fields.get(&java_class_field).map(|s| s.as_str());

        let result = Self {
            class,
            java,
            rust_names: context.config.codegen.field_naming_style.mangle(java, renamed_to),
            ignored,
        };
        result
    }

    pub fn emit(&self, context: &Context, indent: &str, out: &mut impl io::Write) -> io::Result<()> {
        let mut emit_reject_reasons = Vec::new();

        if !self.java.is_public()   { emit_reject_reasons.push("Non-public field"); }
        if self.ignored             { emit_reject_reasons.push("[[ignore]]d"); }

        let mut required_feature = None;

        let descriptor = self.java.descriptor();
        let rust_set_type_buffer;
        let rust_get_type_buffer;
        let (rust_set_type, rust_get_type) = match descriptor {
            field::Descriptor::Single(field::BasicType::Boolean) => ("bool", "bool"),
            field::Descriptor::Single(field::BasicType::Byte)    => ("i8", "i8"),
            field::Descriptor::Single(field::BasicType::Char)    => ("__jni_bindgen::jchar", "__jni_bindgen::jchar"),
            field::Descriptor::Single(field::BasicType::Double)  => ("f64", "f64"),
            field::Descriptor::Single(field::BasicType::Float)   => ("f32", "f32"),
            field::Descriptor::Single(field::BasicType::Int)     => ("i32", "i32"),
            field::Descriptor::Single(field::BasicType::Long)    => ("i64", "i64"),
            field::Descriptor::Single(field::BasicType::Short)   => ("i16", "i16"),
            field::Descriptor::Single(field::BasicType::Class(class::Id("java/lang/String"))) if self.java.is_constant() => ("&'static str", "&'static str"),
            field::Descriptor::Single(field::BasicType::Void) => {
                emit_reject_reasons.push("ERROR:  void is not a valid field type");
                ("()", "()")
            },
            field::Descriptor::Single(field::BasicType::Class(class)) => {
                if let Ok(feature) = Struct::feature_for(context, class) {
                    required_feature = Some(feature);
                } else {
                    emit_reject_reasons.push("ERROR:  Unable to resolve class feature");
                }

                if let Ok(fqn) = Struct::fqn_for(context, class) {
                    rust_set_type_buffer = format!("impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'obj {}>>", &fqn);
                    rust_get_type_buffer = format!("__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, {}>>", &fqn);
                    (rust_set_type_buffer.as_str(), rust_get_type_buffer.as_str())
                } else {
                    emit_reject_reasons.push("ERROR:  Unable to resolve class FQN");
                    ("???", "???")
                }
            },
            field::Descriptor::Array { levels, inner } => {
                let mut buffer = String::new();
                for _ in 0..(levels-1) {
                    buffer.push_str("__jni_bindgen::ObjectArray<");
                }
                match inner {
                    field::BasicType::Boolean      => buffer.push_str("__jni_bindgen::BooleanArray"),
                    field::BasicType::Byte         => buffer.push_str("__jni_bindgen::ByteArray"),
                    field::BasicType::Char         => buffer.push_str("__jni_bindgen::CharArray"),
                    field::BasicType::Short        => buffer.push_str("__jni_bindgen::ShortArray"),
                    field::BasicType::Int          => buffer.push_str("__jni_bindgen::IntArray"),
                    field::BasicType::Long         => buffer.push_str("__jni_bindgen::LongArray"),
                    field::BasicType::Float        => buffer.push_str("__jni_bindgen::FloatArray"),
                    field::BasicType::Double       => buffer.push_str("__jni_bindgen::DoubleArray"),
                    field::BasicType::Class(class) => {
                        if let Ok(feature) = Struct::feature_for(context, class) {
                            required_feature = Some(feature);
                        } else {
                            emit_reject_reasons.push("ERROR:  Unable to resolve class feature");
                        }
                        buffer.push_str("__jni_bindgen::ObjectArray<");
                        match context.java_to_rust_path(class) {
                            Ok(path) => buffer.push_str(path.as_str()),
                            Err(_) => {
                                emit_reject_reasons.push("ERROR:  Failed to resolve JNI path to Rust path for argument type");
                                buffer.push_str("???");
                            }
                        }
                        buffer.push_str(", ");
                        buffer.push_str(context.config.codegen.throwable_type.as_str());
                        buffer.push_str(">");
                    },
                    field::BasicType::Void => {
                        emit_reject_reasons.push("ERROR:  Arrays of void isn't a thing");
                        buffer.push_str("[()]");
                    },
                }
                for _ in 0..(levels-1) { // ObjectArray s
                    buffer.push_str(", ");
                    buffer.push_str(context.config.codegen.throwable_type.as_str());
                    buffer.push_str(">");
                }

                rust_set_type_buffer = format!("impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'obj {}>>", &buffer);
                rust_get_type_buffer = format!("__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, {}>>", &buffer);
                (rust_set_type_buffer.as_str(), rust_get_type_buffer.as_str())
            },
        };

        let field_fragment = match self.java.descriptor() { // Contents of {get,set}_[static_]..._field
            field::Descriptor::Single(field::BasicType::Void)        => "void",
            field::Descriptor::Single(field::BasicType::Boolean)     => "boolean",
            field::Descriptor::Single(field::BasicType::Byte)        => "byte",
            field::Descriptor::Single(field::BasicType::Char)        => "char",
            field::Descriptor::Single(field::BasicType::Short)       => "short",
            field::Descriptor::Single(field::BasicType::Int)         => "int",
            field::Descriptor::Single(field::BasicType::Long)        => "long",
            field::Descriptor::Single(field::BasicType::Float)       => "float",
            field::Descriptor::Single(field::BasicType::Double)      => "double",
            field::Descriptor::Single(field::BasicType::Class(_))    => "object",
            field::Descriptor::Array { .. }                          => "object",
        };

        if self.rust_names.is_err() {
            emit_reject_reasons.push(match self.java.name.as_str() {
                "$VALUES"                   => "Failed to mangle field name: enum $VALUES", // Expected
                s if s.starts_with("this$") => "Failed to mangle field name: this$N outer class pointer", // Expected
                _                           => "ERROR:  Failed to mangle field name(s)",
            });
        }

        let emit_reject_reasons = emit_reject_reasons; // Freeze
        let indent = if emit_reject_reasons.is_empty() {
            format!("{}        ", indent)
        } else {
            if !context.config.codegen.keep_rejected_emits {
                return Ok(());
            }
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

        let env_param = if self.java.is_static() { "env: &'env __jni_bindgen::Env" } else { "&'env self" };

        let url = KnownDocsUrl::from_field(context, self.class.path.as_str(), self.java.name.as_str(), self.java.descriptor());
        let url = url.as_ref();

        match self.rust_names.as_ref() {
            Ok(FieldMangling::ConstValue(constant, value)) => {
                let value = *value;
                if let Some(url) = url {
                    writeln!(out, "{}/// {} {}", indent, &keywords, url)?;
                }
                if let Some(required_feature) = required_feature.as_ref() {
                    writeln!(out, "{}///", indent)?;
                    writeln!(out, "{}/// Required feature: {:?}", indent, required_feature)?;
                    writeln!(out, "{}#[cfg(any(feature = \"all\", feature = {:?}))]", indent, required_feature)?;
                }
                match descriptor {
                    field::Descriptor::Single(field::BasicType::Char)       => writeln!(out, "{}{}pub const {} : {} = {}({});", indent, &attributes, constant, rust_get_type, rust_get_type, value)?,
                    field::Descriptor::Single(field::BasicType::Boolean)    => writeln!(out, "{}{}pub const {} : {} = {};", indent, &attributes, constant, rust_get_type, if value == &field::Constant::Integer(0) { "false" } else { "true" })?,
                    _                                                       => writeln!(out, "{}{}pub const {} : {} = {};", indent, &attributes, constant, rust_get_type, value)?,
                }
            },
            Ok(FieldMangling::GetSet(get, set)) => {
                // Getter
                if let Some(url) = url {
                    writeln!(out, "{}/// **get** {} {}", indent, &keywords, url)?;
                } else {
                    writeln!(out, "{}/// **get** {} {}", indent, &keywords, self.java.name.as_str())?;
                }
                if let Some(required_feature) = required_feature.as_ref() {
                    writeln!(out, "{}///", indent)?;
                    writeln!(out, "{}/// Required feature: {:?}", indent, required_feature)?;
                    writeln!(out, "{}#[cfg(any(feature = \"all\", feature = {:?}))]", indent, required_feature)?;
                }
                writeln!(out, "{}{}pub fn {}<'env>({}) -> {} {{", indent, &attributes, get, env_param, rust_get_type)?;
                writeln!(out, "{}    unsafe {{", indent)?;
                if !self.java.is_static() {
                    writeln!(out, "{}        let env = __jni_bindgen::Env::from_ptr(self.0.env);", indent)?;
                }
                writeln!(out, "{}        let (__jni_class, __jni_field) = env.require_class_{}field({}, {}, {});", indent, if self.java.is_static() { "static_" } else { "" }, emit_cstr(self.class.path.as_str()), emit_cstr(self.java.name.as_str()), emit_cstr(self.java.descriptor_str()) )?;
                if self.java.is_static() {
                    writeln!(out, "{}        env.get_static_{}_field(__jni_class, __jni_field)", indent, field_fragment)?;
                } else {
                    writeln!(out, "{}        env.get_{}_field(self.0.object, __jni_field)", indent, field_fragment)?;
                }
                writeln!(out, "{}    }}", indent)?;
                writeln!(out, "{}}}", indent)?;

                // Setter
                if !self.java.is_final() {
                    let lifetimes = if field_fragment == "object" { "'env, 'obj" } else { "'env" };

                    writeln!(out, "")?;
                    if let Some(url) = url {
                        writeln!(out, "{}/// **set** {} {}", indent, &keywords, url)?;
                    } else {
                        writeln!(out, "{}/// **set** {} {}", indent, &keywords, self.java.name.as_str())?;
                    }
                    if let Some(required_feature) = required_feature.as_ref() {
                        writeln!(out, "{}///", indent)?;
                        writeln!(out, "{}/// Required feature: {:?}", indent, required_feature)?;
                        writeln!(out, "{}#[cfg(any(feature = \"all\", feature = {:?}))]", indent, required_feature)?;
                    }
                    writeln!(out, "{}{}pub fn {}<{}>({}, value: {}) {{", indent, &attributes, set, lifetimes, env_param, rust_set_type)?;
                    writeln!(out, "{}    unsafe {{", indent)?;
                    if !self.java.is_static() {
                        writeln!(out, "{}        let env = __jni_bindgen::Env::from_ptr(self.0.env);", indent)?;
                    }
                    writeln!(out, "{}        let (__jni_class, __jni_field) = env.require_class_{}field({}, {}, {});", indent, if self.java.is_static() { "static_" } else { "" }, emit_cstr(self.class.path.as_str()), emit_cstr(self.java.name.as_str()), emit_cstr(self.java.descriptor_str()) )?;
                    if self.java.is_static() {
                        writeln!(out, "{}        env.set_static_{}_field(__jni_class, __jni_field, value)", indent, field_fragment)?;
                    } else {
                        writeln!(out, "{}        env.set_{}_field(self.0.object, __jni_field, value)", indent, field_fragment)?;
                    }
                    writeln!(out, "{}    }}", indent)?;
                    writeln!(out, "{}}}", indent)?;
                }
            },
            Err(_) => {
                writeln!(out, "{}{}pub fn get_{:?}<'env>({}) -> {} {{ ... }}", indent, &attributes, self.java.name.as_str(), env_param, rust_get_type)?;
                if !self.java.is_final() {
                    writeln!(out, "{}{}pub fn set_{:?}<'env>({}) -> {} {{ ... }}", indent, &attributes, self.java.name.as_str(), env_param, rust_set_type)?;
                }
            },
        }

        Ok(())
    }
}

fn emit_cstr(s: &str) -> String {
    let mut s = format!("{:?}", s); // XXX
    s.insert_str(s.len() - 1, "\\0");
    s
}
