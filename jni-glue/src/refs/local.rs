use super::*;



/// A [Local](https://www.ibm.com/support/knowledgecenter/en/SSYKE2_8.0.0/com.ibm.java.vm.80.doc/docs/jni_refs.html),
/// non-null, reference to a Java object (+ &[Env]) limited to the current thread/stack.
/// 
/// Including the env allows for the convenient execution of methods without having to individually pass the env as an
/// argument to each and every one.  Since this is limited to the current thread/stack, these cannot be sanely stored
/// in any kind of static storage, nor shared between threads - instead use a [Global] if you need to do either.
/// 
/// Will DeleteLocalRef when dropped, invalidating the jobject but ensuring threads that rarely or never return to
/// Java may run without being guaranteed to eventually exhaust their local reference limit.  If this is not desired,
/// convert to a plain Ref with:
/// 
/// ```rust,no_run
/// # use jni_glue::*;
/// # fn example<Class: AsValidJObjectAndEnv>(local: Local<Class>) {
/// let local = Local::leak(local);
/// # }
/// ```
/// 
/// **Not FFI Safe:**  #\[repr(rust)\], and exact layout is likely to change - depending on exact features used - in the
/// future.  Specifically, on Android, since we're guaranteed to only have a single ambient VM, we can likely store the
/// \*const JNIEnv in thread local storage instead of lugging it around in every Local.  Of course, there's no
/// guarantee that's actually an *optimization*...
/// 
/// [Env]:    struct.Env.html
/// [Global]: struct.Global.html
pub struct Local<'env, Class: AsValidJObjectAndEnv> {
    pub(crate) oae:    ObjectAndEnv,
    pub(crate) _env:   PhantomData<&'env Env>,
    pub(crate) _class: PhantomData<&'env Class>,
}

// Could implement clone if necessary via NewLocalRef
// Do *not* implement Copy, cannot be safely done.

impl<'env, Class: AsValidJObjectAndEnv> Local<'env, Class> {
    pub unsafe fn from_env_object(env: *const JNIEnv, object: jobject) -> Self {
        Self {
            oae: ObjectAndEnv { object, env },
            _env:   PhantomData,
            _class: PhantomData,
        }
    }

    pub fn leak(local: Self) -> Ref<'env, Class> {
        let result = Ref {
            oae: ObjectAndEnv {
                object: local.oae.object,
                env:    local.oae.env,
            },
            _env:   PhantomData,
            _class: PhantomData,
        };
        std::mem::forget(local); // Don't allow local to DeleteLocalRef the jobject
        result
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
