package com.maulingmonkey.jni_bindgen.example_jni_java;

public class RustJniGlueAdder implements Adder {
    @Override public        String add(String a, String b) { return a+b; } // XXX
    @Override public native float add(float a, float b);
    @Override public native int add(int a, int b);

    static void test() {
        System.loadLibrary("example_jni_java");
        Adder adder = new RustJniGlueAdder();
        assert adder.add("1", "2").equals("12");
        assert adder.add(1.0f, 2.0f) == 3.0f;
        assert adder.add(1, 2) == 3;
    }
}
