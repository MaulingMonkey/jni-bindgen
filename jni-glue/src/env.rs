use super::*;

/// FFI:  Use **&Env** instead of *const JNIEnv.  This represents a per-thread Java exection environment.
/// 
/// A "safe" alternative to jni_sys::JNIEnv raw pointers, with the following caveats:
/// 
/// 1)  A null env will result in **undefined behavior**.  Java should not be invoking your native functions with a null
///     *mut JNIEnv, however, so I don't believe this is a problem in practice unless you've bindgened the C header
///     definitions elsewhere, calling them (requiring `unsafe`), and passing null pointers (generally UB for JNI
///     functions anyways, so can be seen as a caller soundness issue.)
/// 
/// 2)  Allowing the underlying JNIEnv to be modified is **undefined behavior**.  I don't believe the JNI libraries
///     modify the JNIEnv, so as long as you're not accepting a *mut JNIEnv elsewhere, using unsafe to dereference it,
///     and mucking with the methods on it yourself, I believe this "should" be fine.
/// 
/// # Example
/// 
/// ### MainActivity.java
/// 
/// ```java
/// package com.maulingmonkey.example;
/// 
/// public class MainActivity extends androidx.appcompat.app.AppCompatActivity {
///     @Override
///     public native boolean dispatchKeyEvent(android.view.KeyEvent keyEvent);
/// 
///     // ...
/// }
/// ```
/// 
/// ### main_activity.rs
/// 
/// ```rust
/// use jni_sys::{jboolean, jobject, JNI_TRUE}; // TODO: Replace with safer equivalent
/// use jni_glue::Env;
/// 
/// #[no_mangle] pub extern "system"
/// fn Java_com_maulingmonkey_example_MainActivity_dispatchKeyEvent<'env>(
///     _env:       &Env,
///     _this:      jobject, // TODO: Replace with safer equivalent
///     _key_event: jobject  // TODO: Replace with safer equivalent
/// ) -> jboolean {
///     // ...
///     JNI_TRUE
/// }
/// ```
#[repr(transparent)]
pub struct Env(JNIEnv);

impl Env {
    pub fn as_jni_env(&self) -> *mut JNIEnv { &self.0 as *const _ as *mut _ }
    pub(crate) unsafe fn from_jni_local(env: &JNIEnv) -> &Env { &*(env as *const JNIEnv as *const Env) }
    pub(crate) unsafe fn from_jni_void_ref(ptr: &*mut c_void) -> &Env { Self::from_jni_local(&*(*ptr as *const c_void as *const JNIEnv)) }

    pub(crate) fn get_gen_vm(&self) -> GenVM {
        let jni_env = self.as_jni_env();
        let mut vm = null_mut();
        let err = unsafe { (**jni_env).GetJavaVM.unwrap()(jni_env, &mut vm) };
        assert_eq!(err, JNI_OK);
        assert_ne!(vm, null_mut());
        VMS.read().unwrap().get_gen_vm(vm)
    }
}
