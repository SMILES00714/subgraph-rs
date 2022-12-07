//! Subgraph JSON values.

use crate::{
    ffi::{
        boxed::AscRef,
        buf::AscTypedArray,
        str::AscString,
        sys,
        value::{AscJsonValue, AscJsonValueData},
    },
    num::BigInt,
};
use indexmap::IndexMap;
use std::{
    borrow::Cow,
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    str::FromStr,
};

/// A Subgraph JSON value.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum Value {
    #[default]
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<Value>),
    Object(IndexMap<String, Value>),
}

impl Value {
    /// Creates a new instance from a raw JSON value.
    fn from_raw(raw: &AscRef<AscJsonValue>) -> Self {
        match raw.data() {
            AscJsonValueData::Null(()) => Self::Null,
            AscJsonValueData::Bool(value) => Self::Bool(value),
            AscJsonValueData::Number(value) => {
                Self::Number(Number(Cow::Owned(value.to_string_lossy())))
            }
            AscJsonValueData::String(value) => Self::String(value.to_string_lossy()),
            AscJsonValueData::Array(value) => Self::Array(
                value
                    .as_slice()
                    .iter()
                    .map(|value| Self::from_raw(value.as_asc_ref()))
                    .collect(),
            ),
            AscJsonValueData::Object(value) => Self::Object(
                value
                    .entries()
                    .iter()
                    .map(|entry| {
                        let entry = entry.as_asc_ref();
                        (
                            entry.key().to_string_lossy(),
                            Self::from_raw(entry.value().as_asc_ref()),
                        )
                    })
                    .collect(),
            ),
        }
    }

    /// Parses a new JSON from from some bytes.
    pub fn from_bytes(bytes: impl AsRef<[u8]>) -> Self {
        let bytes = bytes.as_ref();
        let array = AscTypedArray::from_bytes(bytes);
        let raw = unsafe { &*sys::json__from_bytes(array.as_ptr()) };

        Self::from_raw(raw)
    }

    /// Parses a new JSON value from bytes, returning and error on failure.
    pub fn try_from_bytes(bytes: impl AsRef<[u8]>) -> Result<Self, ParseError> {
        let bytes = bytes.as_ref();
        let array = AscTypedArray::from_bytes(bytes);
        let result = unsafe { &*sys::json__try_from_bytes(array.as_ptr()) };
        let raw = result.as_std_result().map_err(|_| ParseError)?.as_asc_ref();

        Ok(Self::from_raw(raw))
    }

    /// Returns the JSON value as a unit value, or `None` if the value is not
    /// `null`.
    pub fn as_null(&self) -> Option<()> {
        match self {
            Value::Null => Some(()),
            _ => None,
        }
    }

    /// Returns the JSON value as a boolean value, or `None` if the value is not
    /// `true` or `false`.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns the JSON value as a numeric value, or `None` if the value is not
    /// a number.
    pub fn as_number(&self) -> Option<&Number> {
        match self {
            Value::Number(value) => Some(value),
            _ => None,
        }
    }

    /// Returns the JSON value as a string value, or `None` if the value is not
    /// a string.
    pub fn as_string(&self) -> Option<&str> {
        match self {
            Value::String(value) => Some(value),
            _ => None,
        }
    }

    /// Returns the JSON value as a slice of values, or `None` if the value is
    /// not an array.
    pub fn as_array(&self) -> Option<&[Value]> {
        match self {
            Value::Array(value) => Some(value),
            _ => None,
        }
    }

    /// Returns the JSON value as a map of values, or `None` if the value is not
    /// an object.
    pub fn as_object(&self) -> Option<&IndexMap<String, Value>> {
        match self {
            Value::Object(value) => Some(value),
            _ => None,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Null => f.write_str("null"),
            Self::Bool(value) => write!(f, "{value}"),
            Self::Number(value) => write!(f, "{value}"),
            Self::String(value) => write!(f, "{value:?}"),
            Self::Array(value) => {
                f.write_str("[")?;
                for (i, value) in value.iter().enumerate() {
                    if i > 0 {
                        f.write_str(",")?;
                    }
                    write!(f, "{value}")?;
                }
                f.write_str("]")
            }
            Self::Object(value) => {
                f.write_str("{")?;
                for (i, (key, value)) in value.iter().enumerate() {
                    if i > 0 {
                        f.write_str(",")?;
                    }
                    write!(f, "\"{key}\":{value}")?;
                }
                f.write_str("}")
            }
        }
    }
}

/// A arbitrary-precision JSON number.
#[derive(Clone, Eq, PartialEq)]
pub struct Number(Cow<'static, str>);

impl Number {
    /// Converts this number to a [`BigInt`].
    pub fn to_big_int(&self) -> BigInt {
        let str = AscString::new(&self.0);
        let raw = unsafe { &*sys::json__to_big_int(str.as_ptr()) };
        BigInt::from_raw(raw)
    }

    /// Converts this number to a 64-bit float.
    pub fn to_f64(&self) -> f64 {
        let str = AscString::new(&self.0);
        unsafe { sys::json__to_f64(str.as_ptr()) }
    }

    /// Converts this number to a 64-bit signed integer.
    pub fn to_i64(&self) -> i64 {
        let str = AscString::new(&self.0);
        unsafe { sys::json__to_i64(str.as_ptr()) }
    }

    /// Converts this number to a 64-bit un-signed integer.
    pub fn to_u64(&self) -> u64 {
        let str = AscString::new(&self.0);
        unsafe { sys::json__to_u64(str.as_ptr()) }
    }
}

impl Debug for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl Default for Number {
    fn default() -> Self {
        Self(Cow::Borrowed("0"))
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for Value {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from_bytes(s)
    }
}

/// A JSON parse error.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ParseError;

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str("JSON parse error")
    }
}

impl Error for ParseError {}
