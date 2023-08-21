use std::marker::PhantomData;

use crate::{sys, RuntimeHandle};

pub struct JsiArrayBuffer<'rt>(
    pub(crate) cxx::UniquePtr<sys::JsiArrayBuffer>,
    pub(crate) PhantomData<&'rt mut ()>,
);

impl<'rt> JsiArrayBuffer<'rt> {
    pub fn data(&mut self, rt: &mut RuntimeHandle<'rt>) -> &mut [u8] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.0.pin_mut().data(rt.get_inner_mut()),
                self.0.length(rt.get_inner_mut()),
            )
        }
    }
}
