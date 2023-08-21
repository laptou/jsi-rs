use cxx::UniquePtr;

use splicer_js_tests::ffi::bridge::*;
use jsi::RuntimeHandle;
use jsi_sys::*;

pub fn create_raw_runtime() -> UniquePtr<jsi_sys::Runtime> {
    let config = create_runtime_config();
    let rt = create_hermes_runtime(&*config);
    cast_hermes_runtime(rt)
}
