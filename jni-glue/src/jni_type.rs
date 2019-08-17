use super::*;

/// JNI bindings rely on this type being accurate.
/// 
/// **unsafe**:  static_with_jni_type must pass a string terminated by '\0'.  Failing to do so is a soundness bug, as
/// the string is passed directly to JNI as a raw pointer!  Additionally, passing the wrong type may be a soundness bug
/// as although the Android JVM will simply panic and abort, I've no idea if that's a guarantee or not.
/// 
/// Why the awkward callback style instead of returning `&'static str`?  Arrays of arrays may need to dynamically
/// construct their type strings, which would need to leak.  Worse, we can't easily intern those strings via
/// lazy_static without running into:
/// 
/// ```text
/// error[E0401]: can't use generic parameters from outer function
/// ```
pub unsafe trait JniType {
    fn static_with_jni_type<R>(callback: impl FnOnce(&str) -> R) -> R;
}

unsafe impl JniType for ()      { fn static_with_jni_type<R>(callback: impl FnOnce(&str) -> R) -> R { callback("V\0") } }
unsafe impl JniType for bool    { fn static_with_jni_type<R>(callback: impl FnOnce(&str) -> R) -> R { callback("Z\0") } }
unsafe impl JniType for jbyte   { fn static_with_jni_type<R>(callback: impl FnOnce(&str) -> R) -> R { callback("B\0") } }
unsafe impl JniType for jchar   { fn static_with_jni_type<R>(callback: impl FnOnce(&str) -> R) -> R { callback("C\0") } }
unsafe impl JniType for jshort  { fn static_with_jni_type<R>(callback: impl FnOnce(&str) -> R) -> R { callback("S\0") } }
unsafe impl JniType for jint    { fn static_with_jni_type<R>(callback: impl FnOnce(&str) -> R) -> R { callback("I\0") } }
unsafe impl JniType for jlong   { fn static_with_jni_type<R>(callback: impl FnOnce(&str) -> R) -> R { callback("J\0") } }
unsafe impl JniType for jfloat  { fn static_with_jni_type<R>(callback: impl FnOnce(&str) -> R) -> R { callback("F\0") } }
unsafe impl JniType for jdouble { fn static_with_jni_type<R>(callback: impl FnOnce(&str) -> R) -> R { callback("D\0") } }
unsafe impl JniType for &str    { fn static_with_jni_type<R>(callback: impl FnOnce(&str) -> R) -> R { callback("Ljava/lang/String;\0") } }
