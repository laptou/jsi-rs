use std::{env, path::PathBuf};

fn main() {
    let base = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let base = PathBuf::from(base);

    let includes = vec![
        base.join("../../../react-native/node_modules/react-native"),
        base.join("../../../react-native/node_modules/react-native/React"),
        base.join("../../../react-native/node_modules/react-native/React/Base"),
        base.join("../../../react-native/node_modules/react-native/ReactCommon/jsi"),
        base.join("../../../react-native/node_modules/react-native/ReactCommon/callinvoker"),
        base.join("vendor/hermes/API"),
        base.join("vendor/hermes/public"),
        base.join("include"),
    ];

    for include in &includes {
        println!("cargo:rerun-if-changed={:?}", include);
    }

    // if let Some("android") = target_os {
    //     includes.push(
    //         base.join("../../../react-native/node_modules/react-native/ReactAndroid/src/main/java/com/facebook/react/turbomodule/core/jni")
    //     );
    //     includes.push(base.join("vendor/fbjni/cxx"));
    // }

    let includes: Vec<_> = IntoIterator::into_iter(includes)
        .map(|p| dunce::canonicalize(&p).expect(&format!("missing include path {:?}", p)))
        .collect();

    let compiles: Vec<PathBuf> = vec![];

    let compiles: Vec<_> = IntoIterator::into_iter(compiles)
        .map(|p| dunce::canonicalize(&p).expect(&format!("missing compile file {:?}", p)))
        .collect();

    cxx_build::CFG
        .exported_header_dirs
        .extend(includes.iter().map(|e| e.as_path()));

    let bridges = vec!["src/ffi.rs"];

    for bridge in &bridges {
        println!("cargo:rerun-if-changed={}", bridge);
    }

    cxx_build::bridges(bridges)
        .flag_if_supported("-std=c++17")
        .files(compiles)
        .compile("js-tests");

    println!("cargo:rustc-link-lib=hermes");
    println!("cargo:rustc-link-search={}", base.join("lib").to_string_lossy());
    println!("cargo:rustc-env=LD_LIBRARY_PATH={}", base.join("lib").to_string_lossy());
}
