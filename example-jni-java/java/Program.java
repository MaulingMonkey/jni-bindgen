package com.maulingmonkey.jni_bindgen.example_jni_java;

import com.maulingmonkey.jni_bindgen.example_jni_java.JavaAdder;
import com.maulingmonkey.jni_bindgen.example_jni_java.RustJniSysAdder;

public class Program {
    public static void main(String[] args) {
        System.loadLibrary("example_jni_java");

        System.out.println();
        System.out.println("example-jni-java:  Hello, world!");
        System.out.println();

        Adder[] adders = new Adder[]{
            new JavaAdder(),
            new RustJniSysAdder(),
            //new RustJniGlueAdder(), // FIXME
        };
        for (Adder adder : adders) {
            System.out.println(adder.getClass().getName());
            System.out.println(" 1  +  2  = " + adder.add( 1,    2  ));
            System.out.println("'1' + '2' = '"+ adder.add("1",  "2" ) + "'");
            System.out.println("1.0 + 2.0 = " + adder.add(1.0f, 2.0f));
            System.out.println();

            assert adder.add( 1,    2  ) == 3;
            assert adder.add("1",  "2" ).equals("12");
            assert adder.add(1.0f, 2.0f) == 3.0f;
        }
    }
}
