use super::*;

/// Represents a "safe" JNIEnv.  Construct ala:
/// 
/// ```no_run
/// use jni_sys::*;
/// use jni_glue::*;
/// #[no_mangle] #[allow(non_snake_case)] pub extern "system"
/// fn Java_example_Class_method(env: &Env, this: jobject) {
///     // Env::from_jni_local is marked unsafe, as:
///     //   1) There is no guarantee `env` was a valid pointer.
///     //   2) There is no guarantee `env` will remain valid for the duration of Env's existence.
///     // In an attempt to reduce the chances of misuse, Env::from requires env be passed by
///     // reference, and limits the resulting env's lifetime to the lifetime of that pointer.
/// 
///     // BAD, NO, STOP IT:
///     // static ENV1 : *mut JNIEnv = std::ptr::null_mut();
///     // static ENV2 : &'static Env = Env::from_jni_local(unsafe { &*ENV1 });
/// }
/// ```
#[repr(transparent)]
pub struct Env(JNIEnv);

impl Env {
    pub fn as_jni_env(&self) -> *mut JNIEnv { &self.0 as *const _ as *mut _ }
    pub unsafe fn from_jni_local(env: &JNIEnv) -> &Env { &*(env as *const JNIEnv as *const Env) }
    pub unsafe fn from_jni_void_ref(ptr: &*mut c_void) -> &Env { Self::from_jni_local(&*(*ptr as *const c_void as *const JNIEnv)) }

    fn get_gen_vm(&self) -> GenVM {
        let jni_env = self.as_jni_env();
        let mut vm = null_mut();
        let err = unsafe { (**jni_env).GetJavaVM.unwrap()(jni_env, &mut vm) };
        assert_eq!(err, JNI_OK);
        assert_ne!(vm, null_mut());
        VMS.read().unwrap().get_gen_vm(vm)
    }
}
