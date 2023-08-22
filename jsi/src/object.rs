use std::marker::PhantomData;

use crate::array::JsiArray;
use crate::array_buffer::JsiArrayBuffer;
use crate::function::JsiFn;
use crate::host_object::{OwnedJsiHostObject, SharedJsiHostObject};
use crate::{
    sys, FromValue, JsiValue, OwnedJsiUserHostObject, PropName, RuntimeHandle,
    SharedJsiUserHostObject,
};

unsafe impl<'rt> Send for JsiObject<'rt> {}

/// A JavaScript `Object`.
pub struct JsiObject<'rt>(
    pub(crate) cxx::UniquePtr<sys::JsiObject>,
    pub(crate) PhantomData<&'rt mut ()>,
);

impl<'rt> JsiObject<'rt> {
    pub fn new(rt: &mut RuntimeHandle<'rt>) -> Self {
        JsiObject(sys::Object_create(rt.get_inner_mut()), PhantomData)
    }

    pub fn get(&self, prop: PropName, rt: &mut RuntimeHandle<'rt>) -> JsiValue<'rt> {
        JsiValue(
            sys::Object_getProperty(
                self.0.as_ref().unwrap(),
                rt.get_inner_mut(),
                prop.0.as_ref().unwrap(),
            ),
            PhantomData,
        )
    }

    pub fn has(&self, prop: PropName, rt: &mut RuntimeHandle<'rt>) -> bool {
        self.0
            .has_property(rt.get_inner_mut(), prop.0.as_ref().unwrap())
    }

    pub fn set(&mut self, prop: PropName, value: &JsiValue, rt: &mut RuntimeHandle<'rt>) {
        sys::Object_setProperty(
            self.0.pin_mut(),
            rt.get_inner_mut(),
            prop.0.as_ref().unwrap(),
            value.0.as_ref().unwrap(),
        )
    }

    pub fn properties(&mut self, rt: &mut RuntimeHandle<'rt>) -> JsiArray<'rt> {
        JsiArray(
            sys::Object_getPropertyNames(self.0.pin_mut(), rt.get_inner_mut()),
            PhantomData,
        )
    }

    pub fn is_array(&self, rt: &mut RuntimeHandle<'rt>) -> bool {
        self.0.is_array(rt.get_inner_mut())
    }

    pub fn is_array_buffer(&self, rt: &mut RuntimeHandle<'rt>) -> bool {
        self.0.is_array_buffer(rt.get_inner_mut())
    }

    pub fn is_fn(&self, rt: &mut RuntimeHandle<'rt>) -> bool {
        self.0.is_function(rt.get_inner_mut())
    }

    pub fn is_instance(&mut self, ctor: JsiFn, rt: &mut RuntimeHandle<'rt>) -> bool {
        self.0
            .pin_mut()
            .instance_of(rt.get_inner_mut(), ctor.0.as_ref().unwrap())
    }
}

pub trait FromObject<'rt>: Sized {
    fn from_object(obj: &JsiObject<'rt>, rt: &mut RuntimeHandle<'rt>) -> Option<Self>;
}

pub trait IntoObject<'rt> {
    fn into_object(self, rt: &mut RuntimeHandle<'rt>) -> JsiObject<'rt>;
}

impl<'rt> FromObject<'rt> for JsiArray<'rt> {
    fn from_object(obj: &JsiObject<'rt>, rt: &mut RuntimeHandle<'rt>) -> Option<Self> {
        sys::Object_asArray(&*obj.0, rt.get_inner_mut())
            .ok()
            .map(|raw| JsiArray(raw, PhantomData))
    }
}

impl<'rt> FromObject<'rt> for JsiArrayBuffer<'rt> {
    fn from_object(ojb: &JsiObject<'rt>, rt: &mut RuntimeHandle<'rt>) -> Option<Self> {
        sys::Object_asArrayBuffer(&*ojb.0, rt.get_inner_mut())
            .ok()
            .map(|raw| JsiArrayBuffer(raw, PhantomData))
    }
}

impl<'rt> FromObject<'rt> for JsiFn<'rt> {
    fn from_object(obj: &JsiObject<'rt>, rt: &mut RuntimeHandle<'rt>) -> Option<Self> {
        sys::Object_asFunction(&*obj.0, rt.get_inner_mut())
            .ok()
            .map(|raw| JsiFn(raw, PhantomData))
    }
}

impl<'rt, T: FromValue<'rt>> FromObject<'rt> for Vec<T> {
    fn from_object(obj: &JsiObject<'rt>, rt: &mut RuntimeHandle<'rt>) -> Option<Self> {
        let arr: Option<JsiArray> = FromObject::from_object(obj, rt);
        arr.and_then(|arr| {
            let arr: Vec<_> = arr.iter(rt).collect();

            arr.into_iter()
                .map(|it| FromValue::from_value(&it, rt))
                .collect()
        })
    }
}

impl<'rt> From<JsiArray<'rt>> for JsiObject<'rt> {
    fn from(a: JsiArray<'rt>) -> Self {
        // JsiArray is a subclass of JsiObject, so we can just pointer cast
        JsiObject(
            unsafe { cxx::UniquePtr::<_>::from_raw(a.0.into_raw() as *mut _) },
            PhantomData,
        )
    }
}

impl<'rt> From<JsiArrayBuffer<'rt>> for JsiObject<'rt> {
    fn from(a: JsiArrayBuffer<'rt>) -> Self {
        // JsiArrayBuffer is a subclass of JsiObject, so we can just pointer cast
        JsiObject(
            unsafe { cxx::UniquePtr::<_>::from_raw(a.0.into_raw() as *mut _) },
            PhantomData,
        )
    }
}

impl<'rt> From<JsiFn<'rt>> for JsiObject<'rt> {
    fn from(a: JsiFn<'rt>) -> Self {
        // JsiFn is a subclass of JsiObject, so we can just pointer cast
        JsiObject(
            unsafe { cxx::UniquePtr::<_>::from_raw(a.0.into_raw() as *mut _) },
            PhantomData,
        )
    }
}

impl<'rt, T: Into<JsiObject<'rt>>> IntoObject<'rt> for T {
    fn into_object(self, _: &mut RuntimeHandle<'rt>) -> JsiObject<'rt> {
        Into::into(self)
    }
}

impl<'rt> IntoObject<'rt> for OwnedJsiHostObject<'rt> {
    fn into_object(self, rt: &mut RuntimeHandle<'rt>) -> JsiObject<'rt> {
        JsiObject(
            sys::Object_createFromHostObjectUnique(rt.get_inner_mut(), self.0),
            PhantomData,
        )
    }
}

impl<'rt> IntoObject<'rt> for OwnedJsiUserHostObject<'rt> {
    fn into_object(self, rt: &mut RuntimeHandle<'rt>) -> JsiObject<'rt> {
        let obj: OwnedJsiHostObject = self.into();
        obj.into_object(rt)
    }
}

impl<'rt> IntoObject<'rt> for SharedJsiHostObject<'rt> {
    fn into_object(self, rt: &mut RuntimeHandle<'rt>) -> JsiObject<'rt> {
        JsiObject(
            sys::Object_createFromHostObjectShared(rt.get_inner_mut(), self.0),
            PhantomData,
        )
    }
}

impl<'rt> IntoObject<'rt> for SharedJsiUserHostObject<'rt> {
    fn into_object(self, rt: &mut RuntimeHandle<'rt>) -> JsiObject<'rt> {
        let obj: SharedJsiHostObject = self.into();
        obj.into_object(rt)
    }
}

impl<'rt> FromObject<'rt> for SharedJsiHostObject<'rt> {
    fn from_object(obj: &JsiObject<'rt>, rt: &mut RuntimeHandle<'rt>) -> Option<Self> {
        sys::Object_asHostObject(&*obj.0, rt.get_inner_mut())
            .ok()
            .map(|raw| SharedJsiHostObject(raw, PhantomData))
    }
}

impl<'rt> FromObject<'rt> for SharedJsiUserHostObject<'rt> {
    fn from_object(obj: &JsiObject<'rt>, rt: &mut RuntimeHandle<'rt>) -> Option<Self> {
        let obj: Option<SharedJsiHostObject> = FromObject::from_object(obj, rt);
        obj.and_then(|obj| obj.try_into().ok())
    }
}
