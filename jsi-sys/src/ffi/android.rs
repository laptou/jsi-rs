#[cxx::bridge]
pub mod ffi {
    #[namespace = "facebook::react"]
    unsafe extern "C++" {
        include!("ReactCommon/CallInvokerHolder.h");

        type CallInvoker = crate::ffi::base::CallInvoker;
        pub type CallInvokerHolder;

        pub fn getCallInvoker(self: Pin<&mut CallInvokerHolder>) -> SharedPtr<CallInvoker>;
    }
}

pub use ffi::CallInvokerHolder;
