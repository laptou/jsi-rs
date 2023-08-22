//! # Host objects
//!
//! Host objects are used in JSI to create special objects accessible from
//! JavaScript which have behaviour that is defined in native code.

use anyhow::bail;
use std::{marker::PhantomData, pin::Pin};

use crate::{sys, IntoValue, JsTaskCallback, JsiValue, PropName, RuntimeHandle};
use sys::CallInvokerCallback;

/// An owned host object
pub struct OwnedJsiHostObject<'rt>(
    pub(crate) cxx::UniquePtr<sys::HostObject>,
    pub(crate) PhantomData<&'rt mut ()>,
);

impl<'rt> OwnedJsiHostObject<'rt> {
    pub fn get(&mut self, prop: &PropName, rt: &mut RuntimeHandle<'rt>) -> JsiValue<'rt> {
        JsiValue(
            sys::HostObject_get(
                self.0.pin_mut(),
                rt.get_inner_mut(),
                prop.0.as_ref().unwrap(),
            ),
            PhantomData,
        )
    }

    pub fn set(&mut self, prop: &PropName, value: &JsiValue, rt: &mut RuntimeHandle<'rt>) {
        self.0.pin_mut().set(
            rt.get_inner_mut(),
            prop.0.as_ref().unwrap(),
            value.0.as_ref().unwrap(),
        )
    }

    pub fn properties(&mut self, rt: &mut RuntimeHandle<'rt>) -> Vec<PropName> {
        let mut props = sys::HostObject_getPropertyNames(self.0.pin_mut(), rt.get_inner_mut());
        let mut vec = Vec::with_capacity(props.len());
        loop {
            let ptr = sys::pop_prop_name_vector(props.pin_mut());
            if ptr.is_null() {
                break;
            }
            vec.push(PropName(ptr, PhantomData));
        }
        vec
    }
}

/// A shared reference to a host object
pub struct SharedJsiHostObject<'rt>(
    pub(crate) cxx::SharedPtr<sys::HostObject>,
    pub(crate) PhantomData<&'rt mut ()>,
);

/// Helper trait for implementing a host object in Rust
pub trait UserHostObject<'rt> {
    fn get(
        &mut self,
        name: PropName<'rt>,
        rt: &mut RuntimeHandle<'rt>,
    ) -> anyhow::Result<JsiValue<'rt>>;

    fn set(
        &mut self,
        name: PropName<'rt>,
        value: JsiValue<'rt>,
        rt: &mut RuntimeHandle<'rt>,
    ) -> anyhow::Result<()>;

    fn properties(&mut self, rt: &mut RuntimeHandle<'rt>) -> Vec<PropName<'rt>>;
}

impl<'rt, T: UserHostObject<'rt>> IntoValue<'rt> for T {
    fn into_value(self, rt: &mut RuntimeHandle<'rt>) -> JsiValue<'rt> {
        let host_object = OwnedJsiUserHostObject::new(self, rt);
        let host_object: OwnedJsiHostObject = host_object.into();
        host_object.into_value(rt)
    }
}

/// A generic wrapper struct is used to avoid using a triple-`Box` to contain
/// user host objects.
struct UserHostObjectWrapper<T>(T);

impl<'rt, T: UserHostObject<'rt>> sys::HostObjectImpl for UserHostObjectWrapper<T> {
    fn get(
        &mut self,
        rt: Pin<&mut sys::Runtime>,
        name: &sys::PropNameID,
    ) -> anyhow::Result<cxx::UniquePtr<sys::JsiValue>> {
        let mut rt = RuntimeHandle::new_unchecked(unsafe { rt.get_unchecked_mut() as *mut _ });
        let name = PropName(sys::PropNameID_copy(name, rt.get_inner_mut()), PhantomData);
        let value = UserHostObject::get(&mut self.0, name, &mut rt)?;
        Ok(value.0)
    }

    fn set(
        &mut self,
        rt: Pin<&mut sys::Runtime>,
        name: &sys::PropNameID,
        value: &sys::JsiValue,
    ) -> anyhow::Result<()> {
        let mut rt = RuntimeHandle::new_unchecked(unsafe { rt.get_unchecked_mut() as *mut _ });
        let name = PropName(sys::PropNameID_copy(name, rt.get_inner_mut()), PhantomData);
        let value = JsiValue(sys::Value_copy(value, rt.get_inner_mut()), PhantomData);
        UserHostObject::set(&mut self.0, name, value, &mut rt)
    }

    fn properties(&mut self, rt: Pin<&mut sys::Runtime>) -> Vec<cxx::UniquePtr<sys::PropNameID>> {
        let mut rt = RuntimeHandle::new_unchecked(unsafe { rt.get_unchecked_mut() as *mut _ });
        let props = UserHostObject::properties(&mut self.0, &mut rt);
        props.into_iter().map(|p| p.0).collect()
    }
}

impl<T> Drop for UserHostObjectWrapper<T> {
    fn drop(&mut self) {
        #[cfg(feature = "host-object-trace")]
        log::trace!(
            "dropping host object wrapper {:p} with host object {:p}",
            self,
            &self.0
        );
    }
}

/// A host object that is implemented in Rust using a [`UserHostObject`] trait
/// object. Start with this if you want to implement a host object.
pub struct OwnedJsiUserHostObject<'rt>(
    cxx::UniquePtr<sys::CxxHostObject>,
    PhantomData<&'rt mut ()>,
);

impl<'rt> OwnedJsiUserHostObject<'rt> {
    // The methods in here have an extra lifetime parameter `'u` so that
    // user-defined host objects don't have to have their lifetimes limited by
    // `'rt`. This allows us to use `'static` objects as host objects, for
    // example.

    pub fn new<'u: 'rt, T: UserHostObject<'u>>(ho: T, _rt: &mut RuntimeHandle<'rt>) -> Self {
        // Box 1 b/c C++ can't hold Rust objects w/o a Box
        // Box 2 b/c RustHostObject can't be generic (no way for the trampoline
        // function to know) which function to call w/o dynamic dispatch

        let b = Box::new(UserHostObjectWrapper(ho));

        #[cfg(feature = "host-object-trace")]
        log::trace!(
            "created owned jsi user host object of {} inner box: {:p}",
            std::any::type_name::<T>(),
            b.as_ref()
        );

        let b = Box::new(sys::RustHostObject(b));

        #[cfg(feature = "host-object-trace")]
        log::trace!(
            "created owned jsi user host object of {} outer box: {:p}",
            std::any::type_name::<T>(),
            b.as_ref()
        );

        let ptr = sys::CxxHostObject_create(b);

        #[cfg(feature = "host-object-trace")]
        log::trace!(
            "created owned jsi user host of {} object: {:p}",
            std::any::type_name::<T>(),
            ptr.as_ref().unwrap()
        );

        OwnedJsiUserHostObject(ptr, PhantomData)
    }

    // TODO: find some way to do a safety check at runtime w/o relying on `Any`,
    // because `Any` imposes `'static` which makes life hard due to how there
    // are lifetimes everywhere in this code
    pub fn get_inner_mut<'u: 'rt, T: UserHostObject<'u>>(&mut self) -> Option<&mut T> {
        let rho = sys::CxxHostObject_getInnerMut(self.0.as_mut().unwrap());
        let ho_inner = unsafe {
            &mut *(rho.0.as_mut() as *mut dyn sys::HostObjectImpl as *mut UserHostObjectWrapper<T>)
        };
        Some(&mut ho_inner.0)
    }

    pub fn get_inner<'u: 'rt, T: UserHostObject<'u>>(&self) -> Option<&T> {
        let rho = sys::CxxHostObject_getInner(self.0.as_ref().unwrap());
        let ho_inner = unsafe {
            &*(rho.0.as_ref() as *const dyn sys::HostObjectImpl as *const UserHostObjectWrapper<T>)
        };
        Some(&ho_inner.0)
    }
}

impl<'rt> Into<OwnedJsiHostObject<'rt>> for OwnedJsiUserHostObject<'rt> {
    fn into(self) -> OwnedJsiHostObject<'rt> {
        OwnedJsiHostObject(sys::CxxHostObject_toHostObjectU(self.0), self.1)
    }
}

impl<'rt> TryInto<OwnedJsiUserHostObject<'rt>> for OwnedJsiHostObject<'rt> {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<OwnedJsiUserHostObject<'rt>, Self::Error> {
        let ptr = sys::CxxHostObject_fromHostObjectU(self.0);

        if ptr.is_null() {
            bail!("this host object is not a Rust host object");
        } else {
            Ok(OwnedJsiUserHostObject(ptr, self.1))
        }
    }
}

/// A host object that is implemented in Rust using a [`UserHostObject`] trait object.
#[derive(Clone)]
pub struct SharedJsiUserHostObject<'rt>(
    cxx::SharedPtr<sys::CxxHostObject>,
    PhantomData<&'rt mut ()>,
);

impl<'rt> SharedJsiUserHostObject<'rt> {
    pub fn get_inner<'u: 'rt, T: UserHostObject<'u>>(&self) -> Option<&T> {
        #[cfg(feature = "host-object-trace")]
        log::trace!(
            "recovering shared inner host object as {} from {:p}",
            std::any::type_name::<T>(),
            self.0.as_ref().unwrap()
        );

        let rho = sys::CxxHostObject_getInner(self.0.as_ref().unwrap());

        #[cfg(feature = "host-object-trace")]
        log::trace!("recovered outer box {:p}", rho);

        let ho_inner = unsafe {
            &*(rho.0.as_ref() as *const dyn sys::HostObjectImpl as *const UserHostObjectWrapper<T>)
        };

        #[cfg(feature = "host-object-trace")]
        log::trace!("recovered inner box {:p}", ho_inner);

        #[cfg(feature = "host-object-trace")]
        log::trace!("recovered user object {:p}", &ho_inner.0);

        Some(&ho_inner.0)
    }
}

impl<'rt> Into<SharedJsiHostObject<'rt>> for SharedJsiUserHostObject<'rt> {
    fn into(self) -> SharedJsiHostObject<'rt> {
        SharedJsiHostObject(sys::CxxHostObject_toHostObjectS(self.0), self.1)
    }
}

impl<'rt> TryInto<SharedJsiUserHostObject<'rt>> for SharedJsiHostObject<'rt> {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<SharedJsiUserHostObject<'rt>, Self::Error> {
        let ptr = sys::CxxHostObject_fromHostObjectS(self.0);

        if ptr.is_null() {
            bail!("this host object is not a Rust host object");
        } else {
            Ok(SharedJsiUserHostObject(ptr, self.1))
        }
    }
}

unsafe impl<'rt> Send for OwnedJsiHostObject<'rt> {}
unsafe impl<'rt> Send for OwnedJsiUserHostObject<'rt> {}
unsafe impl<'rt> Send for SharedJsiHostObject<'rt> {}
unsafe impl<'rt> Send for SharedJsiUserHostObject<'rt> {}

/// Support trait to allow implementation of async functions via attribute
/// macro.
pub trait AsyncUserHostObject<'rt> {
    fn spawn(task: JsTaskCallback);

    fn invoke(cb: CallInvokerCallback);
}
