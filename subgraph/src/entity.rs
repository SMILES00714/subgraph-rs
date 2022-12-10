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
#[derive(Clone, Debug, Default, Eq, PartialEq)]
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
    pub(crate) fn from_raw(raw: &'static AscRef<AscEntityValue>) -> Self {
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
    pub(crate) fn to_raw(&self) -> AscBox<AscEntityValue> {
        match self {
            Self::String(value) => AscEntityValue::string(AscString::new(value)),
            Self::Int(value) => AscEntityValue::int(*value),
            Self::BigDecimal(value) => AscEntityValue::bigdecimal(value.as_raw().to_owned()),
            Self::Bool(value) => AscEntityValue::bool(*value),
            Self::Array(value) => {
                AscEntityValue::array(AscArray::new(value.iter().map(Value::to_raw).collect()))
            }
            Self::Null => AscEntityValue::null(()),
            Self::Bytes(value) => AscEntityValue::bytes(AscTypedArray::from_bytes(value)),
            Self::BigInt(value) => AscEntityValue::bigint(value.as_raw().to_owned()),
        }
    }

    /// Returns the entity value as a string, or `None` if the value the wrong
    /// type.
    pub fn as_string(&self) -> Option<&str> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }

    /// Returns the entity value as an int, or `None` if the value the wrong
    /// type.
    pub fn as_int(&self) -> Option<i32> {
        match self {
            Self::Int(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns the entity value as a big decimal, or `None` if the value the
    /// wrong type.
    pub fn as_big_decimal(&self) -> Option<&BigDecimal> {
        match self {
            Self::BigDecimal(value) => Some(value),
            _ => None,
        }
    }

    /// Returns the entity value as a bool, or `None` if the value the wrong
    /// type.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns the entity value as an array, or `None` if the value the wrong
    /// type.
    pub fn as_array(&self) -> Option<&[Value]> {
        match self {
            Self::Array(value) => Some(value),
            _ => None,
        }
    }

    /// Returns the entity value as null, or `None` if the value the wrong type.
    pub fn as_null(&self) -> Option<()> {
        match self {
            Self::Null => Some(()),
            _ => None,
        }
    }

    /// Returns the entity value as bytes, or `None` if the value the wrong
    /// type.
    pub fn as_bytes(&self) -> Option<&Vec<u8>> {
        match self {
            Self::Bytes(value) => Some(value),
            _ => None,
        }
    }

    /// Returns the entity value as a big int, or `None` if the value the wrong
    /// type.
    pub fn as_big_int(&self) -> Option<&BigInt> {
        match self {
            Self::BigInt(value) => Some(value),
            _ => None,
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
