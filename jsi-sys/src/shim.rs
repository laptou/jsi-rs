use std::pin::Pin;

use crate::ffi::*;
use cxx::*;

impl HostObject {
    pub fn get(
        self: Pin<&mut HostObject>,
        rt: Pin<&mut Runtime>,
        name: &PropNameID,
    ) -> UniquePtr<JsiValue> {
        unsafe { HostObject_get(self, rt, name) }
    }

    pub fn get_property_names(
        self: Pin<&mut HostObject>,
        rt: Pin<&mut Runtime>,
    ) -> UniquePtr<CxxVector<PropNameID>> {
        unsafe { HostObject_getPropertyNames(self, rt) }
    }
}

#[repr(transparent)]
pub struct RustHostObject<'a>(pub Box<dyn HostObjectImpl + 'a>);

// i wanted to put these functions inside of an impl block, but this create a
// circular dependency headache on the C++ side b/c you can't access the members
// of a type before it is defined, so they're just normal functions

// functions are used by FFI interface, but Rust thinks it's dead code b/c it's
// pub(crate)
#[allow(dead_code)]
pub(crate) fn rho_get(
    rho: &mut RustHostObject,
    rt: Pin<&mut Runtime>,
    name: &PropNameID,
) -> anyhow::Result<UniquePtr<JsiValue>> {
    rho.0.get(rt, name)
}

#[allow(dead_code)]
pub(crate) fn rho_set(
    rho: &mut RustHostObject,
    rt: Pin<&mut Runtime>,
    name: &PropNameID,
    value: &JsiValue,
) -> anyhow::Result<()> {
    rho.0.set(rt, name, value)
}

#[allow(dead_code)]
pub(crate) fn rho_properties(
    rho: &mut RustHostObject,
    rt: Pin<&mut Runtime>,
) -> UniquePtr<CxxVector<PropNameID>> {
    unsafe {
        let props = rho.0.properties(rt);
        let mut vec = create_prop_name_vector();
        for prop in props {
            push_prop_name_vector(vec.pin_mut(), prop);
        }
        vec
    }
}

pub trait HostObjectImpl {
    fn get(
        &mut self,
        rt: Pin<&mut Runtime>,
        name: &PropNameID,
    ) -> anyhow::Result<UniquePtr<JsiValue>>;
    fn set(
        &mut self,
        rt: Pin<&mut Runtime>,
        name: &PropNameID,
        value: &JsiValue,
    ) -> anyhow::Result<()>;
    fn properties(&mut self, rt: Pin<&mut Runtime>) -> Vec<UniquePtr<PropNameID>>;
}

impl Runtime {
    pub fn evaluate_javascript(
        self: Pin<&mut Runtime>,
        buffer: &SharedPtr<Buffer>,
        source_url: &str,
    ) -> UniquePtr<JsiValue> {
        unsafe { Runtime_evaluateJavaScript(self, buffer, source_url) }
    }

    pub fn prepare_javascript(
        self: Pin<&mut Runtime>,
        buffer: &SharedPtr<Buffer>,
        source_url: &str,
    ) -> SharedPtr<ConstPreparedJavaScript> {
        unsafe { Runtime_prepareJavaScript(self, buffer, source_url) }
    }

    pub fn evaluate_prepared_javascript(
        self: Pin<&mut Runtime>,
        js: &SharedPtr<ConstPreparedJavaScript>,
    ) -> UniquePtr<JsiValue> {
        unsafe { Runtime_evaluatePreparedJavaScript(self, &js) }
    }

    pub fn global(self: Pin<&mut Runtime>) -> UniquePtr<JsiObject> {
        unsafe { Runtime_global(self) }
    }

    pub fn description(self: Pin<&mut Runtime>) -> UniquePtr<CxxString> {
        unsafe { Runtime_description(self) }
    }
}

impl PropNameID {
    pub fn from_str(rt: Pin<&mut Runtime>, s: &str) -> UniquePtr<Self> {
        unsafe { PropNameID_forUtf8(rt, s) }
    }

    pub fn from_jsi_string(rt: Pin<&mut Runtime>, s: &JsiString) -> UniquePtr<Self> {
        unsafe { PropNameID_forString(rt, s) }
    }

    pub fn to_string(&self, rt: Pin<&mut Runtime>) -> UniquePtr<CxxString> {
        unsafe { PropNameID_toUtf8(self, rt) }
    }

    pub fn compare(&self, other: &Self, rt: Pin<&mut Runtime>) -> bool {
        unsafe { PropNameID_compare(rt, self, other) }
    }
}

impl JsiSymbol {
    pub fn to_string(&self, rt: Pin<&mut Runtime>) -> UniquePtr<CxxString> {
        unsafe { Symbol_toString(self, rt) }
    }

    pub fn compare(&self, other: &Self, rt: Pin<&mut Runtime>) -> bool {
        unsafe { Symbol_compare(rt, self, other) }
    }
}

impl JsiString {
    pub fn from_str(rt: Pin<&mut Runtime>, s: &str) -> UniquePtr<Self> {
        unsafe { String_fromUtf8(rt, s) }
    }

    pub fn to_string(&self, rt: Pin<&mut Runtime>) -> UniquePtr<CxxString> {
        unsafe { String_toString(self, rt) }
    }

    pub fn compare(&self, other: &Self, rt: Pin<&mut Runtime>) -> bool {
        unsafe { String_compare(rt, self, other) }
    }
}

impl JsiObject {
    pub fn new(rt: Pin<&mut Runtime>) -> UniquePtr<Self> {
        unsafe { Object_create(rt) }
    }

    pub fn from_host_object(rt: Pin<&mut Runtime>, ho: SharedPtr<HostObject>) -> UniquePtr<Self> {
        unsafe { Object_createFromHostObjectShared(rt, ho) }
    }

    pub fn compare(&self, other: &Self, rt: Pin<&mut Runtime>) -> bool {
        unsafe { Object_compare(rt, self, other) }
    }

    pub fn get_property(&self, rt: Pin<&mut Runtime>, prop: &PropNameID) -> UniquePtr<JsiValue> {
        unsafe { Object_getProperty(self, rt, prop) }
    }

    pub fn set_property(
        self: Pin<&mut Self>,
        rt: Pin<&mut Runtime>,
        prop: &PropNameID,
        value: &JsiValue,
    ) {
        unsafe { Object_setProperty(self, rt, prop, value) }
    }

    pub fn as_array(&self, rt: Pin<&mut Runtime>) -> Option<UniquePtr<JsiArray>> {
        unsafe { Object_asArray(self, rt).ok() }
    }

    pub fn as_array_buffer(&self, rt: Pin<&mut Runtime>) -> Option<UniquePtr<JsiArrayBuffer>> {
        unsafe { Object_asArrayBuffer(self, rt).ok() }
    }

    pub fn as_function(&self, rt: Pin<&mut Runtime>) -> Option<UniquePtr<JsiFunction>> {
        unsafe { Object_asFunction(self, rt).ok() }
    }

    pub fn get_property_names(self: Pin<&mut Self>, rt: Pin<&mut Runtime>) -> UniquePtr<JsiArray> {
        unsafe { Object_getPropertyNames(self, rt) }
    }
}

impl JsiValue {
    pub fn undefined() -> UniquePtr<Self> {
        unsafe { Value_fromUndefined() }
    }

    pub fn null() -> UniquePtr<Self> {
        unsafe { Value_fromNull() }
    }

    pub fn int(i: i32) -> UniquePtr<Self> {
        unsafe { Value_fromInt(i) }
    }

    pub fn bool(b: bool) -> UniquePtr<Self> {
        unsafe { Value_fromBool(b) }
    }

    pub fn double(d: f64) -> UniquePtr<Self> {
        unsafe { Value_fromDouble(d) }
    }

    pub fn object(rt: Pin<&mut Runtime>, o: &JsiObject) -> UniquePtr<Self> {
        unsafe { Value_copyFromObject(rt, o) }
    }

    pub fn symbol(rt: Pin<&mut Runtime>, s: &JsiSymbol) -> UniquePtr<Self> {
        unsafe { Value_copyFromSymbol(rt, s) }
    }

    pub fn string(rt: Pin<&mut Runtime>, s: &JsiString) -> UniquePtr<Self> {
        unsafe { Value_copyFromString(rt, s) }
    }

    pub fn from_json(rt: Pin<&mut Runtime>, json: &str) -> UniquePtr<Self> {
        unsafe { Value_fromJson(rt, json) }
    }

    pub fn as_object(&self, rt: Pin<&mut Runtime>) -> Result<UniquePtr<JsiObject>, cxx::Exception> {
        unsafe { Value_asObject(self, rt) }
    }

    pub fn as_symbol(&self, rt: Pin<&mut Runtime>) -> Result<UniquePtr<JsiSymbol>, cxx::Exception> {
        unsafe { Value_asSymbol(self, rt) }
    }

    pub fn as_string(&self, rt: Pin<&mut Runtime>) -> Result<UniquePtr<JsiString>, cxx::Exception> {
        unsafe { Value_asString(self, rt) }
    }

    pub fn to_string(&self, rt: Pin<&mut Runtime>) -> UniquePtr<JsiString> {
        unsafe { Value_toString(self, rt) }
    }
}

impl JsiWeakObject {
    pub fn from_object(rt: Pin<&mut Runtime>, object: &JsiObject) -> UniquePtr<Self> {
        unsafe { WeakObject_fromObject(rt, object) }
    }

    pub fn lock(self: Pin<&mut Self>, rt: Pin<&mut Runtime>) -> UniquePtr<JsiValue> {
        unsafe { WeakObject_lock(self, rt) }
    }
}
