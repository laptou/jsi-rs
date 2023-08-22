use std::{pin::Pin, future::Future};

pub use jsi_sys as sys;
#[cfg(feature = "macros")]
pub use jsi_macros::host_object;

// allows us to use the proc macros inside this crate
extern crate self as jsi;

mod array;
mod array_buffer;
mod call_invoker;
mod function;
mod host_object;
mod host_function;
mod object;
mod prop_name;
mod runtime;
mod string;
mod symbol;
mod value;

pub use array::*;
pub use array_buffer::*;
pub use call_invoker::*;
pub use function::*;
pub use host_object::*;
pub use host_function::*;
pub use object::*;
pub use prop_name::*;
pub use runtime::*;
pub use string::*;
pub use symbol::*;
pub use value::*;

#[macro_export]
macro_rules! js_error {
    ($err: expr, $rt: expr) => {{
        let mut rt = $rt;
        let error_ctor = rt.global().get(PropName::new("Error", rt), rt);
        let error_ctor: ::jsi::JsiFn = error_ctor.try_into_js(rt).unwrap();
        let error_str = format!("{:?}", $err);
        let error_str = ::jsi::JsiValue::new_string(error_str.as_str(), rt);
        let error = error_ctor
            .call_as_constructor(::std::iter::once(error_str), rt)
            .expect("Error constructor threw an exception");
        let error: ::jsi::JsiObject = error.try_into_js(rt)
            .expect("Error constructor returned a non-object");
        error
    }};
}


pub type JsTaskCallback = Box<
    dyn (for<'a> FnOnce(
            &'a mut RuntimeHandle<'a>,
        ) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + 'a>>)
        + Send,
>;
