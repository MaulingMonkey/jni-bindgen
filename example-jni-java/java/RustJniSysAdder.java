package com.maulingmonkey.jni_bindgen.example_jni_java;

public class RustJniSysAdder implements Adder {
    @Override public native String add(String a, String b);
    @Override public native float add(float a, float b);
    @Override public native int add(int a, int b);
}
