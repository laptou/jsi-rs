use jsi::{FromObject, IntoObject, IntoValue, JsiFn, JsiObject, JsiString, JsiValueKind, PropName, FromValue};

#[cfg(target_os = "android")]
mod android;

pub fn init(rt: *mut jsi::sys::Runtime, call_invoker: cxx::SharedPtr<jsi::sys::CallInvoker>) {
    let (mut rt, call_invoker) = jsi::init(rt, call_invoker);

    let console = PropName::new("console", &mut rt);
    let console = rt.global().get(console, &mut rt);
    let console = JsiObject::from_value(&console, &mut rt).unwrap();

    let console_log = console.get(PropName::new("log", &mut rt), &mut rt);
    let console_log = JsiObject::from_value(&console_log, &mut rt).unwrap();
    let console_log = JsiFn::from_object(&console_log, &mut rt).unwrap();
    console_log.call(
        [JsiString::new("hello from Rust", &mut rt).into_value(&mut rt)],
        &mut rt,
    );

    // we called console.log("hello from Rust") using JSI! you should see the
    // log in your React Native bundler terminal

    // this is just an example, but from here, you could spawn threads or really
    // do whatever you want with the RuntimeHandle

    // make sure that any multithreaded operations use the CallInvoker if they
    // want to call back to JavaScript
}
