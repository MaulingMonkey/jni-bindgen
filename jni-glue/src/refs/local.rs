use super::*;

pub struct Local<'env, Class: AsValidJObjectAndEnv> {
    pub(crate) oae: ObjectAndEnv,
    pub(crate) pd:  PhantomData<&'env Class>,
}

impl<'env, Class: AsValidJObjectAndEnv> Local<'env, Class> {
    pub unsafe fn from_object_lifetime_and_raw_env_obj(_: &'env impl AsValidJObjectAndEnv, env: *const JNIEnv, object: jobject) -> Self {
        Self {
            oae: ObjectAndEnv { object, env },
            pd: PhantomData,
        }
    }
}

impl<'env, Class: AsValidJObjectAndEnv> Deref for Local<'env, Class> {
    type Target = Class;
    fn deref(&self) -> &Self::Target {
        unsafe { &*(&self.oae as *const ObjectAndEnv as *const Self::Target) }
    }
}

impl<'env, Class: AsValidJObjectAndEnv> Drop for Local<'env, Class> {
    fn drop(&mut self) {
        let env = self.oae.env as *mut JNIEnv;
        unsafe { (**env).DeleteLocalRef.unwrap()(env, self.oae.object); }
    }
}
