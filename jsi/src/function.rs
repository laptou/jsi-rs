use crate::host_function::UserHostFunction;
use crate::{sys, IntoValue, JsiObject, JsiValue, PropName, RuntimeHandle};
use anyhow::{bail, Context};
use std::marker::PhantomData;
use std::pin::Pin;

/// A JavaScript function.
pub struct JsiFn<'rt>(
    pub(crate) cxx::UniquePtr<sys::JsiFunction>,
    pub(crate) PhantomData<&'rt mut ()>,
);

impl<'rt> JsiFn<'rt> {
    pub fn call<T: IntoIterator<Item = JsiValue<'rt>>>(
        &self,
        args: T,
        rt: &mut RuntimeHandle<'rt>,
    ) -> Result<JsiValue<'rt>, cxx::Exception> {
        let mut args_cxx = sys::create_value_vector();
        for arg in args {
            sys::push_value_vector(args_cxx.pin_mut(), arg.0);
        }

        Ok(JsiValue(
            sys::Function_call(
                self.0.as_ref().unwrap(),
                rt.get_inner_mut(),
                args_cxx.as_ref().unwrap(),
            )?,
            PhantomData,
        ))
    }

    pub fn call_as_constructor<T: IntoIterator<Item = JsiValue<'rt>>>(
        &self,
        args: T,
        rt: &mut RuntimeHandle<'rt>,
    ) -> Result<JsiValue<'rt>, cxx::Exception> {
        let mut args_cxx = sys::create_value_vector();
        for arg in args {
            sys::push_value_vector(args_cxx.pin_mut(), arg.0);
        }

        Ok(JsiValue(
            sys::Function_callAsConstructor(
                self.0.as_ref().unwrap(),
                rt.get_inner_mut(),
                args_cxx.as_ref().unwrap(),
            )?,
            PhantomData,
        ))
    }

    pub fn call_with_this<'a, 'ret, T: IntoIterator<Item = JsiValue<'a>>>(
        &self,
        this: &JsiObject,
        args: T,
        rt: &mut RuntimeHandle<'rt>,
    ) -> Result<JsiValue<'ret>, cxx::Exception>
    where
        'rt: 'a,
        'rt: 'ret,
    {
        let mut args_cxx = sys::create_value_vector();
        for arg in args {
            sys::push_value_vector(args_cxx.pin_mut(), arg.0);
        }

        Ok(JsiValue(
            sys::Function_callWithThis(
                self.0.as_ref().unwrap(),
                rt.get_inner_mut(),
                this.0.as_ref().unwrap(),
                args_cxx.as_ref().unwrap(),
            )?,
            PhantomData,
        ))
    }

    pub fn from_host_fn(
        name: &PropName,
        param_count: usize,
        mut body: Box<UserHostFunction<'rt>>,
        rt: &mut RuntimeHandle<'rt>,
    ) -> Self {
        #[cfg(feature = "host-fn-trace")]
        log::trace!(
            "creating host fn {} with closure at {:p}",
            rt.display(name),
            body
        );

        let cb: sys::HostFunctionCallback =
            Box::new(move |rt: Pin<&mut sys::Runtime>, this, args| {
                let mut rt: RuntimeHandle =
                    unsafe { RuntimeHandle::new_unchecked(rt.get_unchecked_mut() as *mut _) };
                let this = JsiValue(sys::Value_copy(this, rt.get_inner_mut()), PhantomData);
                let args = args
                    .into_iter()
                    .map(|arg| JsiValue(sys::Value_copy(arg, rt.get_inner_mut()), PhantomData))
                    .collect();

                #[cfg(feature = "host-fn-trace")]
                log::trace!("host fn call with closure at {:p}", body);

                // log::trace!(
                //     "host fn call with closure at {:p} (this = {:?}, args = {:?})",
                //     body.as_ref(),
                //     this,
                //     args
                // );

                let val = body(this, args, &mut rt)?;
                Ok(val.0)
            });

        let cb = Box::into_raw(Box::new(cb)) as *mut _;

        JsiFn(
            unsafe {
                sys::Function_createFromHostFunction(
                    rt.get_inner_mut(),
                    name.0.as_ref().unwrap(),
                    param_count as u32,
                    cb,
                )
            },
            PhantomData,
        )
    }
}

unsafe impl<'rt> Send for JsiFn<'rt> {}

pub fn create_promise<
    'rt,
    F: 'rt + FnOnce(JsiFn<'rt>, JsiFn<'rt>, &mut RuntimeHandle<'rt>) -> (),
>(
    body: F,
    rt: &mut RuntimeHandle<'rt>,
) -> JsiObject<'rt> {
    let mut inner = Some(body);

    let body = JsiFn::from_host_fn(
        &PropName::new("_", rt),
        2,
        Box::new(move |_this, mut args, rt| {
            if args.len() != 2 {
                bail!(
                    "promise callback called with {} args, expected 2",
                    args.len()
                );
            }

            let resolve: JsiFn = args
                .remove(0)
                .try_into_js(rt)
                .context("promise resolver is not a function")?;

            let reject: JsiFn = args
                .remove(0)
                .try_into_js(rt)
                .context("promise rejecter is not a function")?;

            match inner.take() {
                Some(inner) => inner(resolve, reject, rt),
                None => anyhow::bail!("promise lambda is only supposed to be called once!"),
            }

            Ok(JsiValue::new_undefined())
        }),
        rt,
    );

    let ctor = rt.global().get(PropName::new("Promise", rt), rt);
    let ctor: JsiFn = ctor
        .try_into_js(rt)
        .expect("Promise constructor is not an object");

    let promise = ctor
        .call_as_constructor(vec![body.into_value(rt)], rt)
        .expect("Promise constructor threw an exception");

    promise
        .try_into_js(rt)
        .expect("Promise constructor did not return an object")
}
