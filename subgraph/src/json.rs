//! Subgraph JSON values.

use crate::ffi::{
    buf::AscTypedArray,
    sys,
    value::{AscJsonValue, AscJsonValueData},
};
use std::{
    borrow::Cow,
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};

pub use indexmap::IndexMap;

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
    fn from_raw(raw: &AscJsonValue) -> Self {
        match raw.data() {
            AscJsonValueData::Null => Self::Null,
            AscJsonValueData::Bool(value) => Self::Bool(value),
            AscJsonValueData::Number(value) => {
                Self::Number(Number(Cow::Owned(value.to_string_lossy())))
            }
            AscJsonValueData::String(value) => Self::String(value.to_string_lossy()),
            AscJsonValueData::Array(value) => {
                Self::Array(value.iter().copied().map(Self::from_raw).collect())
            }
            AscJsonValueData::Object(value) => Self::Object(
                value
                    .iter()
                    .copied()
                    .map(|entry| {
                        (
                            entry.key.to_string_lossy(),
                            Self::from_raw(entry.value.data()),
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
        let raw = unsafe { &*sys::json__from_bytes(array.data() as *const _) };

        Self::from_raw(raw)
    }

    /// Parses a new JSON value from bytes, returning and error on failure.
    pub fn try_from_bytes(bytes: impl AsRef<[u8]>) -> Result<Self, ParseError> {
        let bytes = bytes.as_ref();
        let array = AscTypedArray::from_bytes(bytes);
        let result = unsafe { &*sys::json__try_from_bytes(array.data() as *const _) };
        let raw = result.as_std_result().map_err(|_| ParseError)?.data();

        Ok(Self::from_raw(raw))
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

/// A JSON parse error.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ParseError;

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str("JSON parse error")
    }
}

impl Error for ParseError {}
