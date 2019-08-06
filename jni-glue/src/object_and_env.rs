use jni_sys::*;

#[repr(C)] // Given how frequently we transmute to/from this, we'd better keep a consistent layout.
#[doc(hidden)] // You should generally not be interacting with this type directly, but it must be public for codegen.
pub struct ObjectAndEnv {
    pub object: jobject,
    pub env:    *const JNIEnv,
}
