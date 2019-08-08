use super::*;

#[doc(hidden)] // You should generally not be interacting with this type directly, but it must be public for codegen.
/// By implementing this you assert that you're constructing a valid jvalue for the given argument type (e.g. valid
/// jobject pointer if the function is supposed to take a jobject)
pub unsafe trait AsJValue           { fn as_jvalue(&self) -> jvalue; }

unsafe impl AsJValue for bool       { fn as_jvalue(&self) -> jvalue { jvalue { z: if *self { JNI_TRUE } else { JNI_FALSE } } } }
unsafe impl AsJValue for jbyte      { fn as_jvalue(&self) -> jvalue { jvalue { b: *self } } }
unsafe impl AsJValue for jchar      { fn as_jvalue(&self) -> jvalue { jvalue { c: self.0 } } }
unsafe impl AsJValue for jshort     { fn as_jvalue(&self) -> jvalue { jvalue { s: *self } } }
unsafe impl AsJValue for jint       { fn as_jvalue(&self) -> jvalue { jvalue { i: *self } } }
unsafe impl AsJValue for jlong      { fn as_jvalue(&self) -> jvalue { jvalue { j: *self } } }
unsafe impl AsJValue for jfloat     { fn as_jvalue(&self) -> jvalue { jvalue { f: *self } } }
unsafe impl AsJValue for jdouble    { fn as_jvalue(&self) -> jvalue { jvalue { d: *self } } }
//unsafe impl AsJValue for jobject  { fn as_jvalue(&self) -> jvalue { jvalue { l: *self } } } // do NOT implement, no guarantee any given jobject is actually safe!

unsafe impl<T: AsValidJObjectAndEnv> AsJValue for Option<&T> {
    fn as_jvalue(&self) -> jvalue {
        match self {
            None => jvalue { l: null_mut() },
            Some(inner) => inner.as_jvalue(),
        }
    }
}
