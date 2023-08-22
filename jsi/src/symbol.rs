use std::marker::PhantomData;

use crate::{sys, RuntimeDisplay, RuntimeEq, RuntimeHandle};

/// A JavaScript `Symbol`
pub struct JsiSymbol<'rt>(
    pub(crate) cxx::UniquePtr<sys::JsiSymbol>,
    pub(crate) PhantomData<&'rt ()>,
);

impl RuntimeEq for JsiSymbol<'_> {
    fn eq(&self, other: &Self, rt: &mut RuntimeHandle<'_>) -> bool {
        sys::Symbol_compare(
            rt.get_inner_mut(),
            self.0.as_ref().unwrap(),
            other.0.as_ref().unwrap(),
        )
    }
}

impl RuntimeDisplay for JsiSymbol<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>, rt: &mut RuntimeHandle<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string(rt.get_inner_mut()))
    }
}

unsafe impl<'rt> Send for JsiSymbol<'rt> {}
