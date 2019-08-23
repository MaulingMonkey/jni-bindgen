use super::*;

use java::method;

use std::collections::BTreeSet;
use std::io;



pub struct Method<'a> {
    pub class:      &'a java::Class,
    pub java:       &'a java::Method,
    rust_name:      Option<String>,
    mangling_style: MethodManglingStyle,
}

impl<'a> Method<'a> {
    pub fn new(context: &Context, class: &'a java::Class, java: &'a java::Method) -> Self {
        let mut result = Self {
            class,
            java,
            rust_name:      None,
            mangling_style: MethodManglingStyle::Java, // Immediately overwritten bellow
        };
        result.set_mangling_style(context.config.codegen.method_naming_style); // rust_name + mangling_style
        result
    }

    pub fn rust_name(&self) -> Option<&str> {
        self.rust_name.as_ref().map(|s| s.as_str())
    }

    pub fn set_mangling_style(&mut self, style: MethodManglingStyle) {
        self.mangling_style = style;
        self.rust_name = if let Ok(name) = self.mangling_style.mangle(self.java.name.as_str(), self.java.descriptor()) {
            Some(name)
        } else {
            None // Failed to mangle
        };
    }

    pub fn emit(&self, context: &Context, indent: &str, out: &mut impl io::Write) -> io::Result<()> {
        let mut emit_reject_reasons = Vec::new();
        let mut required_features = BTreeSet::new();

        let java_class_method       = format!("{}\x1f{}", self.class.path.as_str(), &self.java.name);
        let java_class_method_sig   = format!("{}\x1f{}\x1f{}", self.class.path.as_str(), &self.java.name, self.java.descriptor_str());

        let ignored =
            context.config.ignore_class_methods    .contains(&java_class_method) ||
            context.config.ignore_class_method_sigs.contains(&java_class_method_sig);

        let renamed_to = context.config.rename_class_methods    .get(&java_class_method)
            .or_else(||  context.config.rename_class_method_sigs.get(&java_class_method_sig));

        let descriptor = self.java.descriptor();

        let method_name = if let Some(renamed_to) = renamed_to {
            renamed_to.clone()
        } else if let Some(name) = self.rust_name() {
            name.to_owned()
        } else {
            emit_reject_reasons.push("ERROR:  Failed to mangle method name");
            self.java.name.to_owned()
        };

        if !self.java.is_public()       { emit_reject_reasons.push("Non-public method"); }
        if self.java.is_bridge()        { emit_reject_reasons.push("Bridge method - type erasure"); }
        if self.java.is_static_init()   { emit_reject_reasons.push("Static class constructor - never needs to be called by Rust."); return Ok(()); }
        if ignored                      { emit_reject_reasons.push("[[ignore]]d"); }

        // Parameter names may or may not be available as extra debug information.  Example:
        // https://docs.oracle.com/javase/tutorial/reflect/member/methodparameterreflection.html

        let mut params_array = String::new(); // Contents of let __jni_args = [...];

        // Contents of fn name<'env>(...) {
        let mut params_decl = if self.java.is_constructor() || self.java.is_static() {
            match context.config.codegen.static_env {
                config::toml::StaticEnvStyle::Explicit => String::from("__jni_env: &'env __jni_bindgen::Env"),
                config::toml::StaticEnvStyle::__NonExhaustive => {
                    emit_reject_reasons.push("ERROR:  StaticEnvStyle::__NonExhaustive is invalid, silly goose!");
                    String::new()
                },
            }
        } else {
            String::from("&'env self")
        };

        for (arg_idx, arg) in descriptor.arguments().enumerate() {
            let arg_name = format!("arg{}", arg_idx);

            let mut param_is_object = false; // XXX

            let arg_type = match arg {
                method::Type::Single(method::BasicType::Void) => {
                    emit_reject_reasons.push("ERROR:  Void arguments aren't a thing");
                    "()".to_owned()
                },
                method::Type::Single(method::BasicType::Boolean)     => "bool".to_owned(),
                method::Type::Single(method::BasicType::Byte)        => "i8".to_owned(),
                method::Type::Single(method::BasicType::Char)        => "__jni_bindgen::jchar".to_owned(),
                method::Type::Single(method::BasicType::Short)       => "i16".to_owned(),
                method::Type::Single(method::BasicType::Int)         => "i32".to_owned(),
                method::Type::Single(method::BasicType::Long)        => "i64".to_owned(),
                method::Type::Single(method::BasicType::Float)       => "f32".to_owned(),
                method::Type::Single(method::BasicType::Double)      => "f64".to_owned(),
                method::Type::Single(method::BasicType::Class(class)) => {
                    if let Ok(feature) = Struct::feature_for(context, class) {
                        required_features.insert(feature);
                    } else {
                        emit_reject_reasons.push("ERROR:  Unable to resolve class feature");
                    }
                    param_is_object = true;
                    match context.java_to_rust_path(class) {
                        Ok(path) => format!("impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env {}>>", path),
                        Err(_) => {
                            emit_reject_reasons.push("ERROR:  Failed to resolve JNI path to Rust path for argument type");
                            format!("{:?}", class)
                        }
                    }
                },
                method::Type::Array { levels, inner } => {
                    let mut buffer = "impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env ".to_owned();
                    for _ in 0..(levels-1) {
                        buffer.push_str("__jni_bindgen::ObjectArray<");
                    }
                    match inner {
                        method::BasicType::Boolean      => buffer.push_str("__jni_bindgen::BooleanArray"),
                        method::BasicType::Byte         => buffer.push_str("__jni_bindgen::ByteArray"),
                        method::BasicType::Char         => buffer.push_str("__jni_bindgen::CharArray"),
                        method::BasicType::Short        => buffer.push_str("__jni_bindgen::ShortArray"),
                        method::BasicType::Int          => buffer.push_str("__jni_bindgen::IntArray"),
                        method::BasicType::Long         => buffer.push_str("__jni_bindgen::LongArray"),
                        method::BasicType::Float        => buffer.push_str("__jni_bindgen::FloatArray"),
                        method::BasicType::Double       => buffer.push_str("__jni_bindgen::DoubleArray"),
                        method::BasicType::Class(class) => {
                            if let Ok(feature) = Struct::feature_for(context, class) {
                                required_features.insert(feature);
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
                            buffer.push_str(">");
                        },
                        method::BasicType::Void => {
                            emit_reject_reasons.push("ERROR:  Arrays of void isn't a thing");
                            buffer.push_str("[()]");
                        },
                    }
                    for _ in 0..(levels-1) {
                        buffer.push_str(">"); // ObjectArray s
                    }
                    buffer.push_str(">>"); // Option, Into

                    param_is_object = true;
                    buffer
                },
            };

            if !params_array.is_empty() {
                params_array.push_str(", ");
            }

            params_array.push_str("__jni_bindgen::AsJValue::as_jvalue(");
            params_array.push_str("&");
            params_array.push_str(arg_name.as_str());
            if param_is_object { params_array.push_str(".into()"); }
            params_array.push_str(")");

            if !params_decl.is_empty() {
                params_decl.push_str(", ");
            }

            params_decl.push_str(arg_name.as_str());
            params_decl.push_str(": ");
            params_decl.push_str(arg_type.as_str());
        }

        let mut ret_decl = match descriptor.return_type() { // Contents of fn name<'env>() -> Result<...> {
            method::Type::Single(method::BasicType::Void)        => "()".to_owned(),
            method::Type::Single(method::BasicType::Boolean)     => "bool".to_owned(),
            method::Type::Single(method::BasicType::Byte)        => "i8".to_owned(),
            method::Type::Single(method::BasicType::Char)        => "__jni_bindgen::jchar".to_owned(),
            method::Type::Single(method::BasicType::Short)       => "i16".to_owned(),
            method::Type::Single(method::BasicType::Int)         => "i32".to_owned(),
            method::Type::Single(method::BasicType::Long)        => "i64".to_owned(),
            method::Type::Single(method::BasicType::Float)       => "f32".to_owned(),
            method::Type::Single(method::BasicType::Double)      => "f64".to_owned(),
            method::Type::Single(method::BasicType::Class(class)) => {
                if let Ok(feature) = Struct::feature_for(context, class) {
                    required_features.insert(feature);
                } else {
                    emit_reject_reasons.push("ERROR:  Unable to resolve class feature");
                }
                match context.java_to_rust_path(class) {
                    Ok(path) => format!("__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, {}>>", path),
                    Err(_) => {
                        emit_reject_reasons.push("ERROR:  Failed to resolve JNI path to Rust path for return type");
                        format!("{:?}", class)
                    },
                }
            },
            method::Type::Array { levels: 1, inner: method::BasicType::Void      } => {
                emit_reject_reasons.push("ERROR:  Returning arrays of void isn't a thing");
                "???".to_owned()
            }
            method::Type::Array { levels, inner } => {
                let mut buffer = "__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, ".to_owned();
                for _ in 0..(levels-1) {
                    buffer.push_str("__jni_bindgen::ObjectArray<");
                }
                match inner {
                    method::BasicType::Boolean      => buffer.push_str("__jni_bindgen::BooleanArray"),
                    method::BasicType::Byte         => buffer.push_str("__jni_bindgen::ByteArray"),
                    method::BasicType::Char         => buffer.push_str("__jni_bindgen::CharArray"),
                    method::BasicType::Short        => buffer.push_str("__jni_bindgen::ShortArray"),
                    method::BasicType::Int          => buffer.push_str("__jni_bindgen::IntArray"),
                    method::BasicType::Long         => buffer.push_str("__jni_bindgen::LongArray"),
                    method::BasicType::Float        => buffer.push_str("__jni_bindgen::FloatArray"),
                    method::BasicType::Double       => buffer.push_str("__jni_bindgen::DoubleArray"),
                    method::BasicType::Class(class) => {
                        if let Ok(feature) = Struct::feature_for(context, class) {
                            required_features.insert(feature);
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
                        buffer.push_str(">");
                    },
                    method::BasicType::Void => {
                        emit_reject_reasons.push("ERROR:  Arrays of void isn't a thing");
                        buffer.push_str("[()]");
                    },
                }
                for _ in 0..(levels-1) {
                    buffer.push_str(">"); // ObjectArray s
                }
                buffer.push_str(">>"); // Local, Option
                buffer
            },
        };

        let mut ret_method_fragment = match descriptor.return_type() { // Contents of call_..._method_a
            method::Type::Single(method::BasicType::Void)        => "void",
            method::Type::Single(method::BasicType::Boolean)     => "boolean",
            method::Type::Single(method::BasicType::Byte)        => "byte",
            method::Type::Single(method::BasicType::Char)        => "char",
            method::Type::Single(method::BasicType::Short)       => "short",
            method::Type::Single(method::BasicType::Int)         => "int",
            method::Type::Single(method::BasicType::Long)        => "long",
            method::Type::Single(method::BasicType::Float)       => "float",
            method::Type::Single(method::BasicType::Double)      => "double",
            method::Type::Single(method::BasicType::Class(_))    => "object",
            method::Type::Array { .. }                           => "object",
        };

        if self.java.is_constructor() {
            if descriptor.return_type() == method::Type::Single(method::BasicType::Void) {
                ret_method_fragment = "object";
                ret_decl = match context.java_to_rust_path(self.class.path.as_id()) {
                    Ok(path) => format!("__jni_bindgen::Local<'env, {}>", path),
                    Err(_) => {
                        emit_reject_reasons.push("ERROR:  Failed to resolve JNI path to Rust path for this type");
                        format!("{:?}", self.class.path.as_str())
                    },
                };
            } else {
                emit_reject_reasons.push("ERROR:  Constructor should've returned void");
            }
        }

        let indent = if !emit_reject_reasons.is_empty() {
            format!("{}        // ", indent)
        } else {
            format!("{}        ", indent)
        };
        let access = if self.java.is_public() { "pub " } else { "" };
        let attributes = format!("{}",
            if self.java.deprecated { "#[deprecated] " } else { "" }
        );



        writeln!(out, "")?;
        for reason in &emit_reject_reasons {
            writeln!(out, "{}// Not emitting: {}", indent, reason)?;
        }
        if let Some(url) = KnownDocsUrl::from_method(context, self) {
            writeln!(out, "{}/// {}", indent, url)?;
        } else {
            writeln!(out, "{}/// {}", indent, self.java.name.as_str())?;
        }
        if required_features.len() > 0 {
            // Feature doc comments
            writeln!(out, "{}///", indent)?;
            write!(out, "{}/// Required features: ", indent)?;
            for (idx, feature) in required_features.iter().enumerate() {
                if idx != 0 {
                    write!(out, ", ")?;
                }
                write!(out, "{:?}", feature)?;
            }
            writeln!(out, "")?;

            // Feature cfgs
            write!(out, "{}#[cfg(any(feature = \"all\", all(", indent)?;
            for (idx, feature) in required_features.iter().enumerate() {
                if idx != 0 {
                    write!(out, ", ")?;
                }
                write!(out, "feature = {:?}", feature)?;
            }
            writeln!(out, ")))]")?;
        }
        writeln!(out, "{}{}{}fn {}<'env>({}) -> __jni_bindgen::Result<{}> {{", indent, attributes, access, method_name, params_decl, ret_decl)?;
        writeln!(out, "{}    // class.path == {:?}, java.flags == {:?}, .name == {:?}, .descriptor == {:?}", indent, &self.class.path.as_str(), self.java.flags, &self.java.name, &self.java.descriptor_str())?;
        writeln!(out, "{}    unsafe {{", indent)?;
        writeln!(out, "{}        let __jni_args = [{}];", indent, params_array)?;
        if self.java.is_constructor() || self.java.is_static() {
            match context.config.codegen.static_env {
                config::toml::StaticEnvStyle::Explicit          => {},
                config::toml::StaticEnvStyle::__NonExhaustive   => writeln!(out, "{}    let __jni_env = ...?;", indent)?, // XXX
            };
        } else {
            writeln!(out, "{}        let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);", indent)?;
        }

        writeln!(out, "{}        let (__jni_class, __jni_method) = __jni_env.require_class_{}method({}, {}, {});", indent, if self.java.is_static() { "static_" } else { "" }, emit_cstr(self.class.path.as_str()), emit_cstr(self.java.name.as_str()), emit_cstr(self.java.descriptor_str()) )?;

        if self.java.is_constructor() {
            writeln!(out, "{}        __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())", indent)?;
        } else if self.java.is_static() {
            writeln!(out, "{}        __jni_env.call_static_{}_method_a(__jni_class, __jni_method, __jni_args.as_ptr())", indent, ret_method_fragment)?;
        } else {
            writeln!(out, "{}        __jni_env.call_{}_method_a(self.0.object, __jni_method, __jni_args.as_ptr())", indent, ret_method_fragment)?;
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
