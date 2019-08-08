use super::*;



/// A [Global](https://www.ibm.com/support/knowledgecenter/en/SSYKE2_8.0.0/com.ibm.java.vm.80.doc/docs/jni_refs.html),
/// non-null, reference to a Java object (+ &VM).
/// 
/// Unlike Local, this can be stored statically and shared between threads.  This has a few caveats:
/// * You must create a GlobalRef before use.
/// * The Global can be invalidated if the VM is unloaded.
/// 
/// **Not FFI Safe:**  #[repr(rust)], and exact layout is likely to change - depending on exact features used - in the
/// future.  Specifically, on Android, since we're guaranteed to only have a single ambient VM, we can likely store the
/// *const JavaVM in static and/or thread local storage instead of lugging it around in every Local.  Of course, there's
/// no guarantee that's actually an *optimization*...
pub struct Global<Class: AsValidJObjectAndEnv> {
    pub(crate) global:  jobject,
    pub(crate) gen_vm:  GenVM,
    pub(crate) pd:      PhantomData<Class>,
}

unsafe impl<Class: AsValidJObjectAndEnv> Send for Global<Class> {}
unsafe impl<Class: AsValidJObjectAndEnv> Sync for Global<Class> {}

impl<Class: AsValidJObjectAndEnv> Global<Class> {
    pub fn with<'env>(&'env self, env: &'env Env) -> GlobalRef<'env, Class> {
        assert_eq!(self.gen_vm, env.get_gen_vm()); // Soundness check - env *must* belong to the same VM!
        unsafe { self.with_unchecked(env) }
    }

    pub unsafe fn with_unchecked<'env>(&'env self, env: &'env Env) -> GlobalRef<'env, Class> {
        let env = env.as_jni_env();
        GlobalRef {
            oae: ObjectAndEnv {
                object: self.global,
                env,
            },
            _env:   PhantomData,
            _class: PhantomData,
        }
    }
}

impl<Class: AsValidJObjectAndEnv> Drop for Global<Class> {
    fn drop(&mut self) {
        VMS.read().unwrap().use_vm(self.gen_vm, |vm|{
            vm.with_env(|env|{
                let env = env.as_jni_env();
                unsafe { (**env).DeleteGlobalRef.unwrap()(env, self.global); }
            });
        });
    }
}



/// A [Global](https://www.ibm.com/support/knowledgecenter/en/SSYKE2_8.0.0/com.ibm.java.vm.80.doc/docs/jni_refs.html),
/// non-null, reference to a Java object (+ &Env).
/// 
/// Much like Local, the inclusion of an Env means this cannot be stored statically or shared between threads.
/// 
/// **Not FFI Safe:**  #[repr(rust)], and exact layout is likely to change - depending on exact features used - in the
/// future.  Specifically, on Android, since we're guaranteed to only have a single ambient VM, we can likely store the
/// *const JNIEnv in thread local storage instead of lugging it around in every Local.  Of course, there's no
/// guarantee that's actually an *optimization*...
pub type GlobalRef<'env, Class> = Ref<'env, Class>;
