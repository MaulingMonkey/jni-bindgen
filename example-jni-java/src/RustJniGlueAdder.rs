use jni_sys::{JNIEnv, jobject, jstring, jint, jfloat};

#[no_mangle] pub extern "stdcall" fn Java_com_maulingmonkey_jni_1bindgen_example_1jni_1java_RustJniGlueAdder_add__Ljava_lang_String_2Ljava_lang_String_2(_env: *mut JNIEnv, _this: jobject, a: jstring, _b: jstring) -> jstring {
    a // FIXME
}

#[no_mangle] pub extern "stdcall" fn Java_com_maulingmonkey_jni_1bindgen_example_1jni_1java_RustJniGlueAdder_add__FF(_env: *mut JNIEnv, _this: jobject, a: jfloat, b: jfloat) -> jfloat {
    a + b
}

#[no_mangle] pub extern "stdcall" fn Java_com_maulingmonkey_jni_1bindgen_example_1jni_1java_RustJniGlueAdder_add__II(_env: *mut JNIEnv, _this: jobject, a: jint, b: jint) -> jint {
    a + b
}

#[test] fn test() -> Result<(), jerk_test::JavaTestError> {
    jerk_test::run_test(
        "com.maulingmonkey.jni_bindgen.example_jni_java",
        "RustJniSysAdder",
        "test"
    )
}
