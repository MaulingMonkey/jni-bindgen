package com.maulingmonkey.jni_bindgen.example_android_studio;

import androidx.appcompat.app.AppCompatActivity;

import android.os.Bundle;
import android.view.KeyEvent;

public class MainActivity extends AppCompatActivity {
    static {
        System.loadLibrary("example_android_studio");
    }

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);
    }

    @Override
    public native boolean dispatchKeyEvent(KeyEvent keyEvent);
}
