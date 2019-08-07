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
            write!(out, "{}", &context.java_to_rust_path(interface.name()).unwrap())?;
        }
        writeln!(out, " {{")?;

        let mut id_repeats = HashMap::new();

        for method in self.java_class.methods() {
            if !method.access_flags().contains(MethodAccessFlags::PUBLIC) { continue; } // Skip private/protected methods
            let method_name = if let Ok(name) = MethodManglingStyle::Rustify.mangle(method.name(), method.descriptor()) { name } else { continue };
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

            let method_name = if let Ok(name) = MethodManglingStyle::Rustify.mangle(method.name(), method.descriptor()) {
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
                if let Ok(name) = MethodManglingStyle::RustifyShortSignature.mangle(method.name(), method.descriptor()) {
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
            //if overloaded   { emit_reject_reasons.push("Overloaded - I haven't decided how I want to deconflict overloads."); }
            if static_init  { emit_reject_reasons.push("Static class constructor - never needs to be called by Rust."); }
            if constructor  { emit_reject_reasons.push("I haven't decided how to pass JNIEnv for constructors yet."); }
            if static_      { emit_reject_reasons.push("I haven't decided how to pass JNIEnv for static methods yet."); }
            if ignored      { emit_reject_reasons.push("[[ignore]]d"); }

            // Parameter names may or may not be available as extra debug information.  Example:
            // https://docs.oracle.com/javase/tutorial/reflect/member/methodparameterreflection.html

            let mut params_array = String::new();
            let mut params_decl  = String::from(if static_ || constructor { "" } else { "&'env self" });
            let mut ret_decl = String::new();
            let mut ret_method_fragment = "";
            let descriptor = if let Ok(d) = JniDescriptor::new(descriptor) {
                d
            } else {
                emit_reject_reasons.push("Invalid method descriptor");
                JniDescriptor::new("()V").unwrap()
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
                            JniField::Single(JniBasicType::Char)        => "__bindgen_jni::jchar".to_owned(),
                            JniField::Single(JniBasicType::Short)       => "i16".to_owned(),
                            JniField::Single(JniBasicType::Int)         => "i32".to_owned(),
                            JniField::Single(JniBasicType::Long)        => "i64".to_owned(),
                            JniField::Single(JniBasicType::Float)       => "f32".to_owned(),
                            JniField::Single(JniBasicType::Double)      => "f64".to_owned(),
                            JniField::Single(JniBasicType::Class(class)) => {
                                param_is_object = true;
                                match context.java_to_rust_path(class) {
                                    // TODO: Should this take an Option?  Probably...
                                    Ok(path) => format!("&impl __bindgen_jni::std::convert::AsRef<{}>", path),
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
                            // TODO: Should these take an Option?  Probably...
                            JniField::Array { levels: 1, inner: JniBasicType::Boolean   } => { param_is_object = true; "&impl __bindgen_jni::std::convert::AsRef<__bindgen_jni::BooleanArray>".to_owned() },
                            JniField::Array { levels: 1, inner: JniBasicType::Byte      } => { param_is_object = true; "&impl __bindgen_jni::std::convert::AsRef<__bindgen_jni::ByteArray   >".to_owned() },
                            JniField::Array { levels: 1, inner: JniBasicType::Char      } => { param_is_object = true; "&impl __bindgen_jni::std::convert::AsRef<__bindgen_jni::CharArray   >".to_owned() },
                            JniField::Array { levels: 1, inner: JniBasicType::Short     } => { param_is_object = true; "&impl __bindgen_jni::std::convert::AsRef<__bindgen_jni::ShortArray  >".to_owned() },
                            JniField::Array { levels: 1, inner: JniBasicType::Int       } => { param_is_object = true; "&impl __bindgen_jni::std::convert::AsRef<__bindgen_jni::IntArray    >".to_owned() },
                            JniField::Array { levels: 1, inner: JniBasicType::Long      } => { param_is_object = true; "&impl __bindgen_jni::std::convert::AsRef<__bindgen_jni::LongArray   >".to_owned() },
                            JniField::Array { levels: 1, inner: JniBasicType::Float     } => { param_is_object = true; "&impl __bindgen_jni::std::convert::AsRef<__bindgen_jni::FloatArray  >".to_owned() },
                            JniField::Array { levels: 1, inner: JniBasicType::Double    } => { param_is_object = true; "&impl __bindgen_jni::std::convert::AsRef<__bindgen_jni::DoubleArray >".to_owned() },
                            JniField::Array { .. } => {
                                param_is_object = true;
                                emit_reject_reasons.push("Passing in arrays not yet supported");
                                format!("{:?}", parameter)
                            },
                        };

                        if !params_array.is_empty() {
                            params_array.push_str(", ");
                        }

                        params_array.push_str("__bindgen_jni::AsJValue::as_jvalue(");
                        if !param_is_object { params_array.push_str("&"); }
                        params_array.push_str(arg_name.as_str());
                        if param_is_object { params_array.push_str(".as_ref()"); }
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
                            JniField::Single(JniBasicType::Char)        => "__bindgen_jni::jchar".to_owned(),
                            JniField::Single(JniBasicType::Short)       => "i16".to_owned(),
                            JniField::Single(JniBasicType::Int)         => "i32".to_owned(),
                            JniField::Single(JniBasicType::Long)        => "i64".to_owned(),
                            JniField::Single(JniBasicType::Float)       => "f32".to_owned(),
                            JniField::Single(JniBasicType::Double)      => "f64".to_owned(),
                            JniField::Single(JniBasicType::Class(class)) => {
                                match context.java_to_rust_path(class) {
                                    Ok(path) => format!("__bindgen_jni::std::option::Option<__bindgen_jni::Local<'env, {}>>", path),
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
                            JniField::Array { levels: 1, inner: JniBasicType::Boolean   } => "__bindgen_jni::std::option::Option<__bindgen_jni::Local<'env, __bindgen_jni::BooleanArray>>".to_owned(),
                            JniField::Array { levels: 1, inner: JniBasicType::Byte      } => "__bindgen_jni::std::option::Option<__bindgen_jni::Local<'env, __bindgen_jni::ByteArray   >>".to_owned(),
                            JniField::Array { levels: 1, inner: JniBasicType::Char      } => "__bindgen_jni::std::option::Option<__bindgen_jni::Local<'env, __bindgen_jni::CharArray   >>".to_owned(),
                            JniField::Array { levels: 1, inner: JniBasicType::Short     } => "__bindgen_jni::std::option::Option<__bindgen_jni::Local<'env, __bindgen_jni::ShortArray  >>".to_owned(),
                            JniField::Array { levels: 1, inner: JniBasicType::Int       } => "__bindgen_jni::std::option::Option<__bindgen_jni::Local<'env, __bindgen_jni::IntArray    >>".to_owned(),
                            JniField::Array { levels: 1, inner: JniBasicType::Long      } => "__bindgen_jni::std::option::Option<__bindgen_jni::Local<'env, __bindgen_jni::LongArray   >>".to_owned(),
                            JniField::Array { levels: 1, inner: JniBasicType::Float     } => "__bindgen_jni::std::option::Option<__bindgen_jni::Local<'env, __bindgen_jni::FloatArray  >>".to_owned(),
                            JniField::Array { levels: 1, inner: JniBasicType::Double    } => "__bindgen_jni::std::option::Option<__bindgen_jni::Local<'env, __bindgen_jni::DoubleArray >>".to_owned(),
                            JniField::Array { .. } => {
                                emit_reject_reasons.push("Returning arrays of objects or arrays not yet supported");
                                format!("{:?}", ret)
                            }
                        };

                        ret_method_fragment = match ret {
                            JniField::Single(JniBasicType::Void)        => "Void",
                            JniField::Single(JniBasicType::Boolean)     => "Boolean",
                            JniField::Single(JniBasicType::Byte)        => "Byte",
                            JniField::Single(JniBasicType::Char)        => "Char",
                            JniField::Single(JniBasicType::Short)       => "Short",
                            JniField::Single(JniBasicType::Int)         => "Int",
                            JniField::Single(JniBasicType::Long)        => "Long",
                            JniField::Single(JniBasicType::Float)       => "Float",
                            JniField::Single(JniBasicType::Double)      => "Double",
                            JniField::Single(JniBasicType::Class(_))    => "Object",
                            JniField::Array { .. }                      => "Object",
                        };
                    },
                }
            }

            let ret_is_object = ret_method_fragment == "Object";

            if constructor && ret_method_fragment != "Void" { emit_reject_reasons.push("Constructor should've returned void"); }

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
            writeln!(out, "{}{}fn {}<'env>({}) -> __bindgen_jni::Result<{}> {{", indent, access, method_name, params_decl, ret_decl)?;
            writeln!(out, "{}    // class.name() == {:?}, method.access_flags() == {:?}, .name() == {:?}, .descriptor() == {:?}", indent, self.java_class.this_class().name(), method.access_flags(), method.name(), method.descriptor())?;
            writeln!(out, "{}    let __jni_args = [{}];", indent, params_array)?;
            if constructor {
                writeln!(out, "{}    let __jni_env = ...?", indent)?; // XXX
                writeln!(out, "{}    let __jni_class  = unsafe {{ (**__jni_env).FindClass.unwrap()(__jni_env, {}) }};", indent, emit_cstr(self.java_class.this_class().name()))?;
                writeln!(out, "{}    assert_ne!(__jni_class, __bindgen_jni::std::ptr::null_mut());", indent)?;
                writeln!(out, "{}    let __jni_method = unsafe {{ (**__jni_env).GetMethodID.unwrap()(__jni_env, __jni_class, {}, {}) }};", indent, emit_cstr(method.name()), emit_cstr(method.descriptor()) )?;
                writeln!(out, "{}    assert_ne!(__jni_method, __bindgen_jni::std::ptr::null_mut());", indent)?;
                writeln!(out, "{}    let result = unsafe {{ (**__jni_env).NewObjectA.unwrap()(__jni_env, __jni_class, __jni_method, __jni_args.as_ptr()) }};", indent)?;
            } else if static_ {
                writeln!(out, "{}    let __jni_env = ...?", indent)?; // XXX
                writeln!(out, "{}    let __jni_class  = unsafe {{ (**__jni_env).FindClass.unwrap()(__jni_env, {}) }};", indent, emit_cstr(self.java_class.this_class().name()))?;
                writeln!(out, "{}    assert_ne!(__jni_class, __bindgen_jni::std::ptr::null_mut());", indent)?;
                writeln!(out, "{}    let __jni_method = unsafe {{ (**__jni_env).GetStaticMethodID.unwrap()(__jni_env, __jni_class, {}, {}) }};", indent, emit_cstr(method.name()), emit_cstr(method.descriptor()) )?;
                writeln!(out, "{}    assert_ne!(__jni_method, __bindgen_jni::std::ptr::null_mut());", indent)?;
                writeln!(out, "{}    let result = unsafe {{ (**__jni_env).CallStatic{}MethodA.unwrap()(__jni_env, __jni_class, __jni_method, __jni_args.as_ptr()) }};", indent, ret_method_fragment)?;
            } else {
                writeln!(out, "{}    let __jni_this   = self.0.object;", indent)?;
                writeln!(out, "{}    let __jni_env    = self.0.env as *mut __bindgen_jni::jni_sys::JNIEnv;", indent)?;
                writeln!(out, "{}    let __jni_class  = unsafe {{ (**__jni_env).FindClass.unwrap()(__jni_env, {}) }};", indent, emit_cstr(self.java_class.this_class().name()))?;
                writeln!(out, "{}    assert_ne!(__jni_class, __bindgen_jni::std::ptr::null_mut());", indent)?;
                writeln!(out, "{}    let __jni_method = unsafe {{ (**__jni_env).GetMethodID.unwrap()(__jni_env, __jni_class, {}, {}) }};", indent, emit_cstr(method.name()), emit_cstr(method.descriptor()) )?;
                writeln!(out, "{}    assert_ne!(__jni_method, __bindgen_jni::std::ptr::null_mut());", indent)?;
                writeln!(out, "{}    let result = unsafe {{ (**__jni_env).Call{}MethodA.unwrap()(__jni_env, __jni_this, __jni_method, __jni_args.as_ptr()) }};", indent, ret_method_fragment)?;
            }

            writeln!(out, "{}    let __jni_exception = unsafe {{ (**__jni_env).ExceptionOccurred.unwrap()(__jni_env) }};", indent)?;
            writeln!(out, "{}    if !__jni_exception.is_null() {{", indent)?;
            writeln!(out, "{}        unsafe {{ (**__jni_env).ExceptionClear.unwrap()(__jni_env) }};", indent)?;
            writeln!(out, "{}        Err(__jni_exception)", indent)?;
            if ret_is_object {
                writeln!(out, "{}    }} else if result.is_null() {{", indent)?;
                writeln!(out, "{}        Ok(None)", indent)?;
                writeln!(out, "{}    }} else {{", indent)?;
                writeln!(out, "{}        let result = unsafe {{ __bindgen_jni::Local::from_object_lifetime_and_raw_env_obj(self, __jni_env as *const __bindgen_jni::jni_sys::JNIEnv, result) }};", indent)?;
                writeln!(out, "{}        Ok(Some(result))", indent)?;
            } else if ret_method_fragment == "Boolean" {
                writeln!(out, "{}    }} else {{", indent)?;
                writeln!(out, "{}        Ok(result == __bindgen_jni::jni_sys::JNI_TRUE)", indent)?;
            } else {
                writeln!(out, "{}    }} else {{", indent)?;
                writeln!(out, "{}        Ok(result)", indent)?;
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
    s.push_str(".as_ptr() as *const __bindgen_jni::std::os::raw::c_char");
    s
}
