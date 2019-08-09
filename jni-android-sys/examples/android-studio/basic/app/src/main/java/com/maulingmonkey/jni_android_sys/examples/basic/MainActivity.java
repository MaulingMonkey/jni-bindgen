package com.maulingmonkey.jni_android_sys.examples.basic;

import androidx.appcompat.app.AppCompatActivity;

import android.os.Bundle;
import android.view.KeyEvent;

public class MainActivity extends AppCompatActivity {
    static {
        System.loadLibrary("basic");
    }

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);
    }

    @Override
    public native boolean dispatchKeyEvent(KeyEvent keyEvent);
}
