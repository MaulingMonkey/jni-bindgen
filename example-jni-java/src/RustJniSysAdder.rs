use jni_sys::{JNIEnv, jobject, jstring, jint, jfloat};
use std::ffi::CStr;
use std::ptr::null_mut;

#[no_mangle] pub extern "stdcall" fn Java_com_maulingmonkey_jni_1bindgen_example_1jni_1java_RustJniSysAdder_add__Ljava_lang_String_2Ljava_lang_String_2(env: *mut JNIEnv, _this: jobject, a: jstring, b: jstring) -> jstring {
    unsafe {
        let get_string_utf_chars    = (**env).GetStringUTFChars.unwrap();
        let new_string_utf          = (**env).NewStringUTF.unwrap();

        let astr = CStr::from_ptr(get_string_utf_chars(env, a, null_mut())).to_str().unwrap();
        let bstr = CStr::from_ptr(get_string_utf_chars(env, b, null_mut())).to_str().unwrap();
        let r = format!("{}{}\0", astr, bstr);
        new_string_utf(env, r.as_ptr() as *const _)
    }
}

#[no_mangle] pub extern "stdcall" fn Java_com_maulingmonkey_jni_1bindgen_example_1jni_1java_RustJniSysAdder_add__FF(_env: *mut JNIEnv, _this: jobject, a: jfloat, b: jfloat) -> jfloat {
    a + b
}

#[no_mangle] pub extern "stdcall" fn Java_com_maulingmonkey_jni_1bindgen_example_1jni_1java_RustJniSysAdder_add__II(_env: *mut JNIEnv, _this: jobject, a: jint, b: jint) -> jint {
    a + b
}

pub fn do_test() -> Result<(), jerk_test::JavaTestError> {
    jerk_test::run_test(
        "com.maulingmonkey.jni_bindgen.example_jni_java",
        "RustJniSysAdder",
        "test"
    )
}

#[test] fn test() -> Result<(), jerk_test::JavaTestError> { do_test() }
