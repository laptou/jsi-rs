use jsi::*;
use serde::{
    ser::{
        SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
        SerializeTupleStruct, SerializeTupleVariant,
    },
    Serializer,
};

/// Serializes objects into JavaScript via JSI. Useful for transferring Rust
/// structures and objects from `serde_json` into JavaScript.
pub struct JsiSerializer<'a, 'rt: 'a> {
    rt: &'a mut RuntimeHandle<'rt>,
}

impl<'a, 'rt: 'a> JsiSerializer<'a, 'rt> {
    pub fn new(rt: &'a mut RuntimeHandle<'rt>) -> Self {
        Self { rt }
    }
}

impl<'a, 'rt: 'a> Serializer for JsiSerializer<'a, 'rt> {
    type Ok = JsiValue<'rt>;
    type Error = JsiSerializeError;

    type SerializeSeq = JsiSeqSerializer<'a, 'rt>;
    type SerializeTuple = JsiTupleSerializer<'a, 'rt>;
    type SerializeTupleStruct = JsiTupleVariantSerializer<'a, 'rt>;
    type SerializeTupleVariant = JsiTupleVariantSerializer<'a, 'rt>;
    type SerializeMap = JsiMapSerializer<'a, 'rt>;
    type SerializeStruct = JsiStructSerializer<'a, 'rt>;
    type SerializeStructVariant = JsiStructVariantSerializer<'a, 'rt>;

    fn is_human_readable(&self) -> bool {
        true
    }

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(JsiValue::new_bool(v))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(JsiValue::new_number(v as f64))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(JsiValue::new_number(v as f64))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(JsiValue::new_number(v as f64))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(JsiValue::new_number(v as f64))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(JsiValue::new_number(v as f64))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(JsiValue::new_number(v as f64))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(JsiValue::new_number(v as f64))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(JsiValue::new_number(v as f64))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(JsiValue::new_number(v as f64))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(JsiValue::new_number(v))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(JsiString::new(v.encode_utf8(&mut [0; 4]), self.rt).into_value(self.rt))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(JsiString::new(v, self.rt).into_value(self.rt))
    }

    fn serialize_bytes(mut self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        let rt = &mut self.rt;
        let array_buffer_ctor = rt.global().get(PropName::new("ArrayBuffer", rt), rt);
        let array_buffer_ctor: JsiFn = array_buffer_ctor
            .try_into_js(rt)
            .expect("ArrayBuffer constructor is not a function");
        let array_buffer = array_buffer_ctor
            .call_as_constructor(vec![JsiValue::new_number(v.len() as f64)], rt)
            .expect("ArrayBuffer constructor threw an exception");
        let array_buffer: JsiArrayBuffer = array_buffer
            .try_into_js(rt)
            .expect("ArrayBuffer constructor did not return an ArrayBuffer");

        array_buffer.data(rt).copy_from_slice(v);

        Ok(array_buffer.into_value(rt))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(JsiValue::new_null())
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(JsiObject::new(self.rt).into_value(self.rt))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        match len {
            Some(len) => Ok(JsiSeqSerializer::new(len, self.rt)),
            None => Err(JsiSerializeError::UnsizedSequence),
        }
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(JsiTupleSerializer::new(self.rt))
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(JsiTupleVariantSerializer::new(name, self.rt))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(JsiTupleVariantSerializer::new(variant, self.rt))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(JsiMapSerializer::new(self.rt))
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(JsiStructSerializer::new(self.rt))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(JsiStructVariantSerializer::new(variant, self.rt))
    }
}

pub struct JsiSeqSerializer<'a, 'rt: 'a> {
    rt: &'a mut RuntimeHandle<'rt>,
    arr: JsiArray<'rt>,
    idx: usize,
}

impl<'a, 'rt: 'a> JsiSeqSerializer<'a, 'rt> {
    pub fn new(len: usize, rt: &'a mut RuntimeHandle<'rt>) -> Self {
        let arr = JsiArray::new(len, rt);
        Self { rt, arr, idx: 0 }
    }
}

impl<'a, 'rt: 'a> SerializeSeq for JsiSeqSerializer<'a, 'rt> {
    type Ok = JsiValue<'rt>;
    type Error = JsiSerializeError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.arr.set(
            self.idx,
            &value.serialize(JsiSerializer { rt: self.rt })?,
            self.rt,
        );
        self.idx += 1;

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.arr.into_value(self.rt))
    }
}

pub struct JsiTupleSerializer<'a, 'rt: 'a> {
    rt: &'a mut RuntimeHandle<'rt>,
    obj: JsiObject<'rt>,
    idx: usize,
}

impl<'a, 'rt: 'a> JsiTupleSerializer<'a, 'rt> {
    pub fn new(rt: &'a mut RuntimeHandle<'rt>) -> Self {
        let obj = JsiObject::new(rt);
        Self { rt, obj, idx: 0 }
    }
}

impl<'a, 'rt: 'a> SerializeTuple for JsiTupleSerializer<'a, 'rt> {
    type Ok = JsiValue<'rt>;
    type Error = JsiSerializeError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.obj.set(
            PropName::new(self.idx.to_string().as_str(), self.rt),
            &value.serialize(JsiSerializer { rt: self.rt })?,
            self.rt,
        );
        self.idx += 1;

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.obj.into_value(self.rt))
    }
}

pub struct JsiTupleVariantSerializer<'a, 'rt: 'a> {
    inner: JsiTupleSerializer<'a, 'rt>,
    variant: &'static str,
}

impl<'a, 'rt: 'a> JsiTupleVariantSerializer<'a, 'rt> {
    pub fn new(variant: &'static str, rt: &'a mut RuntimeHandle<'rt>) -> Self {
        Self {
            inner: JsiTupleSerializer::new(rt),
            variant,
        }
    }
}

impl<'a, 'rt: 'a> SerializeTupleVariant for JsiTupleVariantSerializer<'a, 'rt> {
    type Ok = JsiValue<'rt>;
    type Error = JsiSerializeError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.inner.serialize_element(value)
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.inner.obj.set(
            PropName::new("__variant", self.inner.rt),
            &JsiValue::new_string(self.variant, self.inner.rt),
            self.inner.rt,
        );
        self.inner.end()
    }
}

impl<'a, 'rt: 'a> SerializeTupleStruct for JsiTupleVariantSerializer<'a, 'rt> {
    type Ok = JsiValue<'rt>;
    type Error = JsiSerializeError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.inner.serialize_element(value)
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.inner.obj.set(
            PropName::new("__name", self.inner.rt),
            &JsiValue::new_string(self.variant, self.inner.rt),
            self.inner.rt,
        );
        self.inner.end()
    }
}

pub struct JsiMapSerializer<'a, 'rt: 'a> {
    rt: &'a mut RuntimeHandle<'rt>,
    map: JsiObject<'rt>,
    setter: JsiFn<'rt>,
    current_key: Option<JsiValue<'rt>>,
}

impl<'a, 'rt: 'a> JsiMapSerializer<'a, 'rt> {
    pub fn new(rt: &'a mut RuntimeHandle<'rt>) -> Self {
        let map_ctor = rt.global().get(PropName::new("Map", rt), rt);
        let map_ctor: JsiFn = map_ctor
            .try_into_js(rt)
            .expect("Map constructor is not a function");
        let map = map_ctor
            .call_as_constructor(vec![], rt)
            .expect("Map constructor threw an exception");
        let map: JsiObject = map
            .try_into_js(rt)
            .expect("Map constructor did not return an object");
        let map_setter = map.get(PropName::new("set", rt), rt);
        let map_setter: JsiFn = map_setter
            .try_into_js(rt)
            .expect("Map.set is not an function");

        Self {
            rt,
            map,
            setter: map_setter,
            current_key: None,
        }
    }
}

impl<'a, 'rt: 'a> SerializeMap for JsiMapSerializer<'a, 'rt> {
    type Ok = JsiValue<'rt>;
    type Error = JsiSerializeError;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        let ser = JsiSerializer { rt: self.rt };
        let key = key.serialize(ser)?;
        self.current_key = Some(key);
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        let value = value.serialize(JsiSerializer { rt: self.rt })?;
        let key = self
            .current_key
            .take()
            .expect("tried to serialize value without serializing key first");

        self.setter
            .call_with_this(&self.map, vec![key, value], self.rt)
            .expect("Map.set threw an exception");

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.map.into_value(self.rt))
    }
}

pub struct JsiStructSerializer<'a, 'rt: 'a> {
    rt: &'a mut RuntimeHandle<'rt>,
    obj: JsiObject<'rt>,
}

impl<'a, 'rt: 'a> JsiStructSerializer<'a, 'rt> {
    pub fn new(rt: &'a mut RuntimeHandle<'rt>) -> Self {
        let obj = JsiObject::new(rt);
        Self { rt, obj }
    }
}

impl<'a, 'rt: 'a> SerializeStruct for JsiStructSerializer<'a, 'rt> {
    type Ok = JsiValue<'rt>;
    type Error = JsiSerializeError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.obj.set(
            PropName::new(key, self.rt),
            &value.serialize(JsiSerializer { rt: self.rt })?,
            self.rt,
        );
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.obj.into_value(self.rt))
    }
}

pub struct JsiStructVariantSerializer<'a, 'rt: 'a> {
    inner: JsiStructSerializer<'a, 'rt>,
    variant: &'static str,
}

impl<'a, 'rt: 'a> JsiStructVariantSerializer<'a, 'rt> {
    pub fn new(variant: &'static str, rt: &'a mut RuntimeHandle<'rt>) -> Self {
        Self {
            inner: JsiStructSerializer::new(rt),
            variant,
        }
    }
}

impl<'a, 'rt: 'a> SerializeStructVariant for JsiStructVariantSerializer<'a, 'rt> {
    type Ok = JsiValue<'rt>;
    type Error = JsiSerializeError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.inner.serialize_field(key, value)
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.inner.obj.set(
            PropName::new("__variant", self.inner.rt),
            &JsiValue::new_string(self.variant, self.inner.rt),
            self.inner.rt,
        );
        self.inner.end()
    }
}

#[derive(Debug)]
pub enum JsiSerializeError {
    Custom(String),
    UnsizedSequence,
}

impl std::fmt::Display for JsiSerializeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsiSerializeError::Custom(s) => f.write_str(s.as_str()),
            JsiSerializeError::UnsizedSequence => f.write_str("cannot serialize unsized sequences"),
        }
    }
}

impl serde::ser::Error for JsiSerializeError {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::Custom(msg.to_string())
    }
}

impl std::error::Error for JsiSerializeError {}
