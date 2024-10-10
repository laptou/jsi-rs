use jsi::{
    host_object, FromObject, FromValue, IntoValue, JsiFn, JsiObject, JsiString, JsiValue, PropName, RuntimeHandle
};

#[cfg(target_os = "android")]
mod android;

#[cfg(target_os = "ios")]
mod ios;

pub fn init(rt: *mut jsi::sys::Runtime, call_invoker: cxx::SharedPtr<jsi::sys::CallInvoker>) {
    let (mut rt, _) = jsi::init(rt, call_invoker);

    let console = PropName::new("console", &mut rt);
    let console = rt.global().get(console, &mut rt);
    let console = JsiObject::from_value(&console, &mut rt).unwrap();

    let console_log = console.get(PropName::new("log", &mut rt), &mut rt);
    let console_log = JsiObject::from_value(&console_log, &mut rt).unwrap();
    let console_log = JsiFn::from_object(&console_log, &mut rt).unwrap();
    console_log
        .call(
            [JsiString::new("hello from Rust", &mut rt).into_value(&mut rt)],
            &mut rt,
        )
        .unwrap();

    // we called console.log("hello from Rust") using JSI! you should see the
    // log in your React Native bundler terminal

    // this is just an example, but from here, you could spawn threads or really
    // do whatever you want with the RuntimeHandle

    // make sure that any multithreaded operations use the CallInvoker if they
    // want to call back to JavaScript

    // now, for my next trick, I will add a host object to the global namespace
    let host_object = ExampleHostObject;
    let host_object = host_object.into_value(&mut rt);

    rt.global().set(PropName::new("ExampleGlobal", &mut rt), &host_object, &mut rt);

    let global_str = JsiString::new("hallo", &mut rt);
    let global_str = global_str.into_value(&mut rt);
    rt.global().set(PropName::new("ExampleGlobal2", &mut rt), &global_str, &mut rt);

    let global_num = JsiValue::new_number(3.200);
    rt.global().set(PropName::new("ExampleGlobal3", &mut rt), &global_num, &mut rt);
}

struct ExampleHostObject;

#[host_object]
impl ExampleHostObject {
    pub fn time(&self, _rt: &mut RuntimeHandle) -> anyhow::Result<i64> {
        Ok(3200)
    }
}
