pub mod de;
pub mod ser;

pub use de::JsiDeserializer;
pub use ser::JsiSerializer;

use jsi::{JsiValue, RuntimeHandle};
use serde::{Deserialize, Serialize};

pub trait SerializeValue {
    fn serialize_value<'rt>(
        &self,
        rt: &mut RuntimeHandle<'rt>,
    ) -> Result<JsiValue<'rt>, ser::JsiSerializeError>;
}

impl<T: Serialize> SerializeValue for T {
    fn serialize_value<'rt>(
        &self,
        rt: &mut RuntimeHandle<'rt>,
    ) -> Result<JsiValue<'rt>, ser::JsiSerializeError> {
        T::serialize(&self, JsiSerializer::new(rt))
    }
}

pub trait DeserializeValue: Sized {
    fn deserialize_value<'rt>(
        val: JsiValue<'rt>,
        rt: &mut RuntimeHandle<'rt>,
    ) -> Result<Self, de::JsiDeserializeError>;
}

impl<'de, T: Deserialize<'de>> DeserializeValue for T {
    fn deserialize_value<'rt>(
        val: JsiValue<'rt>,
        rt: &mut RuntimeHandle<'rt>,
    ) -> Result<Self, de::JsiDeserializeError> {
        Self::deserialize(JsiDeserializer::new(val, rt))
    }
}
