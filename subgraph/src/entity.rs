//! Dynamic Subgraph values.

use crate::{
    ffi::{
        boxed::{AscBox, AscRef},
        buf::AscTypedArray,
        str::AscString,
        value::{AscArray, AscEntity, AscEntityValue, AscEntityValueData, AscMapEntry},
    },
    num::{BigDecimal, BigInt},
};
use indexmap::IndexMap;

/// A Subgraph entity value.
// FIXME(nlordell): Add missing derives.
//#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[derive(Clone, Debug, Default)]
pub enum Value {
    String(String),
    Int(i32),
    BigDecimal(BigDecimal),
    Bool(bool),
    Array(Vec<Value>),
    #[default]
    Null,
    Bytes(Vec<u8>),
    BigInt(BigInt),
}

/// A Subgraph entity.
pub type Entity = IndexMap<String, Value>;

impl Value {
    /// Creates a new instance from a raw Subgraph value.
    fn from_raw(raw: &'static AscRef<AscEntityValue>) -> Self {
        match raw.data() {
            AscEntityValueData::String(value) => Self::String(value.to_string_lossy()),
            AscEntityValueData::Int(value) => Self::Int(value),
            AscEntityValueData::BigDecimal(value) => Self::BigDecimal(BigDecimal::from_raw(value)),
            AscEntityValueData::Bool(value) => Self::Bool(value),
            AscEntityValueData::Array(value) => Self::Array(
                value
                    .as_slice()
                    .iter()
                    .map(|value| Value::from_raw(value.as_asc_ref()))
                    .collect(),
            ),
            AscEntityValueData::Null(()) => Self::Null,
            AscEntityValueData::Bytes(value) => Self::Bytes(value.as_slice().to_owned()),
            AscEntityValueData::BigInt(value) => Self::BigInt(BigInt::from_raw(value)),
        }
    }

    /// Creates a raw AssemblyScript value.
    fn to_raw(&self) -> AscBox<AscEntityValue> {
        match self {
            Value::String(value) => AscEntityValue::string(AscString::new(value)),
            Value::Int(value) => AscEntityValue::int(*value),
            Value::BigDecimal(value) => AscEntityValue::bigdecimal(value.as_raw().to_owned()),
            Value::Bool(value) => AscEntityValue::bool(*value),
            Value::Array(value) => {
                AscEntityValue::array(AscArray::new(value.iter().map(Value::to_raw).collect()))
            }
            Value::Null => AscEntityValue::null(()),
            Value::Bytes(value) => AscEntityValue::bytes(AscTypedArray::from_bytes(value)),
            Value::BigInt(value) => AscEntityValue::bigint(value.as_raw().to_owned()),
        }
    }
}

/// [`Entity`] extension trait.
pub(crate) trait EntityExt: Sized {
    /// Creates a new entity from a raw Subgraph key-value map.
    fn from_raw(raw: &'static AscRef<AscEntity>) -> Self;

    /// Creates a new map from a raw Subgraph key-value map.
    fn to_raw(&self) -> AscBox<AscEntity>;
}

impl EntityExt for Entity {
    fn from_raw(raw: &'static AscRef<AscEntity>) -> Self {
        raw.entries()
            .iter()
            .map(|entry| {
                let entry = entry.as_asc_ref();
                (
                    entry.key().to_string_lossy(),
                    Value::from_raw(entry.value().as_asc_ref()),
                )
            })
            .collect()
    }

    fn to_raw(&self) -> AscBox<AscEntity> {
        AscEntity::new(
            self.iter()
                .map(|(key, value)| AscMapEntry::new(AscString::new(key), value.to_raw()))
                .collect(),
        )
    }
}
