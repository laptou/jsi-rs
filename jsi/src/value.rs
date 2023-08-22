use std::fmt::Debug;
use std::marker::PhantomData;

use crate::array::JsiArray;
use crate::array_buffer::JsiArrayBuffer;
use crate::function::JsiFn;
use crate::object::JsiObject;
use crate::string::JsiString;
use crate::symbol::JsiSymbol;
use crate::{
    sys, FromObject, IntoObject, OwnedJsiHostObject, OwnedJsiUserHostObject, RuntimeClone,
    RuntimeDisplay, RuntimeEq, RuntimeHandle, SharedJsiHostObject, SharedJsiUserHostObject,
};

pub struct JsiValue<'rt>(
    pub(crate) cxx::UniquePtr<sys::JsiValue>,
    pub(crate) PhantomData<&'rt ()>,
);

impl<'rt> JsiValue<'rt> {
    pub fn new_undefined() -> Self {
        Self(sys::Value_fromUndefined(), PhantomData)
    }

    pub fn new_null() -> Self {
        Self(sys::Value_fromUndefined(), PhantomData)
    }

    pub fn new_number(n: f64) -> Self {
        Self(sys::Value_fromDouble(n), PhantomData)
    }

    pub fn new_bool(b: bool) -> Self {
        Self(sys::Value_fromBool(b), PhantomData)
    }

    pub fn new_json(s: &str, rt: &mut RuntimeHandle<'rt>) -> Self {
        Self(sys::Value_fromJson(rt.get_inner_mut(), s), PhantomData)
    }

    pub fn new_string(s: &str, rt: &mut RuntimeHandle<'rt>) -> Self {
        JsiString::new(s, rt).into_value(rt)
    }

    pub fn is_null(&self) -> bool {
        // qualify this method call to avoid confusion with UniquePtr::is_null
        sys::JsiValue::is_null(self.0.as_ref().unwrap())
    }

    pub fn is_undefined(&self) -> bool {
        self.0.is_undefined()
    }

    pub fn is_number(&self) -> bool {
        self.0.is_number()
    }

    pub fn is_bool(&self) -> bool {
        self.0.is_bool()
    }

    pub fn is_string(&self) -> bool {
        self.0.is_string()
    }

    pub fn is_symbol(&self) -> bool {
        self.0.is_symbol()
    }

    pub fn is_object(&self) -> bool {
        self.0.is_object()
    }

    pub fn is_truthy(&self, rt: &mut RuntimeHandle<'rt>) -> bool {
        match self.kind(rt) {
            JsiValueKind::Undefined | JsiValueKind::Null => false,
            JsiValueKind::Number(n) => n.abs() > f64::EPSILON,
            JsiValueKind::Bool(b) => b,
            JsiValueKind::String(s) => rt.display(&s).to_string().len() > 0,
            JsiValueKind::Symbol(_) | JsiValueKind::Object(_) => true,
        }
    }

    pub fn to_js_string(&self, rt: &mut RuntimeHandle<'rt>) -> JsiString<'rt> {
        JsiString(self.0.to_string(rt.get_inner_mut()), PhantomData)
    }

    pub fn kind(&self, rt: &mut RuntimeHandle<'rt>) -> JsiValueKind<'rt> {
        if self.is_null() {
            JsiValueKind::Null
        } else if self.is_undefined() {
            JsiValueKind::Undefined
        } else if self.is_number() {
            JsiValueKind::Number(FromValue::from_value(self, rt).unwrap())
        } else if self.is_bool() {
            JsiValueKind::Bool(FromValue::from_value(self, rt).unwrap())
        } else if self.is_string() {
            JsiValueKind::String(FromValue::from_value(self, rt).unwrap())
        } else if self.is_symbol() {
            JsiValueKind::Symbol(FromValue::from_value(self, rt).unwrap())
        } else if self.is_object() {
            JsiValueKind::Object(FromValue::from_value(self, rt).unwrap())
        } else {
            panic!("JSI value has no known type")
        }
    }

    pub fn try_into_js<T: FromValue<'rt>>(&self, rt: &mut RuntimeHandle<'rt>) -> Option<T> {
        T::from_value(self, rt)
    }

    pub fn into_js<T: FromValue<'rt>>(&self, rt: &mut RuntimeHandle<'rt>) -> T {
        self.try_into_js(rt).unwrap()
    }
}

unsafe impl<'rt> Send for JsiValue<'rt> {}

impl<'rt> Debug for JsiValue<'rt> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "JsiValue({:p})", &*self.0)
    }
}

impl RuntimeEq for JsiValue<'_> {
    fn eq(&self, other: &Self, rt: &mut RuntimeHandle<'_>) -> bool {
        sys::Value_compare(rt.get_inner_mut(), &*self.0, &*other.0)
    }
}

impl RuntimeDisplay for JsiValue<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>, rt: &mut RuntimeHandle<'_>) -> std::fmt::Result {
        let s = JsiString(self.0.to_string(rt.get_inner_mut()), PhantomData);
        {
            let disp = rt.display(&s);
            write!(f, "{}", disp)
        }
    }
}

impl RuntimeClone<'_> for JsiValue<'_> {
    fn clone(&self, rt: &mut RuntimeHandle<'_>) -> Self {
        Self(sys::Value_copy(&*self.0, rt.get_inner_mut()), PhantomData)
    }
}

/// Conversion trait to and from [`JsiValue`]. This is needed instead of the
/// normal `Into` trait b/c creating a JsiValue requires a [`RuntimeHandle`].
// Also has the benefit of allowing us to avoid implementing `Into` separately
// for `T` and `&T`.
pub trait IntoValue<'rt> {
    fn into_value(self, rt: &mut RuntimeHandle<'rt>) -> JsiValue<'rt>;
}

/// Conversion trait to and from [`JsiValue`]. This is needed instead of the
/// normal `Into` trait b/c creating a JsiValue requires a [`RuntimeHandle`].
/// Note that using this trait will create a copy of the data being converted
/// (ex.: using `as_value()` on a [`JsiObject`] will create a value that
/// contains a copy of that object).
pub trait AsValue<'rt> {
    fn as_value(&self, rt: &mut RuntimeHandle<'rt>) -> JsiValue<'rt>;
}

pub trait FromValue<'rt>: Sized {
    fn from_value(value: &JsiValue<'rt>, rt: &mut RuntimeHandle<'rt>) -> Option<Self>;
}

impl<'rt> FromValue<'rt> for JsiValue<'rt> {
    fn from_value(value: &JsiValue<'rt>, rt: &mut RuntimeHandle<'rt>) -> Option<Self> {
        Some(rt.clone(value))
    }
}

impl<'rt> FromValue<'rt> for f64 {
    fn from_value(value: &JsiValue<'rt>, _rt: &mut RuntimeHandle<'rt>) -> Option<Self> {
        value.0.get_number().ok()
    }
}

impl<'rt> FromValue<'rt> for bool {
    fn from_value(value: &JsiValue<'rt>, _rt: &mut RuntimeHandle<'rt>) -> Option<Self> {
        value.0.get_bool().ok()
    }
}

impl<'rt> FromValue<'rt> for JsiObject<'rt> {
    fn from_value(value: &JsiValue<'rt>, rt: &mut RuntimeHandle<'rt>) -> Option<Self> {
        sys::Value_asObject(&*value.0, rt.get_inner_mut())
            .ok()
            .map(|raw| JsiObject(raw, PhantomData))
    }
}

impl<'rt, T: FromObject<'rt>> FromValue<'rt> for T {
    fn from_value(value: &JsiValue<'rt>, rt: &mut RuntimeHandle<'rt>) -> Option<Self> {
        let obj: Option<JsiObject> = FromValue::from_value(value, rt);
        obj.and_then(|obj| FromObject::from_object(&obj, rt))
    }
}

impl<'rt> FromValue<'rt> for String {
    fn from_value(value: &JsiValue<'rt>, rt: &mut RuntimeHandle<'rt>) -> Option<Self> {
        let s: Option<JsiString> = FromValue::from_value(value, rt);
        s.map(|s| rt.to_string(&s))
    }
}

impl<'rt> FromValue<'rt> for JsiString<'rt> {
    fn from_value(value: &JsiValue<'rt>, rt: &mut RuntimeHandle<'rt>) -> Option<Self> {
        sys::Value_asString(&*value.0, rt.get_inner_mut())
            .ok()
            .map(|raw| JsiString(raw, PhantomData))
    }
}

impl<'rt> FromValue<'rt> for JsiSymbol<'rt> {
    fn from_value(value: &JsiValue<'rt>, rt: &mut RuntimeHandle<'rt>) -> Option<Self> {
        sys::Value_asSymbol(&*value.0, rt.get_inner_mut())
            .ok()
            .map(|raw| JsiSymbol(raw, PhantomData))
    }
}

impl<'rt> IntoValue<'rt> for JsiValue<'rt> {
    fn into_value(self, _: &mut RuntimeHandle<'rt>) -> JsiValue<'rt> {
        self
    }
}

impl<'rt> IntoValue<'rt> for bool {
    fn into_value(self, _rt: &mut RuntimeHandle<'rt>) -> JsiValue<'rt> {
        JsiValue::new_bool(self)
    }
}

impl<'rt> IntoValue<'rt> for String {
    fn into_value(self, rt: &mut RuntimeHandle<'rt>) -> JsiValue<'rt> {
        JsiValue::new_string(&self, rt)
    }
}

impl<'rt> IntoValue<'rt> for &str {
    fn into_value(self, rt: &mut RuntimeHandle<'rt>) -> JsiValue<'rt> {
        JsiValue::new_string(self, rt)
    }
}

impl<'rt> IntoValue<'rt> for f64 {
    fn into_value(self, _rt: &mut RuntimeHandle<'rt>) -> JsiValue<'rt> {
        JsiValue::new_number(self)
    }
}

impl<'rt> IntoValue<'rt> for usize {
    fn into_value(self, rt: &mut RuntimeHandle<'rt>) -> JsiValue<'rt> {
        IntoValue::into_value(self as f64, rt)
    }
}

impl<'rt> IntoValue<'rt> for u8 {
    fn into_value(self, rt: &mut RuntimeHandle<'rt>) -> JsiValue<'rt> {
        IntoValue::into_value(self as f64, rt)
    }
}

impl<'rt> IntoValue<'rt> for u16 {
    fn into_value(self, rt: &mut RuntimeHandle<'rt>) -> JsiValue<'rt> {
        IntoValue::into_value(self as f64, rt)
    }
}

impl<'rt> IntoValue<'rt> for u32 {
    fn into_value(self, rt: &mut RuntimeHandle<'rt>) -> JsiValue<'rt> {
        IntoValue::into_value(self as f64, rt)
    }
}

impl<'rt> IntoValue<'rt> for u64 {
    fn into_value(self, rt: &mut RuntimeHandle<'rt>) -> JsiValue<'rt> {
        IntoValue::into_value(self as f64, rt)
    }
}

impl<'rt> IntoValue<'rt> for isize {
    fn into_value(self, rt: &mut RuntimeHandle<'rt>) -> JsiValue<'rt> {
        IntoValue::into_value(self as f64, rt)
    }
}

impl<'rt> IntoValue<'rt> for i8 {
    fn into_value(self, rt: &mut RuntimeHandle<'rt>) -> JsiValue<'rt> {
        IntoValue::into_value(self as f64, rt)
    }
}

impl<'rt> IntoValue<'rt> for i16 {
    fn into_value(self, rt: &mut RuntimeHandle<'rt>) -> JsiValue<'rt> {
        IntoValue::into_value(self as f64, rt)
    }
}

impl<'rt> IntoValue<'rt> for i32 {
    fn into_value(self, rt: &mut RuntimeHandle<'rt>) -> JsiValue<'rt> {
        IntoValue::into_value(self as f64, rt)
    }
}

impl<'rt> IntoValue<'rt> for i64 {
    fn into_value(self, rt: &mut RuntimeHandle<'rt>) -> JsiValue<'rt> {
        IntoValue::into_value(self as f64, rt)
    }
}

impl<'rt> IntoValue<'rt> for () {
    fn into_value(self, _rt: &mut RuntimeHandle<'rt>) -> JsiValue<'rt> {
        JsiValue::new_undefined()
    }
}

impl<'rt, T: IntoValue<'rt>> IntoValue<'rt> for Vec<T> {
    fn into_value(self, rt: &mut RuntimeHandle<'rt>) -> JsiValue<'rt> {
        let mut arr = JsiArray::new(self.len(), rt);

        for (idx, item) in self.into_iter().enumerate() {
            arr.set(idx, &item.into_value(rt), rt);
        }

        arr.into_value(rt)
    }
}

impl<'rt, T: IntoValue<'rt>> IntoValue<'rt> for Option<T> {
    fn into_value(self, rt: &mut RuntimeHandle<'rt>) -> JsiValue<'rt> {
        match self {
            Some(inner) => inner.into_value(rt),
            None => JsiValue::new_null(),
        }
    }
}

impl<'rt> IntoValue<'rt> for JsiObject<'rt> {
    fn into_value(self, rt: &mut RuntimeHandle<'rt>) -> JsiValue<'rt> {
        JsiValue(
            sys::Value_fromObject(rt.get_inner_mut(), self.0),
            PhantomData,
        )
    }
}

impl<'rt> IntoValue<'rt> for JsiString<'rt> {
    fn into_value(self, rt: &mut RuntimeHandle<'rt>) -> JsiValue<'rt> {
        JsiValue(
            sys::Value_fromString(rt.get_inner_mut(), self.0),
            PhantomData,
        )
    }
}

impl<'rt> IntoValue<'rt> for JsiSymbol<'rt> {
    fn into_value(self, rt: &mut RuntimeHandle<'rt>) -> JsiValue<'rt> {
        JsiValue(
            sys::Value_fromSymbol(rt.get_inner_mut(), self.0),
            PhantomData,
        )
    }
}

impl<'rt> IntoValue<'rt> for JsiArray<'rt> {
    fn into_value(self, rt: &mut RuntimeHandle<'rt>) -> JsiValue<'rt> {
        let obj: JsiObject = self.into();
        obj.into_value(rt)
    }
}

impl<'rt> IntoValue<'rt> for JsiArrayBuffer<'rt> {
    fn into_value(self, rt: &mut RuntimeHandle<'rt>) -> JsiValue<'rt> {
        let obj: JsiObject = self.into();
        obj.into_value(rt)
    }
}

impl<'rt> IntoValue<'rt> for JsiFn<'rt> {
    fn into_value(self, rt: &mut RuntimeHandle<'rt>) -> JsiValue<'rt> {
        let obj: JsiObject = self.into();
        obj.into_value(rt)
    }
}

impl<'rt> IntoValue<'rt> for OwnedJsiUserHostObject<'rt> {
    fn into_value(self, rt: &mut RuntimeHandle<'rt>) -> JsiValue<'rt> {
        let obj: JsiObject = self.into_object(rt);
        obj.into_value(rt)
    }
}

impl<'rt> IntoValue<'rt> for OwnedJsiHostObject<'rt> {
    fn into_value(self, rt: &mut RuntimeHandle<'rt>) -> JsiValue<'rt> {
        let obj: JsiObject = self.into_object(rt);
        obj.into_value(rt)
    }
}

impl<'rt> IntoValue<'rt> for SharedJsiUserHostObject<'rt> {
    fn into_value(self, rt: &mut RuntimeHandle<'rt>) -> JsiValue<'rt> {
        let obj: JsiObject = self.into_object(rt);
        obj.into_value(rt)
    }
}

impl<'rt> IntoValue<'rt> for SharedJsiHostObject<'rt> {
    fn into_value(self, rt: &mut RuntimeHandle<'rt>) -> JsiValue<'rt> {
        let obj: JsiObject = self.into_object(rt);
        obj.into_value(rt)
    }
}

impl<'rt, T: AsValue<'rt>> AsValue<'rt> for Option<&T> {
    fn as_value(&self, rt: &mut RuntimeHandle<'rt>) -> JsiValue<'rt> {
        match self {
            Some(inner) => inner.as_value(rt),
            None => JsiValue::new_null(),
        }
    }
}

impl<'rt> AsValue<'rt> for JsiObject<'rt> {
    fn as_value(&self, rt: &mut RuntimeHandle<'rt>) -> JsiValue<'rt> {
        JsiValue(
            sys::Value_copyFromObject(rt.get_inner_mut(), &*self.0),
            PhantomData,
        )
    }
}

impl<'rt> AsValue<'rt> for JsiString<'rt> {
    fn as_value(&self, rt: &mut RuntimeHandle<'rt>) -> JsiValue<'rt> {
        JsiValue(
            sys::Value_copyFromString(rt.get_inner_mut(), self.0.as_ref().unwrap()),
            PhantomData,
        )
    }
}

impl<'rt> AsValue<'rt> for JsiSymbol<'rt> {
    fn as_value(&self, rt: &mut RuntimeHandle<'rt>) -> JsiValue<'rt> {
        JsiValue(
            sys::Value_copyFromSymbol(rt.get_inner_mut(), self.0.as_ref().unwrap()),
            PhantomData,
        )
    }
}

impl<'rt> AsValue<'rt> for JsiArray<'rt> {
    fn as_value(&self, rt: &mut RuntimeHandle<'rt>) -> JsiValue<'rt> {
        // this is a subclass of JsiObject, so pointer cast is safe
        let ptr = &*self.0 as *const _ as *const sys::JsiObject;

        JsiValue(
            sys::Value_copyFromObject(rt.get_inner_mut(), unsafe { &*ptr }),
            PhantomData,
        )
    }
}

impl<'rt> AsValue<'rt> for JsiArrayBuffer<'rt> {
    fn as_value(&self, rt: &mut RuntimeHandle<'rt>) -> JsiValue<'rt> {
        // this is a subclass of JsiObject, so pointer cast is safe
        let ptr = &*self.0 as *const _ as *const sys::JsiObject;

        JsiValue(
            sys::Value_copyFromObject(rt.get_inner_mut(), unsafe { &*ptr }),
            PhantomData,
        )
    }
}

impl<'rt> AsValue<'rt> for JsiFn<'rt> {
    fn as_value(&self, rt: &mut RuntimeHandle<'rt>) -> JsiValue<'rt> {
        // this is a subclass of JsiObject, so pointer cast is safe
        let ptr = &*self.0 as *const _ as *const sys::JsiObject;

        JsiValue(
            sys::Value_copyFromObject(rt.get_inner_mut(), unsafe { &*ptr }),
            PhantomData,
        )
    }
}

pub enum JsiValueKind<'rt> {
    Undefined,
    Null,
    Number(f64),
    Bool(bool),
    String(JsiString<'rt>),
    Symbol(JsiSymbol<'rt>),
    Object(JsiObject<'rt>),
}
