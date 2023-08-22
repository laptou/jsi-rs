use cxx::UniquePtr;

use jsi_tests::ffi::bridge::*;

pub fn create_raw_runtime() -> UniquePtr<jsi_sys::Runtime> {
    let config = create_runtime_config();
    let rt = create_hermes_runtime(&*config);
    cast_hermes_runtime(rt)
}
