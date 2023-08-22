package com.example;

import android.util.Log;

import com.facebook.react.bridge.JSIModulePackage;
import com.facebook.react.bridge.JSIModuleProvider;
import com.facebook.react.bridge.JSIModuleSpec;
import com.facebook.react.bridge.JSIModuleType;
import com.facebook.react.bridge.JavaScriptContextHolder;
import com.facebook.react.bridge.ReactApplicationContext;
import com.facebook.react.turbomodule.core.CallInvokerHolderImpl;

import com.facebook.react.bridge.Arguments;
import com.facebook.react.bridge.Callback;
import com.facebook.react.bridge.JavaScriptContextHolder;
import com.facebook.react.bridge.ReactApplicationContext;
import com.facebook.react.bridge.ReactContextBaseJavaModule;
import com.facebook.react.bridge.ReactMethod;
import com.facebook.react.bridge.ReadableMap;
import com.facebook.react.bridge.WritableArray;
import com.facebook.react.bridge.WritableMap;
import com.facebook.react.module.annotations.ReactModule;

import java.util.Arrays;
import java.util.List;

@ReactModule(name = "ExampleJsiModule")
public class ExampleJsiModule extends ReactContextBaseJavaModule {
    public static native void init(long runtimePtr, CallInvokerHolderImpl callInvoker);

    public ExampleJsiModule(ReactApplicationContext reactContext) {
        super(reactContext);
    }

    @Override
    public String getName() {
        return "ExampleJsiModule";
    }

    @ReactMethod(isBlockingSynchronousMethod = true)
    public boolean install() {
        // load our dynamic library
        System.loadLibrary("example_jsi_module");

        var ctx = getReactApplicationContext();
        var jsContext = ctx.getJavaScriptContextHolder();
        var runtimePtr = jsContext.get();
        var callInvoker = (CallInvokerHolderImpl)ctx.getCatalystInstance().getJSCallInvokerHolder();

        Log.i("ExampleJsiModule", "initializing");

        if (jsContext.get() != 0) {
            ExampleJsiModule.init(runtimePtr, callInvoker);
            Log.i("ExampleJsiModule", "initialized");

            return true;
        } else {
            Log.e("ExampleJsiModule","initialization failed: no runtime");
            return false;
        }
    }
}
