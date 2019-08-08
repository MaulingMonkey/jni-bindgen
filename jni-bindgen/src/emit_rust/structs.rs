use super::*;
use class_file_visitor::method::*;

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

        for method in self.java_class.methods() {
            if !method.access_flags().contains(MethodAccessFlags::PUBLIC) { continue; } // Skip private/protected methods
            let method_name = if let Ok(name) = context.config.codegen.method_naming_style.mangle(method.name(), method.descriptor()) { name } else { continue };
            *id_repeats.entry(method_name).or_insert(0) += 1;
        }

        // TODO: fields

        for method in self.java_class.methods() {
            let mut emit_reject_reasons = Vec::new();

            let constructor = method.name() == "<init>";
            let static_init = method.name() == "<clinit>";
            let public      = method.access_flags().contains(MethodAccessFlags::PUBLIC);
            let protected   = method.access_flags().contains(MethodAccessFlags::PROTECTED);
            let static_     = method.access_flags().contains(MethodAccessFlags::STATIC);
            let varargs     = method.access_flags().contains(MethodAccessFlags::VARARGS);
            let bridge      = method.access_flags().contains(MethodAccessFlags::BRIDGE);
            // Ignored: FINAL | SYNCRONIZED | NATIVE | ABSTRACT | STRICT | SYNTHETIC
            let _private    = !public && !protected;
            let _access     = if public { "public" } else if protected { "protected" } else { "private" };

            let java_class              = self.java_class.this_class().name();
            let java_class_method       = format!("{}\x1f{}", self.java_class.this_class().name(), method.name());
            let java_class_method_sig   = format!("{}\x1f{}\x1f{}", self.java_class.this_class().name(), method.name(), method.descriptor());

            let ignored =
                context.config.ignore_classes          .contains( java_class) ||
                context.config.ignore_class_methods    .contains(&java_class_method) ||
                context.config.ignore_class_method_sigs.contains(&java_class_method_sig);

            let renamed_to = context.config.rename_classes.get(java_class)
                .or_else(||  context.config.rename_class_methods.get(&java_class_method))
                .or_else(||  context.config.rename_class_method_sigs.get(&java_class_method_sig));

            let descriptor = method.descriptor();

            let method_name = if let Ok(name) = context.config.codegen.method_naming_style.mangle(method.name(), method.descriptor()) {
                name
            } else {
                emit_reject_reasons.push("Failed to mangle method name");
                method.name().to_owned()
            };

            let repeats = *id_repeats.get(&method_name).unwrap_or(&0);
            let overloaded = repeats > 1;

            let method_name = if let Some(name) = renamed_to {
                name.clone()
            } else if overloaded {
                if let Ok(name) = context.config.codegen.method_naming_style_collision.mangle(method.name(), method.descriptor()) {
                    name
                } else {
                    method_name
                }
            } else {
                method_name
            };

            if !public      { emit_reject_reasons.push("Non-public method"); }
            if varargs      { emit_reject_reasons.push("Marked as varargs - haven't decided on how I want to handle this."); }
            if bridge       { emit_reject_reasons.push("Bridge method - type erasure"); }
            if static_init  { emit_reject_reasons.push("Static class constructor - never needs to be called by Rust."); }
            if ignored      { emit_reject_reasons.push("[[ignore]]d"); }

            // Parameter names may or may not be available as extra debug information.  Example:
            // https://docs.oracle.com/javase/tutorial/reflect/member/methodparameterreflection.html

            
            let mut params_array = String::new(); // Contents of let __jni_args = [...];
            let mut ret_decl = String::new();     // Contents of fn name<'env>() -> Result<...> {
            let mut ret_method_fragment = "";     // Contents of Call...MethodA
            let descriptor = if let Ok(d) = JniDescriptor::new(descriptor) {
                d
            } else {
                emit_reject_reasons.push("Invalid method descriptor");
                JniDescriptor::new("()V").unwrap()
            };

            // Contents of fn name<'env>(...) {
            let mut params_decl = if constructor || static_ {
                match context.config.codegen.static_env {
                    config::toml::StaticEnvStyle::Explicit => String::from("__jni_env: &'env __jni_bindgen::Env"),
                    config::toml::StaticEnvStyle::Implicit => {
                        emit_reject_reasons.push("StaticEnvStyle::Implicit not yet implemented");
                        String::new()
                    },
                    config::toml::StaticEnvStyle::__NonExhaustive => {
                        emit_reject_reasons.push("StaticEnvStyle::__NonExhaustive is invalid, silly goose!");
                        String::new()
                    },
                }
            } else {
                String::from("&'env self")
            };

            for (arg_idx, segment) in descriptor.enumerate() {
                match segment {
                    JniDescriptorSegment::Parameter(parameter) => {
                        let arg_name = format!("arg{}", arg_idx);

                        let mut param_is_object = false; // XXX

                        let arg_type = match parameter {
                            JniField::Single(JniBasicType::Void) => {
                                emit_reject_reasons.push("Void arguments aren't a thing");
                                "???".to_owned()
                            },
                            JniField::Single(JniBasicType::Boolean)     => "bool".to_owned(),
                            JniField::Single(JniBasicType::Byte)        => "i8".to_owned(),
                            JniField::Single(JniBasicType::Char)        => "__jni_bindgen::jchar".to_owned(),
                            JniField::Single(JniBasicType::Short)       => "i16".to_owned(),
                            JniField::Single(JniBasicType::Int)         => "i32".to_owned(),
                            JniField::Single(JniBasicType::Long)        => "i64".to_owned(),
                            JniField::Single(JniBasicType::Float)       => "f32".to_owned(),
                            JniField::Single(JniBasicType::Double)      => "f64".to_owned(),
                            JniField::Single(JniBasicType::Class(class)) => {
                                param_is_object = true;
                                match context.java_to_rust_path(class) {
                                    Ok(path) => format!("impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env {}>>", path),
                                    Err(_) => {
                                        emit_reject_reasons.push("Failed to resolve JNI path to Rust path for argument type");
                                        format!("{:?}", class)
                                    }
                                }
                            },
                            JniField::Array { levels: 1, inner: JniBasicType::Void      } => {
                                emit_reject_reasons.push("Accepting arrays of void isn't a thing");
                                "???".to_owned()
                            }
                            JniField::Array { levels: 1, inner: JniBasicType::Boolean   } => { param_is_object = true; "impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::BooleanArray>>".to_owned() },
                            JniField::Array { levels: 1, inner: JniBasicType::Byte      } => { param_is_object = true; "impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ByteArray   >>".to_owned() },
                            JniField::Array { levels: 1, inner: JniBasicType::Char      } => { param_is_object = true; "impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::CharArray   >>".to_owned() },
                            JniField::Array { levels: 1, inner: JniBasicType::Short     } => { param_is_object = true; "impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::ShortArray  >>".to_owned() },
                            JniField::Array { levels: 1, inner: JniBasicType::Int       } => { param_is_object = true; "impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::IntArray    >>".to_owned() },
                            JniField::Array { levels: 1, inner: JniBasicType::Long      } => { param_is_object = true; "impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::LongArray   >>".to_owned() },
                            JniField::Array { levels: 1, inner: JniBasicType::Float     } => { param_is_object = true; "impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::FloatArray  >>".to_owned() },
                            JniField::Array { levels: 1, inner: JniBasicType::Double    } => { param_is_object = true; "impl __jni_bindgen::std::convert::Into<__jni_bindgen::std::option::Option<&'env __jni_bindgen::DoubleArray >>".to_owned() },
                            JniField::Array { .. } => {
                                param_is_object = true;
                                emit_reject_reasons.push("Passing in arrays of arrays/objects is not yet supported");
                                format!("{:?}", parameter)
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
                    },
                    JniDescriptorSegment::Return(ret) => {
                        ret_decl = match ret {
                            JniField::Single(JniBasicType::Void)        => "()".to_owned(),
                            JniField::Single(JniBasicType::Boolean)     => "bool".to_owned(),
                            JniField::Single(JniBasicType::Byte)        => "i8".to_owned(),
                            JniField::Single(JniBasicType::Char)        => "__jni_bindgen::jchar".to_owned(),
                            JniField::Single(JniBasicType::Short)       => "i16".to_owned(),
                            JniField::Single(JniBasicType::Int)         => "i32".to_owned(),
                            JniField::Single(JniBasicType::Long)        => "i64".to_owned(),
                            JniField::Single(JniBasicType::Float)       => "f32".to_owned(),
                            JniField::Single(JniBasicType::Double)      => "f64".to_owned(),
                            JniField::Single(JniBasicType::Class(class)) => {
                                match context.java_to_rust_path(class) {
                                    Ok(path) => format!("__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, {}>>", path),
                                    Err(_) => {
                                        emit_reject_reasons.push("Failed to resolve JNI path to Rust path for return type");
                                        format!("{:?}", class)
                                    },
                                }
                            },
                            JniField::Array { levels: 1, inner: JniBasicType::Void      } => {
                                emit_reject_reasons.push("Returning arrays of void isn't a thing");
                                "???".to_owned()
                            }
                            JniField::Array { levels: 1, inner: JniBasicType::Boolean   } => "__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::BooleanArray>>".to_owned(),
                            JniField::Array { levels: 1, inner: JniBasicType::Byte      } => "__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ByteArray   >>".to_owned(),
                            JniField::Array { levels: 1, inner: JniBasicType::Char      } => "__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::CharArray   >>".to_owned(),
                            JniField::Array { levels: 1, inner: JniBasicType::Short     } => "__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::ShortArray  >>".to_owned(),
                            JniField::Array { levels: 1, inner: JniBasicType::Int       } => "__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::IntArray    >>".to_owned(),
                            JniField::Array { levels: 1, inner: JniBasicType::Long      } => "__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::LongArray   >>".to_owned(),
                            JniField::Array { levels: 1, inner: JniBasicType::Float     } => "__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::FloatArray  >>".to_owned(),
                            JniField::Array { levels: 1, inner: JniBasicType::Double    } => "__jni_bindgen::std::option::Option<__jni_bindgen::Local<'env, __jni_bindgen::DoubleArray >>".to_owned(),
                            JniField::Array { .. } => {
                                emit_reject_reasons.push("Returning arrays of objects or arrays not yet supported");
                                format!("{:?}", ret)
                            }
                        };

                        ret_method_fragment = match ret {
                            JniField::Single(JniBasicType::Void)        => "void",
                            JniField::Single(JniBasicType::Boolean)     => "boolean",
                            JniField::Single(JniBasicType::Byte)        => "byte",
                            JniField::Single(JniBasicType::Char)        => "char",
                            JniField::Single(JniBasicType::Short)       => "short",
                            JniField::Single(JniBasicType::Int)         => "int",
                            JniField::Single(JniBasicType::Long)        => "long",
                            JniField::Single(JniBasicType::Float)       => "float",
                            JniField::Single(JniBasicType::Double)      => "double",
                            JniField::Single(JniBasicType::Class(_))    => "object",
                            JniField::Array { .. }                      => "object",
                        };
                    },
                }
            }

            if constructor {
                if ret_method_fragment == "void" {
                    ret_method_fragment = "object";
                    ret_decl = match context.java_to_rust_path(self.java_class.this_class().name()) {
                        Ok(path) => format!("__jni_bindgen::Local<'env, {}>", path),
                        Err(_) => {
                            emit_reject_reasons.push("Failed to resolve JNI path to Rust path for this type");
                            format!("{:?}", self.java_class.this_class().name())
                        },
                    };
                } else {
                    emit_reject_reasons.push("Constructor should've returned void");
                }
            }

            let indent = if !emit_reject_reasons.is_empty() {
                format!("{}        // ", indent)
            } else {
                format!("{}        ", indent)
            };
            let access = if public { "pub " } else { "" };
            writeln!(out, "")?;
            for reason in &emit_reject_reasons {
                writeln!(out, "{}// Not emitting: {}", indent, reason)?;
            }
            if let Some(url) = KnownDocsUrl::from_method(context, self.java_class.this_class().name(), method.name(), method.descriptor()) {
                writeln!(out, "{}/// [{}]({})", indent, url.label, url.url)?;
            }
            writeln!(out, "{}{}fn {}<'env>({}) -> __jni_bindgen::Result<{}> {{", indent, access, method_name, params_decl, ret_decl)?;
            writeln!(out, "{}    // class.name() == {:?}, method.access_flags() == {:?}, .name() == {:?}, .descriptor() == {:?}", indent, self.java_class.this_class().name(), method.access_flags(), method.name(), method.descriptor())?;
            writeln!(out, "{}    unsafe {{", indent)?;
            writeln!(out, "{}        let __jni_args = [{}];", indent, params_array)?;
            if constructor || static_ {
                match context.config.codegen.static_env {
                    config::toml::StaticEnvStyle::Explicit          => {},
                    config::toml::StaticEnvStyle::Implicit          => writeln!(out, "{}    let __jni_env = ...?;", indent)?, // XXX
                    config::toml::StaticEnvStyle::__NonExhaustive   => writeln!(out, "{}    let __jni_env = ...?;", indent)?, // XXX
                };
            } else {
                writeln!(out, "{}        let __jni_env = __jni_bindgen::Env::from_ptr(self.0.env);", indent)?;
            }

            writeln!(out, "{}        let (__jni_class, __jni_method) = __jni_env.require_class_{}method({}, {}, {});", indent, if static_ { "static_" } else { "" }, emit_cstr(self.java_class.this_class().name()), emit_cstr(method.name()), emit_cstr(method.descriptor()) )?;

            if constructor {
                writeln!(out, "{}        __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())", indent)?;
            } else if static_ {
                writeln!(out, "{}        __jni_env.call_static_{}_method_a(__jni_class, __jni_method, __jni_args.as_ptr())", indent, ret_method_fragment)?;
            } else {
                writeln!(out, "{}        __jni_env.call_{}_method_a(self.0.object, __jni_method, __jni_args.as_ptr())", indent, ret_method_fragment)?;
            }
            writeln!(out, "{}    }}", indent)?;
            writeln!(out, "{}}}", indent)?;
        }

        // TODO: fields

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
