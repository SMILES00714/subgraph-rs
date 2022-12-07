//! Dynamic Subgraph values.

use crate::{
    ffi::{
        boxed::{AscBox, AscRef},
        buf::AscTypedArray,
        str::AscString,
        value::{AscArray, AscMapEntry, AscValue, AscValueData, AscValueMap},
    },
    num::{BigDecimal, BigInt},
};
use indexmap::IndexMap;

/// A Subgraph value.
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

/// A Subgraph value map.
pub type Map = IndexMap<String, Value>;

impl Value {
    /// Creates a new instance from a raw Subgraph value.
    fn from_raw(raw: &'static AscRef<AscValue>) -> Self {
        match raw.data() {
            AscValueData::String(value) => Self::String(value.to_string_lossy()),
            AscValueData::Int(value) => Self::Int(value),
            AscValueData::BigDecimal(value) => Self::BigDecimal(BigDecimal::from_raw(value)),
            AscValueData::Bool(value) => Self::Bool(value),
            AscValueData::Array(value) => Self::Array(
                value
                    .as_slice()
                    .iter()
                    .map(|value| Value::from_raw(value.as_asc_ref()))
                    .collect(),
            ),
            AscValueData::Null => Self::Null,
            AscValueData::Bytes(value) => Self::Bytes(value.as_slice().to_owned()),
            AscValueData::BigInt(value) => Self::BigInt(BigInt::from_raw(value)),
        }
    }

    /// Creates a raw AssemblyScript value.
    fn to_raw(&self) -> AscBox<AscValue> {
        match self {
            Value::String(value) => AscValue::string(AscString::new(value)),
            Value::Int(value) => AscValue::int(*value),
            Value::BigDecimal(value) => AscValue::bigdecimal(value.as_raw().to_owned()),
            Value::Bool(value) => AscValue::bool(*value),
            Value::Array(value) => {
                AscValue::array(AscArray::new(value.iter().map(Value::to_raw).collect()))
            }
            Value::Null => AscValue::null(),
            Value::Bytes(value) => AscValue::bytes(AscTypedArray::from_bytes(value)),
            Value::BigInt(value) => AscValue::bigint(value.as_raw().to_owned()),
        }
    }
}

/// [`Map`] extension trait.
pub trait MapExt: Sized {
    /// Creates a new map from a raw Subgraph key-value map.
    fn from_raw(raw: &'static AscRef<AscValueMap>) -> Self;

    /// Creates a new map from a raw Subgraph key-value map.
    fn to_raw(&self) -> AscBox<AscValueMap>;
}

impl MapExt for Map {
    fn from_raw(raw: &'static AscRef<AscValueMap>) -> Self {
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

    fn to_raw(&self) -> AscBox<AscValueMap> {
        AscValueMap::new(
            self.iter()
                .map(|(key, value)| AscMapEntry::new(AscString::new(key), value.to_raw()))
                .collect(),
        )
    }
}
