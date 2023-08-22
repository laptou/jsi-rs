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

extern crate self as splicer_js_api;

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
pub use splicer_js_api_macros::*;
