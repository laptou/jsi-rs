use std::{pin::Pin, sync::Arc};

use futures::Future;
use lazy_static::lazy_static;
use log::*;
use parking_lot::{const_rwlock, RwLock};
use jsi::*;
use tokio::{
    runtime,
    sync::{mpsc, watch, Mutex},
    task,
};

pub mod convert;
pub mod event;
mod impls;

extern crate self as splicer_js_api;

struct JsRuntimeInfo<'rt> {
    task_tx: mpsc::UnboundedSender<JsTaskCallback>,
    call_invoker: CallInvoker<'rt>,
}

static CURRENT_RUNTIME_INFO: RwLock<Option<JsRuntimeInfo>> = const_rwlock(None);

pub fn jsi_module_init(rt: *mut sys::Runtime, call_invoker: cxx::SharedPtr<sys::CallInvoker>) {
    let (task_tx, task_rx) = mpsc::unbounded_channel::<JsTaskCallback>();

    debug!("got JSI runtime pointer: {:p}", rt);
    debug!(
        "got JSI call invoker pointer: {:p}",
        call_invoker.as_ref().unwrap()
    );

    let runtime_handle = Arc::new(Mutex::new(RuntimeHandle::new_unchecked(rt)));
    let call_invoker = CallInvoker::new(call_invoker);

    std::thread::Builder::new()
        .name(format!("Splicer JS ({:p})", rt))
        .spawn(move || {
            let async_rt = runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();

            let task_set = task::LocalSet::new();

            async_rt.block_on(task_set.run_until(async move {
                let drop_rx = {
                    let rt = &mut *runtime_handle.lock().await;

                    let (obj, drop_rx) = impls::GlobalHostObject::new(runtime_handle.clone(), call_invoker.clone());
                    let obj = OwnedJsiUserHostObject::new(obj, rt);
                    let obj = obj.into_value(rt);

                    rt.global().set(PropName::new("splicer", rt), obj, rt);

                    drop_rx
                };

                CURRENT_RUNTIME_INFO.write().replace(JsRuntimeInfo {
                    call_invoker,
                    task_tx,
                });

                info!("jsi init done");

                loop {
                    tokio::select! {
                        _ = drop_rx => {
                            CURRENT_RUNTIME_INFO.write().take();
                            break;
                        },
                        task_fn = task_rx.recv() => {
                            if let Some(task_fn) = task_rx.recv().await {
                                let rt = &mut *runtime_handle.lock().await;
                                task_fn(rt).await.unwrap();
                            } else {
                                break;
                            }
                        }
                    }
                }
            }));
        })
        .unwrap();
}

pub fn spawn(cb: JsTaskCallback) {
    CURRENT_RUNTIME_INFO
        .read()
        .expect("no active js runtime")
        .task_tx
        .send(cb);
}

pub fn invoke(cb: CallInvokerCallback) {
    CURRENT_RUNTIME_INFO
        .read()
        .expect("no active js runtime")
        .call_invoker
        .invoke_async(cb);
}

pub enum JsiAsyncFnError<'a> {
    Rejection(jsi::JsiValue<'a>),
    Error(anyhow::Error),
}

impl<'a> From<jsi::JsiValue<'a>> for JsiAsyncFnError<'a> {
    fn from(v: jsi::JsiValue<'a>) -> Self {
        Self::Rejection(v)
    }
}

impl From<anyhow::Error> for JsiAsyncFnError<'_> {
    fn from(v: anyhow::Error) -> Self {
        Self::Error(v)
    }
}

impl From<ser::JsiSerializeError> for JsiAsyncFnError<'_> {
    fn from(v: ser::JsiSerializeError) -> Self {
        Self::Error(v.into())
    }
}

impl From<de::JsiDeserializeError> for JsiAsyncFnError<'_> {
    fn from(v: de::JsiDeserializeError) -> Self {
        match v {
            de::JsiDeserializeError::Native(v) => Self::Error(v.into()),
            de::JsiDeserializeError::Other(v) => Self::Error(v),
        }
    }
}

pub use convert::*;
pub use impls::*;
pub use splicer_js_api_macros::*;
