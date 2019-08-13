use super::*;
use std::os::raw::c_char;

/// FFI:  Use **&Env** instead of \*const JNIEnv.  This represents a per-thread Java exection environment.
/// 
/// A "safe" alternative to jni_sys::JNIEnv raw pointers, with the following caveats:
/// 
/// 1)  A null env will result in **undefined behavior**.  Java should not be invoking your native functions with a null
///     *mut JNIEnv, however, so I don't believe this is a problem in practice unless you've bindgened the C header
///     definitions elsewhere, calling them (requiring `unsafe`), and passing null pointers (generally UB for JNI
///     functions anyways, so can be seen as a caller soundness issue.)
/// 
/// 2)  Allowing the underlying JNIEnv to be modified is **undefined behavior**.  I don't believe the JNI libraries
///     modify the JNIEnv, so as long as you're not accepting a *mut JNIEnv elsewhere, using unsafe to dereference it,
///     and mucking with the methods on it yourself, I believe this "should" be fine.
/// 
/// # Example
/// 
/// ### MainActivity.java
/// 
/// ```java
/// package com.maulingmonkey.example;
/// 
/// public class MainActivity extends androidx.appcompat.app.AppCompatActivity {
///     @Override
///     public native boolean dispatchKeyEvent(android.view.KeyEvent keyEvent);
/// 
///     // ...
/// }
/// ```
/// 
/// ### main_activity.rs
/// 
/// ```rust
/// use jni_sys::{jboolean, jobject, JNI_TRUE}; // TODO: Replace with safer equivalent
/// use jni_glue::Env;
/// 
/// #[no_mangle] pub extern "system"
/// fn Java_com_maulingmonkey_example_MainActivity_dispatchKeyEvent<'env>(
///     _env:       &Env,
///     _this:      jobject, // TODO: Replace with safer equivalent
///     _key_event: jobject  // TODO: Replace with safer equivalent
/// ) -> jboolean {
///     // ...
///     JNI_TRUE
/// }
/// ```
#[repr(transparent)]
pub struct Env(JNIEnv);

impl Env {
    pub unsafe fn from_ptr<'env>(ptr: *const JNIEnv) -> &'env Env { &*(ptr as *const Env) }

    pub fn as_jni_env(&self) -> *mut JNIEnv { &self.0 as *const _ as *mut _ }
    pub(crate) unsafe fn from_jni_local(env: &JNIEnv) -> &Env { &*(env as *const JNIEnv as *const Env) }
    pub(crate) unsafe fn from_jni_void_ref(ptr: &*mut c_void) -> &Env { Self::from_jni_local(&*(*ptr as *const c_void as *const JNIEnv)) }

    pub(crate) fn get_gen_vm(&self) -> GenVM {
        let jni_env = self.as_jni_env();
        let mut vm = null_mut();
        let err = unsafe { (**jni_env).GetJavaVM.unwrap()(jni_env, &mut vm) };
        assert_eq!(err, JNI_OK);
        assert_ne!(vm, null_mut());
        VMS.read().unwrap().get_gen_vm(vm)
    }

    // Query Methods

    pub unsafe fn require_class(&self, class: &str) -> jclass {
        debug_assert!(class.ends_with('\0'));
        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        let class = (**env).FindClass.unwrap()(env, class.as_ptr() as *const c_char);
        assert!(!class.is_null());
        class
    }

    pub unsafe fn require_method(&self, class: jclass, method: &str, descriptor: &str) -> jmethodID {
        debug_assert!(method.ends_with('\0'));
        debug_assert!(descriptor.ends_with('\0'));

        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        let method = (**env).GetMethodID.unwrap()(env, class, method.as_ptr() as *const c_char, descriptor.as_ptr() as *const c_char);
        assert!(!method.is_null());
        method
    }

    pub unsafe fn require_static_method(&self, class: jclass, method: &str, descriptor: &str) -> jmethodID {
        debug_assert!(method.ends_with('\0'));
        debug_assert!(descriptor.ends_with('\0'));

        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        let method = (**env).GetStaticMethodID.unwrap()(env, class, method.as_ptr() as *const c_char, descriptor.as_ptr() as *const c_char);
        assert!(!method.is_null());
        method
    }

    pub unsafe fn require_field(&self, class: jclass, field: &str, descriptor: &str) -> jfieldID {
        debug_assert!(field.ends_with('\0'));
        debug_assert!(field.ends_with('\0'));

        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        let field = (**env).GetFieldID.unwrap()(env, class, field.as_ptr() as *const c_char, descriptor.as_ptr() as *const c_char);
        assert!(!field.is_null());
        field
    }

    pub unsafe fn require_static_field(&self, class: jclass, field: &str, descriptor: &str) -> jfieldID {
        debug_assert!(field.ends_with('\0'));
        debug_assert!(field.ends_with('\0'));

        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        let field = (**env).GetStaticFieldID.unwrap()(env, class, field.as_ptr() as *const c_char, descriptor.as_ptr() as *const c_char);
        assert!(!field.is_null());
        field
    }

    // Multi-Query Methods

    pub unsafe fn require_class_method(&self, class: &str, method: &str, descriptor: &str) -> (jclass, jmethodID) {
        let class = self.require_class(class);
        (class, self.require_method(class, method, descriptor))
    }

    pub unsafe fn require_class_static_method(&self, class: &str, method: &str, descriptor: &str) -> (jclass, jmethodID) {
        let class = self.require_class(class);
        (class, self.require_static_method(class, method, descriptor))
    }

    pub unsafe fn require_class_field(&self, class: &str, method: &str, descriptor: &str) -> (jclass, jfieldID) {
        let class = self.require_class(class);
        (class, self.require_field(class, method, descriptor))
    }

    pub unsafe fn require_class_static_field(&self, class: &str, method: &str, descriptor: &str) -> (jclass, jfieldID) {
        let class = self.require_class(class);
        (class, self.require_static_field(class, method, descriptor))
    }

    // Constructor Methods

    pub unsafe fn new_object_a<'env, T: AsValidJObjectAndEnv>(&'env self, class: jclass, method: jmethodID, args: *const jvalue) -> Result<Local<'env, T>> {
        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        let result = (**env).NewObjectA.unwrap()(env, class, method, args);
        let exception = (**env).ExceptionOccurred.unwrap()(env);
        if !exception.is_null() {
            (**env).ExceptionClear.unwrap()(env);
            Err(exception)
        } else {
            assert!(!result.is_null());
            Ok(Local::from_env_object(env, result))
        }
    }

    // Instance Methods

    pub unsafe fn call_object_method_a<'env, T: AsValidJObjectAndEnv>(&'env self, this: jobject, method: jmethodID, args: *const jvalue) -> Result<Option<Local<'env, T>>> {
        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        let result = (**env).CallObjectMethodA.unwrap()(env, this, method, args);
        let exception = (**env).ExceptionOccurred.unwrap()(env);
        if !exception.is_null() {
            (**env).ExceptionClear.unwrap()(env);
            Err(exception)
        } else if result.is_null() {
            Ok(None)
        } else {
            Ok(Some(Local::from_env_object(env, result)))
        }
    }

    pub unsafe fn call_boolean_method_a(&self, this: jobject, method: jmethodID, args: *const jvalue) -> Result<bool> {
        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        let result = (**env).CallBooleanMethodA.unwrap()(env, this, method, args);
        let exception = (**env).ExceptionOccurred.unwrap()(env);
        if !exception.is_null() {
            (**env).ExceptionClear.unwrap()(env);
            Err(exception)
        } else {
            Ok(result != JNI_FALSE)
        }
    }

    pub unsafe fn call_byte_method_a(&self, this: jobject, method: jmethodID, args: *const jvalue) -> Result<jbyte> {
        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        let result = (**env).CallByteMethodA.unwrap()(env, this, method, args);
        let exception = (**env).ExceptionOccurred.unwrap()(env);
        if !exception.is_null() {
            (**env).ExceptionClear.unwrap()(env);
            Err(exception)
        } else {
            Ok(result)
        }
    }

    pub unsafe fn call_char_method_a(&self, this: jobject, method: jmethodID, args: *const jvalue) -> Result<jchar> {
        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        let result = (**env).CallCharMethodA.unwrap()(env, this, method, args);
        let exception = (**env).ExceptionOccurred.unwrap()(env);
        if !exception.is_null() {
            (**env).ExceptionClear.unwrap()(env);
            Err(exception)
        } else {
            Ok(jchar(result))
        }
    }

    pub unsafe fn call_short_method_a(&self, this: jobject, method: jmethodID, args: *const jvalue) -> Result<jshort> {
        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        let result = (**env).CallShortMethodA.unwrap()(env, this, method, args);
        let exception = (**env).ExceptionOccurred.unwrap()(env);
        if !exception.is_null() {
            (**env).ExceptionClear.unwrap()(env);
            Err(exception)
        } else {
            Ok(result)
        }
    }

    pub unsafe fn call_int_method_a(&self, this: jobject, method: jmethodID, args: *const jvalue) -> Result<jint> {
        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        let result = (**env).CallIntMethodA.unwrap()(env, this, method, args);
        let exception = (**env).ExceptionOccurred.unwrap()(env);
        if !exception.is_null() {
            (**env).ExceptionClear.unwrap()(env);
            Err(exception)
        } else {
            Ok(result)
        }
    }

    pub unsafe fn call_long_method_a(&self, this: jobject, method: jmethodID, args: *const jvalue) -> Result<jlong> {
        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        let result = (**env).CallLongMethodA.unwrap()(env, this, method, args);
        let exception = (**env).ExceptionOccurred.unwrap()(env);
        if !exception.is_null() {
            (**env).ExceptionClear.unwrap()(env);
            Err(exception)
        } else {
            Ok(result)
        }
    }

    pub unsafe fn call_float_method_a(&self, this: jobject, method: jmethodID, args: *const jvalue) -> Result<jfloat> {
        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        let result = (**env).CallFloatMethodA.unwrap()(env, this, method, args);
        let exception = (**env).ExceptionOccurred.unwrap()(env);
        if !exception.is_null() {
            (**env).ExceptionClear.unwrap()(env);
            Err(exception)
        } else {
            Ok(result)
        }
    }

    pub unsafe fn call_double_method_a(&self, this: jobject, method: jmethodID, args: *const jvalue) -> Result<jdouble> {
        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        let result = (**env).CallDoubleMethodA.unwrap()(env, this, method, args);
        let exception = (**env).ExceptionOccurred.unwrap()(env);
        if !exception.is_null() {
            (**env).ExceptionClear.unwrap()(env);
            Err(exception)
        } else {
            Ok(result)
        }
    }

    pub unsafe fn call_void_method_a(&self, this: jobject, method: jmethodID, args: *const jvalue) -> Result<()> {
        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        let result = (**env).CallVoidMethodA.unwrap()(env, this, method, args);
        let exception = (**env).ExceptionOccurred.unwrap()(env);
        if !exception.is_null() {
            (**env).ExceptionClear.unwrap()(env);
            Err(exception)
        } else {
            Ok(result)
        }
    }

    // Static Methods

    pub unsafe fn call_static_object_method_a<'env, T: AsValidJObjectAndEnv>(&'env self, class: jclass, method: jmethodID, args: *const jvalue) -> Result<Option<Local<'env, T>>> {
        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        let result = (**env).CallStaticObjectMethodA.unwrap()(env, class, method, args);
        let exception = (**env).ExceptionOccurred.unwrap()(env);
        if !exception.is_null() {
            (**env).ExceptionClear.unwrap()(env);
            Err(exception)
        } else if result.is_null() {
            Ok(None)
        } else {
            Ok(Some(Local::from_env_object(env, result)))
        }
    }

    pub unsafe fn call_static_boolean_method_a(&self, class: jclass, method: jmethodID, args: *const jvalue) -> Result<bool> {
        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        let result = (**env).CallStaticBooleanMethodA.unwrap()(env, class, method, args);
        let exception = (**env).ExceptionOccurred.unwrap()(env);
        if !exception.is_null() {
            (**env).ExceptionClear.unwrap()(env);
            Err(exception)
        } else {
            Ok(result != JNI_FALSE)
        }
    }

    pub unsafe fn call_static_byte_method_a(&self, class: jclass, method: jmethodID, args: *const jvalue) -> Result<jbyte> {
        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        let result = (**env).CallStaticByteMethodA.unwrap()(env, class, method, args);
        let exception = (**env).ExceptionOccurred.unwrap()(env);
        if !exception.is_null() {
            (**env).ExceptionClear.unwrap()(env);
            Err(exception)
        } else {
            Ok(result)
        }
    }

    pub unsafe fn call_static_char_method_a(&self, class: jclass, method: jmethodID, args: *const jvalue) -> Result<jchar> {
        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        let result = (**env).CallStaticCharMethodA.unwrap()(env, class, method, args);
        let exception = (**env).ExceptionOccurred.unwrap()(env);
        if !exception.is_null() {
            (**env).ExceptionClear.unwrap()(env);
            Err(exception)
        } else {
            Ok(jchar(result))
        }
    }

    pub unsafe fn call_static_short_method_a(&self, class: jclass, method: jmethodID, args: *const jvalue) -> Result<jshort> {
        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        let result = (**env).CallStaticShortMethodA.unwrap()(env, class, method, args);
        let exception = (**env).ExceptionOccurred.unwrap()(env);
        if !exception.is_null() {
            (**env).ExceptionClear.unwrap()(env);
            Err(exception)
        } else {
            Ok(result)
        }
    }

    pub unsafe fn call_static_int_method_a(&self, class: jclass, method: jmethodID, args: *const jvalue) -> Result<jint> {
        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        let result = (**env).CallStaticIntMethodA.unwrap()(env, class, method, args);
        let exception = (**env).ExceptionOccurred.unwrap()(env);
        if !exception.is_null() {
            (**env).ExceptionClear.unwrap()(env);
            Err(exception)
        } else {
            Ok(result)
        }
    }

    pub unsafe fn call_static_long_method_a(&self, class: jclass, method: jmethodID, args: *const jvalue) -> Result<jlong> {
        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        let result = (**env).CallStaticLongMethodA.unwrap()(env, class, method, args);
        let exception = (**env).ExceptionOccurred.unwrap()(env);
        if !exception.is_null() {
            (**env).ExceptionClear.unwrap()(env);
            Err(exception)
        } else {
            Ok(result)
        }
    }

    pub unsafe fn call_static_float_method_a(&self, class: jclass, method: jmethodID, args: *const jvalue) -> Result<jfloat> {
        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        let result = (**env).CallStaticFloatMethodA.unwrap()(env, class, method, args);
        let exception = (**env).ExceptionOccurred.unwrap()(env);
        if !exception.is_null() {
            (**env).ExceptionClear.unwrap()(env);
            Err(exception)
        } else {
            Ok(result)
        }
    }

    pub unsafe fn call_static_double_method_a(&self, class: jclass, method: jmethodID, args: *const jvalue) -> Result<jdouble> {
        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        let result = (**env).CallStaticDoubleMethodA.unwrap()(env, class, method, args);
        let exception = (**env).ExceptionOccurred.unwrap()(env);
        if !exception.is_null() {
            (**env).ExceptionClear.unwrap()(env);
            Err(exception)
        } else {
            Ok(result)
        }
    }

    pub unsafe fn call_static_void_method_a(&self, class: jclass, method: jmethodID, args: *const jvalue) -> Result<()> {
        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        let result = (**env).CallStaticVoidMethodA.unwrap()(env, class, method, args);
        let exception = (**env).ExceptionOccurred.unwrap()(env);
        if !exception.is_null() {
            (**env).ExceptionClear.unwrap()(env);
            Err(exception)
        } else {
            Ok(result)
        }
    }

    // Instance Fields

    pub unsafe fn get_object_field<'env, T: AsValidJObjectAndEnv>(&'env self, this: jobject, field: jfieldID) -> Option<Local<'env, T>> {
        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        let result = (**env).GetObjectField.unwrap()(env, this, field);
        if result.is_null() {
            None
        } else {
            Some(Local::from_env_object(env, result))
        }
    }

    pub unsafe fn get_boolean_field(&self, this: jobject, field: jfieldID) -> bool {
        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        let result = (**env).GetBooleanField.unwrap()(env, this, field);
        result != JNI_FALSE
    }

    pub unsafe fn get_byte_field(&self, this: jobject, field: jfieldID) -> jbyte {
        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        let result = (**env).GetByteField.unwrap()(env, this, field);
        result
    }

    pub unsafe fn get_char_field(&self, this: jobject, field: jfieldID) -> jchar {
        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        let result = (**env).GetCharField.unwrap()(env, this, field);
        jchar(result)
    }

    pub unsafe fn get_short_field(&self, this: jobject, field: jfieldID) -> jshort {
        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        let result = (**env).GetShortField.unwrap()(env, this, field);
        result
    }

    pub unsafe fn get_int_field(&self, this: jobject, field: jfieldID) -> jint {
        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        let result = (**env).GetIntField.unwrap()(env, this, field);
        result
    }

    pub unsafe fn get_long_field(&self, this: jobject, field: jfieldID) -> jlong {
        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        let result = (**env).GetLongField.unwrap()(env, this, field);
        result
    }

    pub unsafe fn get_float_field(&self, this: jobject, field: jfieldID) -> jfloat {
        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        let result = (**env).GetFloatField.unwrap()(env, this, field);
        result
    }

    pub unsafe fn get_double_field(&self, this: jobject, field: jfieldID) -> jdouble {
        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        let result = (**env).GetDoubleField.unwrap()(env, this, field);
        result
    }

    pub unsafe fn set_object_field<'env, T: AsValidJObjectAndEnv>(&'env self, this: jobject, field: jfieldID, value: jobject) {
        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        (**env).SetObjectField.unwrap()(env, this, field, value);
    }

    pub unsafe fn set_boolean_field(&self, this: jobject, field: jfieldID, value: bool) {
        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        (**env).SetBooleanField.unwrap()(env, this, field, if value { JNI_TRUE } else { JNI_FALSE });
    }

    pub unsafe fn set_byte_field(&self, this: jobject, field: jfieldID, value: jbyte) {
        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        (**env).SetByteField.unwrap()(env, this, field, value);
    }

    pub unsafe fn set_char_field(&self, this: jobject, field: jfieldID, value: jchar) {
        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        (**env).SetCharField.unwrap()(env, this, field, value.0);
    }

    pub unsafe fn set_short_field(&self, this: jobject, field: jfieldID, value: jshort) {
        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        (**env).SetShortField.unwrap()(env, this, field, value);
    }

    pub unsafe fn set_int_field(&self, this: jobject, field: jfieldID, value: jint) {
        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        (**env).SetIntField.unwrap()(env, this, field, value);
    }

    pub unsafe fn set_long_field(&self, this: jobject, field: jfieldID, value: jlong) {
        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        (**env).SetLongField.unwrap()(env, this, field, value);
    }

    pub unsafe fn set_float_field(&self, this: jobject, field: jfieldID, value: jfloat) {
        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        (**env).SetFloatField.unwrap()(env, this, field, value);
    }

    pub unsafe fn set_double_field(&self, this: jobject, field: jfieldID, value: jdouble) {
        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        (**env).SetDoubleField.unwrap()(env, this, field, value);
    }

    // Static Fields

    pub unsafe fn get_static_object_field<'env, T: AsValidJObjectAndEnv>(&'env self, class: jclass, field: jfieldID) -> Option<Local<'env, T>> {
        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        let result = (**env).GetStaticObjectField.unwrap()(env, class, field);
        if result.is_null() {
            None
        } else {
            Some(Local::from_env_object(env, result))
        }
    }

    pub unsafe fn get_static_boolean_field(&self, class: jclass, field: jfieldID) -> bool {
        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        let result = (**env).GetStaticBooleanField.unwrap()(env, class, field);
        result != JNI_FALSE
    }

    pub unsafe fn get_static_byte_field(&self, class: jclass, field: jfieldID) -> jbyte {
        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        let result = (**env).GetStaticByteField.unwrap()(env, class, field);
        result
    }

    pub unsafe fn get_static_char_field(&self, class: jclass, field: jfieldID) -> jchar {
        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        let result = (**env).GetStaticCharField.unwrap()(env, class, field);
        jchar(result)
    }

    pub unsafe fn get_static_short_field(&self, class: jclass, field: jfieldID) -> jshort {
        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        let result = (**env).GetStaticShortField.unwrap()(env, class, field);
        result
    }

    pub unsafe fn get_static_int_field(&self, class: jclass, field: jfieldID) -> jint {
        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        let result = (**env).GetStaticIntField.unwrap()(env, class, field);
        result
    }

    pub unsafe fn get_static_long_field(&self, class: jclass, field: jfieldID) -> jlong {
        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        let result = (**env).GetStaticLongField.unwrap()(env, class, field);
        result
    }

    pub unsafe fn get_static_float_field(&self, class: jclass, field: jfieldID) -> jfloat {
        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        let result = (**env).GetStaticFloatField.unwrap()(env, class, field);
        result
    }

    pub unsafe fn get_static_double_field(&self, class: jclass, field: jfieldID) -> jdouble {
        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        let result = (**env).GetStaticDoubleField.unwrap()(env, class, field);
        result
    }

    pub unsafe fn set_static_object_field<'env, T: AsValidJObjectAndEnv>(&'env self, class: jclass, field: jfieldID, value: jobject) {
        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        (**env).SetStaticObjectField.unwrap()(env, class, field, value);
    }

    pub unsafe fn set_static_boolean_field(&self, class: jclass, field: jfieldID, value: bool) {
        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        (**env).SetStaticBooleanField.unwrap()(env, class, field, if value { JNI_TRUE } else { JNI_FALSE });
    }

    pub unsafe fn set_static_byte_field(&self, class: jclass, field: jfieldID, value: jbyte) {
        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        (**env).SetStaticByteField.unwrap()(env, class, field, value);
    }

    pub unsafe fn set_static_char_field(&self, class: jclass, field: jfieldID, value: jchar) {
        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        (**env).SetStaticCharField.unwrap()(env, class, field, value.0);
    }

    pub unsafe fn set_static_short_field(&self, class: jclass, field: jfieldID, value: jshort) {
        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        (**env).SetStaticShortField.unwrap()(env, class, field, value);
    }

    pub unsafe fn set_static_int_field(&self, class: jclass, field: jfieldID, value: jint) {
        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        (**env).SetStaticIntField.unwrap()(env, class, field, value);
    }

    pub unsafe fn set_static_long_field(&self, class: jclass, field: jfieldID, value: jlong) {
        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        (**env).SetStaticLongField.unwrap()(env, class, field, value);
    }

    pub unsafe fn set_static_float_field(&self, class: jclass, field: jfieldID, value: jfloat) {
        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        (**env).SetStaticFloatField.unwrap()(env, class, field, value);
    }

    pub unsafe fn set_static_double_field(&self, class: jclass, field: jfieldID, value: jdouble) {
        let env = &self.0 as *const JNIEnv as *mut JNIEnv;
        (**env).SetStaticDoubleField.unwrap()(env, class, field, value);
    }
}
