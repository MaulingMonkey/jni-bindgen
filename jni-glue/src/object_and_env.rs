use jni_sys::*;

#[repr(C)] // Given how frequently we transmute to/from this, we'd better keep a consistent layout.
pub struct ObjectAndEnv {
    pub object: jobject,
    pub env:    *const JNIEnv,
}
