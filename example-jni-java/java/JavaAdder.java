package com.maulingmonkey.jni_bindgen.example_jni_java;

public class JavaAdder implements Adder {
    @Override public String add(String a, String b) { return a + b; }
    @Override public float add(float a, float b) { return a + b; }
    @Override public int add(int a, int b) { return a + b; }
}
