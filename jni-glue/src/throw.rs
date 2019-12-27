use super::*;

/// A type-erased exception/jthrowable wrapper for use in Result s
pub struct Throw(ObjectAndEnv);

impl Throw {
    #[doc(hidden)] pub unsafe fn new(throwable: &impl ThrowableType) -> Self {
        let throwable = throwable.as_oae();
        // The fact that we're copying the OAE here is a very incredibly bad sign that this is unsafe.
        Self(ObjectAndEnv {
            env:    throwable.env,
            object: throwable.object,
        })
    }

    pub(crate) fn as_oae(&self) -> &ObjectAndEnv { &self.0 }
}

// These methods probably aren't sound as-is.
// impl<'env, T: ThrowableType> From<Local<'env, T>> for Throw { fn from(local: Local<'env, T>) -> Self { unsafe { Throw::new(&*local) } } }
// impl<'env, T: ThrowableType> From<Ref  <'env, T>> for Throw { fn from(local: Ref  <'env, T>) -> Self { unsafe { Throw::new(&*local) } } }
