# An example React Native project using a `jsi-rs` module

## How to reproduce (Android)

1.  Create a blank React Native project: `npx react-native@latest init example`
2.  Create a blank Cargo project inside: `cd example && cargo init --lib example-jsi-module`
3.  Add `jsi`, `jni`, and `cxx` as dependencies of `example-jsi-module`
    - `jsi` will allow us to create a React Native module
    - `jni` is needed for interop with Java code on Android
    - `cxx` is needed to initialize `jsi` because JSI is implemented in C++ with smart pointers
4.  Write the code in  `example-jsi-module/src`
5.  Update `android/build.gradle` to add the following plugin under `buildscript.dependencies`:
    ```groovy
    buildscript {
        repositories {
          // don't remove the existing repositories, just add maven b/c the Rust plugin is hosted there
          maven {
            url "https://plugins.gradle.org/m2/"
          }
        }
        dependencies {
          // don't remove the existing dependencies, just add this one
          classpath("io.github.MatrixDev.android-rust:plugin:0.3.2")
        }
    }
    ```

    This plugin will compile `example-jsi-module` using the Android NDK as part
    of building our application for Android
6.  Make sure the [Android NDK](https://developer.android.com/ndk) is installed.
    Version 23 should work
7.  Update `android/app/build.gradle` to add the following lines:
    ```groovy
    apply plugin: "io.github.MatrixDev.android-rust"
    androidRust {
        module("example-jsi-module") {
            it.path = file("../example-jsi-module")

            // default abi targets are arm and arm64; if you want to run on Android
            // Emulator then you may need to add x86_64
            
            // targets = ["arm", "arm64", "x86", "x86_64"]
        }
    }
    ```
    
    **Note:** when you compile the program, this Gradle plugin will install all of
    the Rust Android targets if they are not already installed
8.  Write the code in
    `android/app/src/main/java/com/example/ExampleJsiModule.java`. This will be
    called when the application starts, and it gives us a pointer to the React
    Native runtime so we can initialize our Rust code
9.  Write the code in
    `android/app/src/main/java/com/example/ExampleJsiPackage.java`. This lets
    React Native discover our module
10. In `android/app/src/main/java/com/example/MainApplication.java`, add the following line to `getPackages()`:
    ```java
    packages.add(new ExampleJsiPackage());
    ```
11. Add a couple of lines to `App.tsx` to call our native module (I added them near the top):
    ```typescript
    // call our Rust module
    const {ExampleJsiModule} = NativeModules;
    ExampleJsiModule.install();
    ```
12. Connect your Android device or start an emulator
13. Run `npm run start` and press A to deploy to Android
14. You should see `hello from Rust` in the terminal after the app loads
