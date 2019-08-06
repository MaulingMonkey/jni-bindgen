use super::*;



/// A non-null, [reference](https://www.ibm.com/support/knowledgecenter/en/SSYKE2_8.0.0/com.ibm.java.vm.80.doc/docs/jni_refs.html)
/// to a Java object (+ &Env).  This may refer to a Local, Global, local Argument, etc.
/// 
/// **Not FFI Safe:**  #[repr(rust)], and exactly layout is likely to change depending on exact features used in the
/// future.  Specifically, on Android, since we're guaranteed to only have a single ambient VM, we can likely store the
/// *const JNIEnv in thread local storage instead of lugging it around in every Local.  Of course, there's no
/// guarantee that's actually an *optimization*...
pub struct Ref<'env, Class: AsValidJObjectAndEnv> {
    pub(crate) oae:    ObjectAndEnv,
    pub(crate) _env:   PhantomData<&'env Env>,
    pub(crate) _class: PhantomData<&'env Class>,
}

impl<'env, Class: AsValidJObjectAndEnv> Deref for Ref<'env, Class> {
    type Target = Class;
    fn deref(&self) -> &Self::Target {
        unsafe { &*(&self.oae as *const ObjectAndEnv as *const Self::Target) }
    }
}
