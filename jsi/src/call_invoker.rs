use std::marker::PhantomData;

pub use sys::CallInvokerCallback;

use crate::sys;

/// Used to run JavaScript functions in a given runtime from Rust. Required when
/// trying to call a JavaScript function from a thread other than the JS thread.
#[derive(Clone)]
pub struct CallInvoker<'rt>(
    pub(crate) cxx::SharedPtr<sys::CallInvoker>,
    pub(crate) PhantomData<&'rt ()>,
);

unsafe impl Send for CallInvoker<'_> {}
unsafe impl Sync for CallInvoker<'_> {}

impl<'rt> CallInvoker<'rt> {
    pub fn new(ptr: cxx::SharedPtr<sys::CallInvoker>) -> Self {
        CallInvoker(ptr, PhantomData)
    }

    /// WARNING: currently crashes with message "Synchronous native -> JS calls are currently not supported"
    pub fn invoke_sync(&self, job: CallInvokerCallback<'rt>) {
        #[cfg(feature = "call-invoker-trace")]
        log::trace!("call invoker sync call with closure at {:p}", job);

        unsafe {
            sys::CallInvoker_invokeSync(self.0.clone(), Box::into_raw(Box::new(job)) as *mut _)
        }
    }

    pub fn invoke_async(&self, job: CallInvokerCallback<'rt>) {
        #[cfg(feature = "call-invoker-trace")]
        log::trace!("call invoker async call with closure at {:p}", job);

        unsafe {
            sys::CallInvoker_invokeAsync(self.0.clone(), Box::into_raw(Box::new(job)) as *mut _)
        }
    }
}
