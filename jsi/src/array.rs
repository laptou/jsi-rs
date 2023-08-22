use std::marker::PhantomData;

use crate::{sys, JsiValue, RuntimeHandle};

/// A JavaScript
/// [`Array`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array).
/// Can be used to share large buffers of data with a React Native application.
pub struct JsiArray<'rt>(
    pub(crate) cxx::UniquePtr<sys::JsiArray>,
    pub(crate) PhantomData<&'rt mut ()>,
);

impl<'rt> JsiArray<'rt> {
    pub fn new(len: usize, rt: &mut RuntimeHandle<'rt>) -> Self {
        Self(
            sys::Array_createWithLength(rt.get_inner_mut(), len),
            PhantomData,
        )
    }

    pub fn len(&self, rt: &mut RuntimeHandle<'rt>) -> usize {
        self.0.length(rt.get_inner_mut())
    }

    pub fn get(&self, index: usize, rt: &mut RuntimeHandle<'rt>) -> JsiValue<'rt> {
        JsiValue(
            sys::Array_get(&*self.0, rt.get_inner_mut(), index),
            PhantomData,
        )
    }

    pub fn set(&mut self, index: usize, value: &JsiValue<'rt>, rt: &mut RuntimeHandle<'rt>) {
        sys::Array_set(
            self.0.pin_mut(),
            rt.get_inner_mut(),
            index,
            value.0.as_ref().unwrap(),
        )
    }

    pub fn iter<'a>(&'a self, rt: &'a mut RuntimeHandle<'rt>) -> JsiArrayIter<'a, 'rt> {
        JsiArrayIter(self, 0, rt)
    }
}

pub struct JsiArrayIter<'a, 'rt: 'a>(&'a JsiArray<'rt>, usize, &'a mut RuntimeHandle<'rt>);

impl<'a, 'rt: 'a> Iterator for JsiArrayIter<'a, 'rt> {
    type Item = JsiValue<'rt>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.1 < self.0.len(self.2) {
            let val = self.0.get(self.1, self.2);
            self.1 += 1;
            Some(val)
        } else {
            None
        }
    }
}

// impl<'a, 'rt: 'a> ExactSizeIterator for JsiArrayIter<'a, 'rt> {
//     fn len(&self) -> usize {
//         self.0.len(self.2)
//     }
// }
