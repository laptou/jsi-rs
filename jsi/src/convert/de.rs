use anyhow::Context;
use jsi::{
    IntoValue, JsiArray, JsiArrayBuffer, JsiFn, JsiObject, JsiString, JsiValue, JsiValueKind,
    PropName, RuntimeClone, RuntimeHandle,
};
use serde::{
    de::{IntoDeserializer, SeqAccess},
    Deserializer,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum JsiDeserializeError {
    #[error(transparent)]
    Native(#[from] cxx::Exception),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl serde::de::Error for JsiDeserializeError {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        JsiDeserializeError::Other(anyhow::anyhow!("{}", msg))
    }
}

/// Desrializes objects from JavaScript via JSI. Useful for transferring Rust
/// structures and objects from `serde_json` from JavaScript.
pub struct JsiDeserializer<'a, 'rt: 'a> {
    rt: &'a mut RuntimeHandle<'rt>,
    value: JsiValue<'rt>,
}

impl<'a, 'rt: 'a> JsiDeserializer<'a, 'rt> {
    pub fn new(value: JsiValue<'rt>, rt: &'a mut RuntimeHandle<'rt>) -> Self {
        Self { rt, value }
    }
}

impl<'a, 'rt: 'a, 'de> Deserializer<'de> for JsiDeserializer<'a, 'rt> {
    type Error = JsiDeserializeError;

    fn is_human_readable(&self) -> bool {
        true
    }

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        // trace!("deserialize_any: {}", self.value);

        if self.value.is_null() | self.value.is_undefined() {
            return self.deserialize_option(visitor);
        }

        if self.value.is_number() {
            return self.deserialize_f64(visitor);
        }

        if self.value.is_string() {
            return self.deserialize_string(visitor);
        }

        if self.value.is_object() {
            return self.deserialize_map(visitor);
        }

        if self.value.is_symbol() {
            return Err(anyhow::anyhow!("Symbols cannot be transferred to native code yet").into());
        }

        unreachable!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        // trace!("deserialize_bool: {}", self.value);

        let rt = self.rt;
        let val: bool = self
            .value
            .try_into_js(rt)
            .context("value was not a boolean")?;
        visitor.visit_bool(val)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        // trace!("deserialize_i8: {}", self.value);

        let rt = self.rt;
        let val: f64 = self
            .value
            .try_into_js(rt)
            .context("value was not a number")?;
        visitor.visit_i8(val as i8)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        // trace!("deserialize_i16: {}", self.value);

        let rt = self.rt;
        let val: f64 = self
            .value
            .try_into_js(rt)
            .context("value was not a number")?;
        visitor.visit_i16(val as i16)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        // trace!("deserialize_i32: {}", self.value);

        let rt = self.rt;
        let val: f64 = self
            .value
            .try_into_js(rt)
            .context("value was not a number")?;
        visitor.visit_i32(val as i32)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        // trace!("deserialize_i64: {}", self.value);

        let rt = self.rt;
        let val: f64 = self
            .value
            .try_into_js(rt)
            .context("value was not a number")?;
        visitor.visit_i64(val as i64)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        // trace!("deserialize_u8: {}", self.value);

        let rt = self.rt;
        let val: f64 = self
            .value
            .try_into_js(rt)
            .context("value was not a number")?;
        visitor.visit_u8(val as u8)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        // trace!("deserialize_u16: {}", self.value);

        let rt = self.rt;
        let val: f64 = self
            .value
            .try_into_js(rt)
            .context("value was not a number")?;
        visitor.visit_u16(val as u16)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        // trace!("deserialize_u32: {}", self.value);

        let rt = self.rt;
        let val: f64 = self
            .value
            .try_into_js(rt)
            .context("value was not a number")?;
        visitor.visit_u32(val as u32)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        // trace!("deserialize_u64: {}", self.value);

        let rt = self.rt;
        let val: f64 = self
            .value
            .try_into_js(rt)
            .context("value was not a number")?;
        visitor.visit_u64(val as u64)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        // trace!("deserialize_f32: {}", self.value);

        let rt = self.rt;
        let val: f64 = self
            .value
            .try_into_js(rt)
            .context("value was not a number")?;
        visitor.visit_f32(val as f32)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        // trace!("deserialize_f64: {}", self.value);

        let rt = self.rt;
        let val: f64 = self
            .value
            .try_into_js(rt)
            .context("value was not a number")?;
        visitor.visit_f64(val)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        // trace!("deserialize_char: {}", self.value);

        let rt = self.rt;
        let val: JsiString = self
            .value
            .try_into_js(rt)
            .context("the value is not a string")?;
        let val = rt.to_string(&val);
        visitor.visit_char(val.chars().next().unwrap())
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        // trace!("deserialize_str: {}", self.value);

        let rt = self.rt;
        let val: JsiString = self
            .value
            .try_into_js(rt)
            .context("the value is not a string")?;
        let val = rt.to_string(&val);
        visitor.visit_str(val.as_str())
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        // trace!("deserialize_string: {}", self.value);

        let rt = self.rt;
        let val: JsiString = self
            .value
            .try_into_js(rt)
            .context("the value is not a string")?;
        let val = rt.to_string(&val);
        visitor.visit_string(val)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        // trace!("deserialize_bytes: {}", self.value);
        let rt = self.rt;
        let value: JsiArrayBuffer = self
            .value
            .try_into_js(rt)
            .context("the value is not an array buffer")?;
        visitor.visit_bytes(value.data(rt))
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        // trace!("deserialize_byte_buf: {}", self.value);

        let rt = self.rt;

        let value: JsiArrayBuffer = self
            .value
            .try_into_js(rt)
            .context("the value is not an array buffer")?;

        visitor.visit_byte_buf(Vec::from(value.data(rt)))
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        // trace!("deserialize_option: {}", self.value);

        if self.value.is_null() | self.value.is_undefined() {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        // trace!("deserialize_unit: {}", self.value);

        let rt = self.rt;
        let mut value: JsiObject = self
            .value
            .try_into_js(rt)
            .context("expected empty object for unit")?;

        if value.properties(rt).len(rt) == 0 {
            visitor.visit_unit()
        } else {
            Err(anyhow::anyhow!("expected empty object for unit").into())
        }
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        // trace!("deserialize_unit_struct: {}", self.value);

        visitor.visit_unit()
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        // trace!("deserialize_newtype_struct: {}", self.value);

        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        // trace!("deserialize_seq: {}", self.value);

        let rt = self.rt;
        let obj: JsiObject = self
            .value
            .try_into_js(rt)
            .context("value is not an object; cannot deserialize as seq")?;

        let iterator: JsiObject = if obj.is_array(rt) {
            let values: JsiFn = obj
                .get(PropName::new("values", rt), rt)
                .try_into_js(rt)
                .context("Array.values is not an function")?;

            let iterator = values.call_with_this(&obj, std::iter::empty(), rt)?;
            iterator
                .try_into_js(rt)
                .context("Array.values returned a non-object")?
        } else {
            return Err(anyhow::anyhow!("cannot deserialize non-array sequences yet").into());
        };

        let next = iterator
            .get(PropName::new("next", rt), rt)
            .try_into_js(rt)
            .context("Iterator.next is not a function")?;

        visitor.visit_seq(JsiDeserializerSeqAccess {
            rt,
            next,
            this: iterator,
        })
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        // trace!("deserialize_tuple: {}", self.value);

        let rt = self.rt;

        let obj: JsiObject = self
            .value
            .try_into_js(rt)
            .context("value is not an object; cannot deserialize as tuple")?;

        visitor.visit_seq(JsiTupleVisitor {
            rt,
            obj,
            idx: 0,
            len,
        })
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        // trace!("deserialize_tuple_struct: {}", self.value);

        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        // trace!("deserialize_map: {}", self.value);

        let rt = self.rt;
        let mut obj: JsiObject = self
            .value
            .try_into_js(rt)
            .context("value is not an object; cannot deserialize as map/struct")?;

        let map_ctor = rt.global().get(PropName::new("Map", rt), rt);

        let is_map = obj.is_instance(
            map_ctor
                .try_into_js(rt)
                .context("global Map constructor is not a function")?,
            rt,
        );

        let iterator: JsiObject = if is_map {
            // trace!("deserialize_map: is_map = true");

            let entries: JsiFn = obj
                .get(PropName::new("entries", rt), rt)
                .try_into_js(rt)
                .context("Map.entries is not an function")?;

            let iterator = entries.call_with_this(&obj, std::iter::empty(), rt)?;
            iterator
                .try_into_js(rt)
                .context("Map.entries returned a non-object")?
        } else {
            // trace!("deserialize_map: is_map = false");

            let object_ctor: JsiObject = rt
                .global()
                .get(PropName::new("Object", rt), rt)
                .try_into_js(rt)
                .context("global Object constructor is not an object")?;

            let entries: JsiFn = object_ctor
                .get(PropName::new("entries", rt), rt)
                .try_into_js(rt)
                .context("Object.entries is not an function")?;

            let entries: JsiObject = entries
                .call(std::iter::once(obj.into_value(rt)), rt)?
                .try_into_js(rt)
                .context("Object.entries returned a non-object")?;

            // Object.entries() returns an array and not an iterator, so we need
            // one more step

            let values: JsiFn = entries
                .get(PropName::new("values", rt), rt)
                .try_into_js(rt)
                .context("Object.entries().values is not a function")?;

            values
                .call_with_this(&entries, std::iter::empty(), rt)?
                .try_into_js(rt)
                .context("Object.entries().values() returned a non-object")?
        };

        let next = iterator
            .get(PropName::new("next", rt), rt)
            .try_into_js(rt)
            .context("Iterator.next is not a function")?;

        visitor.visit_map(JsiDeserializerMapAccess {
            rt,
            value: None,
            next,
            this: iterator,
        })
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        // trace!("deserialize_struct: {}", self.value);

        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        // trace!("deserialize_enum: {}", self.value);

        match self.value.kind(self.rt) {
            JsiValueKind::Number(idx) => {
                let idx = idx as usize;
                let s = &variants[idx];
                visitor.visit_enum(s.into_deserializer())
            }
            JsiValueKind::String(s) => {
                let s = self.rt.to_string(&s);
                visitor.visit_enum(s.into_deserializer())
            }
            JsiValueKind::Object(mut obj) => {
                let props = obj.properties(self.rt);

                if props.len(self.rt) != 1 {
                    return Err(
                        anyhow::anyhow!("expected object with one property for enum").into(),
                    );
                }

                let variant: JsiString = props
                    .get(0, self.rt)
                    .try_into_js(self.rt)
                    .context("boops")?;

                let prop: PropName = PropName::from_string(variant.clone(self.rt), self.rt);

                let value = obj.get(prop, self.rt);

                visitor.visit_enum(JsiDeserializerEnumAccess {
                    variant: variant.into_value(self.rt),
                    rt: self.rt,
                    value,
                })
            }
            _ => Err(anyhow::anyhow!("invalid type to serialize into enum").into()),
        }
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        // trace!("deserialize_identifier: {}", self.value);

        self.deserialize_string(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        // trace!("deserialize_ignored_any: {}", self.value);

        self.deserialize_any(visitor)
    }
}

struct JsiTupleVisitor<'a, 'rt: 'a> {
    rt: &'a mut RuntimeHandle<'rt>,
    obj: JsiObject<'rt>,
    idx: usize,
    len: usize,
}

impl<'a, 'rt: 'a, 'de> SeqAccess<'de> for JsiTupleVisitor<'a, 'rt> {
    type Error = JsiDeserializeError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        let rt = &mut self.rt;

        if self.idx < self.len {
            let prop = PropName::new(self.idx.to_string().as_str(), rt);
            let value = self.obj.get(prop, rt);
            self.idx += 1;

            let value = seed.deserialize(JsiDeserializer { rt: self.rt, value })?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }
}

pub struct JsiDeserializerEnumAccess<'a, 'rt: 'a> {
    rt: &'a mut RuntimeHandle<'rt>,
    variant: JsiValue<'rt>,
    value: JsiValue<'rt>,
}

impl<'a, 'rt: 'a, 'de> serde::de::EnumAccess<'de> for JsiDeserializerEnumAccess<'a, 'rt> {
    type Error = JsiDeserializeError;

    type Variant = JsiDeserializerValueAccess<'a, 'rt>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        let rt = self.rt;

        let variant = JsiDeserializer {
            rt,
            value: self.variant,
        };

        seed.deserialize(variant).map(move |v| {
            (
                v,
                JsiDeserializerValueAccess {
                    rt,
                    value: self.value,
                },
            )
        })
    }
}

pub struct JsiDeserializerValueAccess<'a, 'rt: 'a> {
    rt: &'a mut RuntimeHandle<'rt>,
    value: JsiValue<'rt>,
}

impl<'a, 'rt: 'a, 'de> serde::de::VariantAccess<'de> for JsiDeserializerValueAccess<'a, 'rt> {
    type Error = JsiDeserializeError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        let rt = self.rt;
        let mut value: JsiObject = self
            .value
            .try_into_js(rt)
            .context("expected empty object for unit")?;

        if value.properties(rt).len(rt) == 0 {
            Ok(())
        } else {
            Err(anyhow::anyhow!("expected empty object for unit").into())
        }
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        seed.deserialize(JsiDeserializer {
            rt: self.rt,
            value: self.value,
        })
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let de = JsiDeserializer {
            rt: self.rt,
            value: self.value,
        };

        de.deserialize_tuple(len, visitor)
    }

    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let de = JsiDeserializer {
            rt: self.rt,
            value: self.value,
        };

        de.deserialize_struct("", fields, visitor)
    }
}

pub struct JsiDeserializerSeqAccess<'a, 'rt: 'a> {
    rt: &'a mut RuntimeHandle<'rt>,

    /// the JS `Iterator` being iterated over
    this: JsiObject<'rt>,

    /// `next()` function of JS `Iterator`
    next: JsiFn<'rt>,
}

impl<'a, 'rt: 'a, 'de> serde::de::SeqAccess<'de> for JsiDeserializerSeqAccess<'a, 'rt> {
    type Error = JsiDeserializeError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        let rt = &mut self.rt;
        let next = self
            .next
            .call_with_this(&self.this, std::iter::empty(), rt)?;
        let next: JsiObject = next
            .try_into_js(rt)
            .context("iterator returned a non-object value")?;

        let done = next.get(PropName::new("done", rt), rt);

        if !done.is_truthy(rt) {
            let item = next.get(PropName::new("value", rt), rt);

            let item = seed.deserialize(JsiDeserializer { rt, value: item })?;

            Ok(Some(item))
        } else {
            Ok(None)
        }
    }
}

pub struct JsiDeserializerMapAccess<'a, 'rt: 'a> {
    rt: &'a mut RuntimeHandle<'rt>,

    /// the JS `Iterator` being iterated over
    this: JsiObject<'rt>,

    /// `next()` function of JS `Iterator`, should yield map entries
    next: JsiFn<'rt>,

    value: Option<JsiValue<'rt>>,
}

impl<'a, 'rt: 'a, 'de> serde::de::MapAccess<'de> for JsiDeserializerMapAccess<'a, 'rt> {
    type Error = JsiDeserializeError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        let rt = &mut self.rt;

        let next = self
            .next
            .call_with_this(&self.this, std::iter::empty(), rt)?;
        let next: JsiObject = next
            .try_into_js(rt)
            .context("iterator returned a non-object value")?;

        let done = next.get(PropName::new("done", rt), rt);

        if !done.is_truthy(rt) {
            let entry = next.get(PropName::new("value", rt), rt);
            let entry: JsiArray = entry
                .try_into_js(rt)
                .context("iterator value is not an array")?;

            if entry.len(rt) != 2 {
                Err(anyhow::anyhow!("iterator value is not a 2-uple"))?;
            }

            let key = entry.get(0, rt);
            let value = entry.get(1, rt);

            self.value = Some(value);

            let key = seed.deserialize(JsiDeserializer { rt: rt, value: key })?;

            Ok(Some(key))
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        match self.value.take() {
            Some(value) => seed.deserialize(JsiDeserializer { rt: self.rt, value }),
            None => Err(anyhow::anyhow!("missing value").into()),
        }
    }
}
