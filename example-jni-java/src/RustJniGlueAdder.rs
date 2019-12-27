jni_glue::java! {
    import com.maulingmonkey.jni_bindgen.example_jni_java.RustJniGlueAdder;
    import com.maulingmonkey.jni_bindgen.example_jni_java.RustJniGlueAdder$Inner;
    import java.lang.String;

    unsafe impl class RustJniGlueAdder {
        float add(&env, this, float a, float b) {
            let r = a + b;
            Ok(r)
        }

        int add(&env, this, int a, int b) {
            let r = a + b;
            Ok(r)
        }

        // jni_sys doesn't yet exist, and I haven't finished implementing object conversion logic
        //String add(&env, this, String a, String b) {
        //    let r = format!("{}{}", a, b);
        //    println!("{} + {} = {}", a, b, r);
        //    Ok(r)
        //}
    }
}

#[test] fn test() -> Result<(), jerk_test::JavaTestError> {
    jerk_test::run_test(
        "com.maulingmonkey.jni_bindgen.example_jni_java",
        "RustJniGlueAdder",
        "test"
    )
}
