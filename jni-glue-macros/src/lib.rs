extern crate proc_macro;

mod argument_and_return;
mod context;
mod escape;
mod parsing1;
mod parsing2;
mod skip_condition;

use argument_and_return::{Argument, Return};
use context::Context;
use parsing1::*;
use parsing2::*;
use skip_condition::*;

// TODO: More usage / examples for doc comments here

/// Generate Rust bindings to safely implement Java `native` methods.
/// 
/// ## Preamble
/// 
/// To avoid encouraging circular dependencies between Java and Rust within a single crate, this macro provides no means
/// of directly defining Java classes from `.rs` files.  Instead, you should define them in a corresponding `.java`
/// file.  You might consider placing `JavaClass.rs` and `JavaClass.java` alongside each other in your src tree and
/// automatically building the latter with my other crate, [jerk](https://github.com/MaulingMonkey/jerk).
/// 
/// Note that this macro currently **does not** yet validate impl blocks against `.class` or `.jar` files.  This means
/// typos and function signature mismatches can lead to link errors, runtime errors, or even soundness bugs - see the
/// safety section for more details!
/// 
/// ## Example
/// 
/// Lets say we wanted to implement the following Java classes:
/// 
/// ```java
/// package com.maulingmonkey.jni_bindgen.example_jni_java;
/// 
/// public class Outer {
///     public native float add(float a, float b);
/// 
///     public class Inner {
///         public static native int add(int a, int b);
///     }
/// }
/// ```
/// 
/// We have two top-level statements available to us - imports and impl blocks:
/// 
/// ```ignore
/// jni_glue_macros::java! {
/// 
///     // 1. Import statements, which let us define convenient shorthand using Java-like syntax.
///     //    Note that you must use '$' to separate an inner class, not '.':
///     import com.maulingmonkey.jni_bindgen.example_jni_java.Outer;
///     import com.maulingmonkey.jni_bindgen.example_jni_java.Outer$Inner;
///     import java.lang.String;
/// 
///     // 2. unsafe impl class blocks.  This uses a Rust-like syntax, but Java-like class names.
///     unsafe impl class Outer {
/// 
///         // Methods use Java-like syntax (return values, naming, method signatures, etc.
///         // Note that instance methods take `(&env, this)` instead of `(&self)` as built-ins.
///         // All methods also return Result<T, Throw> instead of T.
///         float add(&env, this, float a, float b) {
///             let r = a + b;
///             Ok(r)
///         }
/// 
///     }
/// 
///     // Inner classes get their own, separate impl blocks
///     unsafe impl class Inner {
/// 
///         // Static methods get:  (&env, class, ...)
///         // Instead of:          (&env, this, ...)
///         static int add(&env, class, int a, int b) {
///             let r = a + b;
///             Ok(r)
///         }
/// 
///     }
/// 
/// }
/// ```
/// 
/// ## Safety
/// 
/// This macro currently doesn't check function signatures against .class files.  As such, the following mistakes in
/// using this macro **can lead to unsound code**!:
/// 
/// * Using `this` on static methods instead of `static` + `class`, or vicea versa.
/// * Getting the return type wrong, as it's not part of the JNI function signature.
/// 
/// The following behaviors "should" be sound, but will likely lead to link-time or run-time errors:
/// 
/// * Implementing classes/methods that don't exist (the code will simply be ignored)
/// * Getting argument types wrong, as they're unconditionally incorporated into the JNI function signature.
/// 
/// ## Generated Code
/// 
/// The above example will generate all our unsafe JNI boilerplate, along the lines of:
/// 
/// ```ignore
/// // For demonstration purpouses only, generated code uses idents directly
/// use std::any::Any;
/// use jni_sys::{jobject, jfloat, jint};
/// use jni_glue::{Env, Throw};
/// use Panic = Box<dyn 'static + Any + Send>;
/// 
/// // Inner methods (generated names may differ)
/// fn add(env: &jni_glue::Env, this: jobject, a: f32, b: f32) -> Result<f32, Throw> {
///     let r = a + b;
///     Ok(r)
/// }
/// 
/// fn add(env: &jni_glue::Env, class: jclass, a: i32, b: i32) -> Result<i32, Throw> {
///     let r = a + b;
///     Ok(r)
/// }
/// 
/// // Outer methods
/// #[no_mangle] #[doc(hidden)] pub unsafe extern "stdcall"
/// Java_com_maulingmonkey_jni_1bindgen_example_1jni_1java_Outer_add__FF
/// (env: &jni_glue::Env, this:  jobject, a: jfloat, b: jfloat) -> jfloat {
///     // ...implementation details...
/// #   unimplemented!();
/// }
/// 
/// #[no_mangle] #[doc(hidden)] pub unsafe extern "stdcall"
/// Java_com_maulingmonkey_jni_1bindgen_example_1jni_1java_Outer_00024Inner_add__II
/// (env: &jni_glue::Env, class: jclass,  a: jint,   b: jint  ) -> jint {
///     // ...implementation details...
/// #   unimplemented!();
/// }
/// ```
/// 
/// The outer methods will automatically:
/// 
/// * Pass safe-to-use Rustified types to the inner methods.
/// * Convert returned `Throw`s to Java exceptions.
/// * Convert panics to Java exceptions.
#[proc_macro]
pub fn java(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut context = Context::new();
    context.java(proc_macro2::TokenStream::from(input));
    context.output().into()
}

trait TokenIter : Clone + Iterator<Item = proc_macro2::TokenTree> {}
impl<T: Clone + Iterator<Item = proc_macro2::TokenTree>> TokenIter for T {}
