use super::*;

/// Represents a "safe" JavaVM.
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
