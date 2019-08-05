use super::*;

pub struct Global<Class: AsValidJObjectAndEnv> {
    pub(crate) global:  jobject,
    pub(crate) gen_vm:  GenVM,
    pub(crate) pd:      PhantomData<Class>,
}

unsafe impl<Class: AsValidJObjectAndEnv> Send for Global<Class> {}
unsafe impl<Class: AsValidJObjectAndEnv> Sync for Global<Class> {}

impl<Class: AsValidJObjectAndEnv> Global<Class> {
    pub fn to_local<'env>(&'_ self, env: &'env Env) -> Local<'env, Class> {
        let env = env.as_jni_env();
        let object = unsafe { (**env).NewLocalRef.unwrap()(env, self.global) };
        assert!(!object.is_null());
        Local {
            oae: ObjectAndEnv {
                object,
                env,
            },
            pd: PhantomData,
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
