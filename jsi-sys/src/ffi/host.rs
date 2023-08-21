use crate::shim::{rho_get, rho_properties, rho_set, RustHostObject};

#[cxx::bridge]
pub(crate) mod ffi {
    #[namespace = "jsi_rs::ffi"]
    unsafe extern "C++" {
        include!("host.h");

        #[namespace = "facebook::jsi"]
        pub type HostObject = crate::ffi::base::HostObject;
        #[namespace = "facebook::jsi"]
        pub type Runtime = crate::ffi::base::Runtime;
        #[cxx_name = "Value"]
        #[namespace = "facebook::jsi"]
        pub type JsiValue = crate::ffi::base::JsiValue;
        #[namespace = "facebook::jsi"]
        pub type PropNameID = crate::ffi::base::PropNameID;

        pub type CxxHostObject;
        pub fn CxxHostObject_create(rho: Box<RustHostObject<'_>>) -> UniquePtr<CxxHostObject>;

        pub fn CxxHostObject_toHostObjectU(ptr: UniquePtr<CxxHostObject>) -> UniquePtr<HostObject>;
        pub fn CxxHostObject_fromHostObjectU(
            ptr: UniquePtr<HostObject>,
        ) -> UniquePtr<CxxHostObject>;

        pub fn CxxHostObject_toHostObjectS(ptr: SharedPtr<CxxHostObject>) -> SharedPtr<HostObject>;
        pub fn CxxHostObject_fromHostObjectS(
            ptr: SharedPtr<HostObject>,
        ) -> SharedPtr<CxxHostObject>;

        pub fn CxxHostObject_getInner(ptr: &CxxHostObject) -> &RustHostObject;
        pub fn CxxHostObject_getInnerMut(ptr: Pin<&mut CxxHostObject>) -> &mut RustHostObject;
    }

    #[namespace = "jsi_rs::ffi"]
    extern "Rust" {
        type RustHostObject<'a>;

        unsafe fn rho_get<'a>(
            _self: &mut RustHostObject<'a>,
            rt: Pin<&mut Runtime>,
            name: &PropNameID,
        ) -> Result<UniquePtr<JsiValue>>;
        unsafe fn rho_set<'a>(
            _self: &mut RustHostObject<'a>,
            rt: Pin<&mut Runtime>,
            name: &PropNameID,
            value: &JsiValue,
        ) -> Result<()>;
        unsafe fn rho_properties<'a>(
            _self: &mut RustHostObject<'a>,
            rt: Pin<&mut Runtime>,
        ) -> UniquePtr<CxxVector<PropNameID>>;
    }
}

pub use ffi::*;
