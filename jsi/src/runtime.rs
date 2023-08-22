use crate::object::JsiObject;
use crate::sys;
use std::cell::Cell;
use std::marker::PhantomData;
use std::pin::Pin;

#[derive(Debug)]
pub struct RuntimeHandle<'rt>(pub(crate) *mut sys::Runtime, PhantomData<&'rt mut ()>);

impl<'rt> RuntimeHandle<'rt> {
    // Creates a new RuntimeHandle; it's the caller's responsibility to make
    // sure that the runtime is not destroyed while objects under this runtime
    // are still being used
    pub fn new_unchecked(ptr: *mut sys::Runtime) -> Self {
        RuntimeHandle(ptr, PhantomData)
    }

    pub fn get_inner_mut(&mut self) -> Pin<&'rt mut sys::Runtime> {
        unsafe { Pin::new_unchecked(&mut *self.0) }
    }

    pub fn get_inner(&mut self) -> &'rt sys::Runtime {
        unsafe { &*self.0 }
    }

    pub fn global(&mut self) -> JsiObject<'rt> {
        JsiObject(sys::Runtime_global(self.get_inner_mut()), PhantomData)
    }

    pub fn eq<T: RuntimeEq>(&mut self, lhs: &T, rhs: &T) -> bool {
        lhs.eq(rhs, self)
    }

    pub fn clone<T: RuntimeClone<'rt>>(&mut self, it: &T) -> T {
        it.clone(self)
    }

    pub fn display<'a, T: RuntimeDisplay>(&'a mut self, it: &'a T) -> impl std::fmt::Display + 'a
    where
        'rt: 'a,
    {
        // unsafe: transmute converts RuntimeHandle<'rt> to RuntimeHandle<'a>
        RuntimeDisplayWrapper(Cell::new(Some(unsafe { std::mem::transmute(self) })), it)
    }

    pub fn to_string<'a, T: RuntimeDisplay>(&'a mut self, it: &'a T) -> String {
        self.display(it).to_string()
    }
}

unsafe impl<'rt> Send for RuntimeHandle<'rt> {}

pub trait RuntimeEq {
    fn eq(&self, other: &Self, rt: &mut RuntimeHandle<'_>) -> bool;
}

pub trait RuntimeClone<'rt> {
    fn clone(&self, rt: &mut RuntimeHandle<'rt>) -> Self;
}

impl<'a, T: RuntimeClone<'a>> RuntimeClone<'a> for Vec<T> {
    fn clone(&self, rt: &mut RuntimeHandle<'a>) -> Self {
        let mut v = Vec::with_capacity(self.len());
        for i in self {
            v.push(RuntimeClone::clone(i, rt));
        }
        v
    }
}

pub trait RuntimeDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>, rt: &mut RuntimeHandle<'_>) -> std::fmt::Result;
}

struct RuntimeDisplayWrapper<'a, T: RuntimeDisplay>(Cell<Option<&'a mut RuntimeHandle<'a>>>, &'a T);

impl<'a, T: RuntimeDisplay> std::fmt::Display for RuntimeDisplayWrapper<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.0.replace(None).unwrap();
        let r = self.1.fmt(f, s);
        self.0.replace(Some(s));
        r
    }
}
