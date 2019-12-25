package com.maulingmonkey.jni_bindgen.example_jni_java;

public class RustJniSysAdder implements Adder {
    @Override public native String add(String a, String b);
    @Override public native float add(float a, float b);
    @Override public native int add(int a, int b);

    static void test() {
        System.loadLibrary("example_jni_java");
        Adder adder = new RustJniSysAdder();
        assert adder.add("1", "2").equals("12");
        assert adder.add(1.0f, 2.0f) == 3.0f;
        assert adder.add(1, 2) == 3;
    }
}