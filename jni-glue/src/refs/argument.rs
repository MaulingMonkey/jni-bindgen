use super::*;



/// FFI: Use **Argument\<java::lang::Object\>** instead of jobject.  This represents a (null?) function argument.
/// 
/// Unlike most Java reference types from this library, this *can* be null.
/// 
/// FFI safe where a jobject is safe, assuming you match your types correctly.  Using the wrong type may result in
/// soundness issues, but at least on Android mostly seems to just result in JNI aborting execution for the current
/// process when calling methods on an instance of the wrong type.
#[repr(transparent)]
pub struct Argument<Class: AsValidJObjectAndEnv> {
    object: jobject,
    _class: PhantomData<Class>,
}

impl<Class: AsValidJObjectAndEnv> Argument<Class> {
    /// **unsafe**:  There's no guarantee the jobject being passed is valid or null, nor any means of checking it.
    pub unsafe fn from_unchecked(object: jobject) -> Self { Self { object, _class: PhantomData } }

    /// **unsafe**:  This assumes the argument belongs to the given Env/VM, which is technically unsound.  However, the
    /// intended use case of immediately converting any Argument s into ArgumentRef s at the start of a JNI callback,
    /// where Java directly invoked your function with an Env + arguments, is sound.
    pub unsafe fn with_unchecked<'env>(&'env self, env: &'env Env) -> Option<ArgumentRef<'env, Class>> {
        if self.object.is_null() {
            None
        } else {
            let env = env.as_jni_env();
            Some(ArgumentRef {
                oae: ObjectAndEnv {
                    object: self.object,
                    env,
                },
                _env:   PhantomData,
                _class: PhantomData,
            })
        }
    }
}



/// A [Local](https://www.ibm.com/support/knowledgecenter/en/SSYKE2_8.0.0/com.ibm.java.vm.80.doc/docs/jni_refs.html),
/// non-null, reference to a Java object (+ &Env).
/// 
/// Much like Local, the inclusion of an Env means this cannot be stored statically or shared between threads.
/// 
/// **Not FFI Safe:**  #[repr(rust)], and exact layout is likely to change - depending on exact features used - in the
/// future.  Specifically, on Android, since we're guaranteed to only have a single ambient VM, we can likely store the
/// *const JNIEnv in thread local storage instead of lugging it around in every Local.  Of course, there's no
/// guarantee that's actually an *optimization*...
pub type ArgumentRef<'env, Class> = Ref<'env, Class>;
