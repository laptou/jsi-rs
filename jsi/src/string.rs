use std::marker::PhantomData;

use crate::{sys, RuntimeClone, RuntimeDisplay, RuntimeEq, RuntimeHandle};

/// A JavaScript `String`
pub struct JsiString<'rt>(
    pub(crate) cxx::UniquePtr<sys::JsiString>,
    pub(crate) PhantomData<&'rt mut ()>,
);

impl<'rt> JsiString<'rt> {
    pub fn new(name: &str, rt: &mut RuntimeHandle<'rt>) -> Self {
        JsiString(sys::String_fromUtf8(rt.get_inner_mut(), name), PhantomData)
    }
}

impl RuntimeEq for JsiString<'_> {
    fn eq(&self, other: &Self, rt: &mut RuntimeHandle<'_>) -> bool {
        sys::String_compare(
            rt.get_inner_mut(),
            self.0.as_ref().unwrap(),
            other.0.as_ref().unwrap(),
        )
    }
}

impl RuntimeDisplay for JsiString<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>, rt: &mut RuntimeHandle<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string(rt.get_inner_mut()))
    }
}

impl<'rt> RuntimeClone<'rt> for JsiString<'rt> {
    fn clone(&self, rt: &mut RuntimeHandle<'rt>) -> Self {
        let text = self.0.to_string(rt.get_inner_mut());
        let text = String::from_utf8_lossy(text.as_bytes());
        Self::new(text.as_ref(), rt)
    }
}

unsafe impl<'rt> Send for JsiString<'rt> {}
