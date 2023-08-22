use cxx::UniquePtr;

use jsi_tests::ffi::bridge::*;

#[test]
fn create_runtime() {
    let config = create_runtime_config();
    let rt = create_hermes_runtime(&*config);
    let mut rt: UniquePtr<jsi_sys::Runtime> = cast_hermes_runtime(rt);
    let rt = rt.as_mut().unwrap();
    assert_eq!("HermesRuntime", rt.description().to_string());
}

#[test]
fn evaluate_exprs() {
    let config = create_runtime_config();
    let rt = create_hermes_runtime(&*config);
    let mut rt: UniquePtr<jsi_sys::Runtime> = cast_hermes_runtime(rt);

    let out = eval_js(rt.pin_mut(), "1 + 1");
    let out = out.get_number().unwrap();
    assert_eq!(2., out);

    let out = eval_js(rt.pin_mut(), "'hello'");
    let out = out.as_string(rt.pin_mut()).unwrap();
    assert_eq!("hello", out.to_string(rt.pin_mut()).to_string());
}
