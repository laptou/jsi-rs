use std::pin::Pin;

use crate::ffi::*;
use cxx::*;

impl HostObject {
    pub fn get(
        self: Pin<&mut HostObject>,
        rt: Pin<&mut Runtime>,
        name: &PropNameID,
    ) -> UniquePtr<JsiValue> {
        HostObject_get(self, rt, name)
    }

    pub fn get_property_names(
        self: Pin<&mut HostObject>,
        rt: Pin<&mut Runtime>,
    ) -> UniquePtr<CxxVector<PropNameID>> {
        HostObject_getPropertyNames(self, rt)
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
    let props = rho.0.properties(rt);
    let mut vec = create_prop_name_vector();
    for prop in props {
        push_prop_name_vector(vec.pin_mut(), prop);
    }
    vec
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
        Runtime_evaluateJavaScript(self, buffer, source_url)
    }

    pub fn prepare_javascript(
        self: Pin<&mut Runtime>,
        buffer: &SharedPtr<Buffer>,
        source_url: &str,
    ) -> SharedPtr<ConstPreparedJavaScript> {
        Runtime_prepareJavaScript(self, buffer, source_url)
    }

    pub fn evaluate_prepared_javascript(
        self: Pin<&mut Runtime>,
        js: &SharedPtr<ConstPreparedJavaScript>,
    ) -> UniquePtr<JsiValue> {
        Runtime_evaluatePreparedJavaScript(self, &js)
    }

    pub fn global(self: Pin<&mut Runtime>) -> UniquePtr<JsiObject> {
        Runtime_global(self)
    }

    pub fn description(self: Pin<&mut Runtime>) -> UniquePtr<CxxString> {
        Runtime_description(self)
    }
}

impl PropNameID {
    pub fn from_str(rt: Pin<&mut Runtime>, s: &str) -> UniquePtr<Self> {
        PropNameID_forUtf8(rt, s)
    }

    pub fn from_jsi_string(rt: Pin<&mut Runtime>, s: &JsiString) -> UniquePtr<Self> {
        PropNameID_forString(rt, s)
    }

    pub fn to_string(&self, rt: Pin<&mut Runtime>) -> UniquePtr<CxxString> {
        PropNameID_toUtf8(self, rt)
    }

    pub fn compare(&self, other: &Self, rt: Pin<&mut Runtime>) -> bool {
        PropNameID_compare(rt, self, other)
    }
}

impl JsiSymbol {
    pub fn to_string(&self, rt: Pin<&mut Runtime>) -> UniquePtr<CxxString> {
        Symbol_toString(self, rt)
    }

    pub fn compare(&self, other: &Self, rt: Pin<&mut Runtime>) -> bool {
        Symbol_compare(rt, self, other)
    }
}

impl JsiString {
    pub fn from_str(rt: Pin<&mut Runtime>, s: &str) -> UniquePtr<Self> {
        String_fromUtf8(rt, s)
    }

    pub fn to_string(&self, rt: Pin<&mut Runtime>) -> UniquePtr<CxxString> {
        String_toString(self, rt)
    }

    pub fn compare(&self, other: &Self, rt: Pin<&mut Runtime>) -> bool {
        String_compare(rt, self, other)
    }
}

impl JsiObject {
    pub fn new(rt: Pin<&mut Runtime>) -> UniquePtr<Self> {
        Object_create(rt)
    }

    pub fn from_host_object(rt: Pin<&mut Runtime>, ho: SharedPtr<HostObject>) -> UniquePtr<Self> {
        Object_createFromHostObjectShared(rt, ho)
    }

    pub fn compare(&self, other: &Self, rt: Pin<&mut Runtime>) -> bool {
        Object_compare(rt, self, other)
    }

    pub fn get_property(&self, rt: Pin<&mut Runtime>, prop: &PropNameID) -> UniquePtr<JsiValue> {
        Object_getProperty(self, rt, prop)
    }

    pub fn set_property(
        self: Pin<&mut Self>,
        rt: Pin<&mut Runtime>,
        prop: &PropNameID,
        value: UniquePtr<JsiValue>,
    ) {
        Object_setProperty(self, rt, prop, value)
    }

    pub fn as_array(&self, rt: Pin<&mut Runtime>) -> Option<UniquePtr<JsiArray>> {
        Object_asArray(self, rt).ok()
    }

    pub fn as_array_buffer(&self, rt: Pin<&mut Runtime>) -> Option<UniquePtr<JsiArrayBuffer>> {
        Object_asArrayBuffer(self, rt).ok()
    }

    pub fn as_function(&self, rt: Pin<&mut Runtime>) -> Option<UniquePtr<JsiFunction>> {
        Object_asFunction(self, rt).ok()
    }

    pub fn get_property_names(self: Pin<&mut Self>, rt: Pin<&mut Runtime>) -> UniquePtr<JsiArray> {
        Object_getPropertyNames(self, rt)
    }
}

impl JsiValue {
    pub fn create_undefined() -> UniquePtr<Self> {
        Value_fromUndefined()
    }

    pub fn create_null() -> UniquePtr<Self> {
        Value_fromNull()
    }

    pub fn create_int(i: i32) -> UniquePtr<Self> {
        Value_fromInt(i)
    }

    pub fn create_bool(b: bool) -> UniquePtr<Self> {
        Value_fromBool(b)
    }

    pub fn create_double(d: f64) -> UniquePtr<Self> {
        Value_fromDouble(d)
    }

    pub fn copy_object(rt: Pin<&mut Runtime>, o: &JsiObject) -> UniquePtr<Self> {
        Value_copyFromObject(rt, o)
    }

    pub fn copy_symbol(rt: Pin<&mut Runtime>, s: &JsiSymbol) -> UniquePtr<Self> {
        Value_copyFromSymbol(rt, s)
    }

    pub fn copy_string(rt: Pin<&mut Runtime>, s: &JsiString) -> UniquePtr<Self> {
        Value_copyFromString(rt, s)
    }

    pub fn from_json(rt: Pin<&mut Runtime>, json: &str) -> UniquePtr<Self> {
        Value_fromJson(rt, json)
    }

    pub fn as_object(&self, rt: Pin<&mut Runtime>) -> Result<UniquePtr<JsiObject>, cxx::Exception> {
        Value_asObject(self, rt)
    }

    pub fn as_symbol(&self, rt: Pin<&mut Runtime>) -> Result<UniquePtr<JsiSymbol>, cxx::Exception> {
        Value_asSymbol(self, rt)
    }

    pub fn as_string(&self, rt: Pin<&mut Runtime>) -> Result<UniquePtr<JsiString>, cxx::Exception> {
        Value_asString(self, rt)
    }

    pub fn to_string(&self, rt: Pin<&mut Runtime>) -> UniquePtr<JsiString> {
        Value_toString(self, rt)
    }
}

impl JsiWeakObject {
    pub fn from_object(rt: Pin<&mut Runtime>, object: &JsiObject) -> UniquePtr<Self> {
        WeakObject_fromObject(rt, object)
    }

    pub fn lock(self: Pin<&mut Self>, rt: Pin<&mut Runtime>) -> UniquePtr<JsiValue> {
        WeakObject_lock(self, rt)
    }
}
