use super::*;

/// FFI: Use **&VM** instead of *const JavaVM.  This represents a global, process-wide Java exection environment.
/// 
/// On Android, there is only one VM per-process, although on desktop it's possible (if rare) to have multiple VMs
/// within the same process.  While this library does not yet support having multiple VMs active simultaniously, please
/// don't hesitate to [file an issue](https://github.com/MaulingMonkey/bindgen-jni/issues/new) if this is an important
/// use case for you.
///
/// This is a "safe" alternative to jni_sys::JavaVM raw pointers, with the following caveats:
/// 
/// 1)  A null vm will result in **undefined behavior**.  Java should not be invoking your native functions with a null
///     *mut JavaVM, however, so I don't believe this is a problem in practice unless you've bindgened the C header
///     definitions elsewhere, calling them (requiring `unsafe`), and passing null pointers (generally UB for JNI
///     functions anyways, so can be seen as a caller soundness issue.)
/// 
/// 2)  Allowing the underlying JavaVM to be modified is **undefined behavior**.  I don't believe the JNI libraries
///     modify the JavaVM, so as long as you're not accepting a *mut JavaVM elsewhere, using unsafe to dereference it,
///     and mucking with the methods on it yourself, I believe this "should" be fine.
#[repr(transparent)]
pub struct VM(JavaVM);
impl VM {
    pub fn as_java_vm(&self) -> *const JavaVM { &self.0 }
    pub unsafe fn from_jni_local(vm: &JavaVM) -> &VM { &*(vm as *const JavaVM as *const VM) }

    pub fn with_env(&self, callback: impl FnOnce(&Env)) {
        let mut java_vm = self.0;
        let mut env = null_mut();
        match unsafe { (*java_vm).GetEnv.unwrap()(&mut java_vm, &mut env, JNI_VERSION_1_2) } {
            JNI_OK => callback(unsafe { Env::from_jni_void_ref(&env) }),
            JNI_EDETACHED => match unsafe { (*java_vm).AttachCurrentThread.unwrap()(&mut java_vm, &mut env, null_mut()) } {
                JNI_OK => callback(unsafe { Env::from_jni_void_ref(&env) }),
                unexpected => panic!("AttachCurrentThread returned unknown error: {}", unexpected),
            },
            JNI_EVERSION => panic!("GetEnv returned JNI_EVERSION"),
            unexpected => panic!("GetEnv returned unknown error: {}", unexpected),
        }
    }
}

unsafe impl Send for VM {}
unsafe impl Sync for VM {}
