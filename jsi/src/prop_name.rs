use std::marker::PhantomData;

use crate::string::JsiString;
use crate::{sys, RuntimeClone, RuntimeDisplay, RuntimeEq, RuntimeHandle};

/// A `PropName`, which is used to retrieve properties from `Object`s.
pub struct PropName<'rt>(
    pub(crate) cxx::UniquePtr<sys::PropNameID>,
    pub(crate) PhantomData<&'rt mut ()>,
);

impl<'rt> PropName<'rt> {
    pub fn new(name: &str, rt: &mut RuntimeHandle<'rt>) -> Self {
        PropName(
            sys::PropNameID_forUtf8(rt.get_inner_mut(), name),
            PhantomData,
        )
    }

    pub fn from_string(name: JsiString<'rt>, rt: &mut RuntimeHandle<'rt>) -> Self {
        PropName(
            sys::PropNameID_forString(rt.get_inner_mut(), &*name.0),
            PhantomData,
        )
    }
}

impl RuntimeClone<'_> for PropName<'_> {
    fn clone(&self, rt: &mut RuntimeHandle<'_>) -> Self {
        PropName(
            sys::PropNameID_copy(self.0.as_ref().unwrap(), rt.get_inner_mut()),
            PhantomData,
        )
    }
}

impl RuntimeDisplay for PropName<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>, rt: &mut RuntimeHandle<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            sys::PropNameID_toUtf8(&*self.0, rt.get_inner_mut()).to_string()
        )
    }
}

impl RuntimeEq for PropName<'_> {
    fn eq(&self, other: &Self, rt: &mut RuntimeHandle<'_>) -> bool {
        sys::PropNameID_compare(rt.get_inner_mut(), &*self.0, &*other.0)
    }
}
