//! Ethereum, in all its glory.
//!
//! TODO(nlordell): Type-safe `FixedBytes`, and `Array` values.

use crate::{
    address::Address,
    ffi::{
        boxed::{AscBox, AscRef},
        eth::AscEthereumSmartContractCall,
        str::AscString,
        sys,
        types::AscBytes,
        value::{AscArray, AscEthereumValue, AscEthereumValueData},
    },
    num::BigInt,
};

/// Execute an Ethereum call.
pub fn call(call: SmartContractCall) -> Option<Vec<Value>> {
    let call = call.to_raw();
    let result = unsafe {
        let result = sys::ethereum__call(call.as_ptr());
        if result.is_null() {
            return None;
        }
        &*result
    };

    Some(
        result
            .as_slice()
            .iter()
            .map(|value| Value::from_raw(value.as_asc_ref()))
            .collect(),
    )
}

/// ABI-encode and Ethereum value.
pub fn encode(value: &Value) -> Option<Vec<u8>> {
    let value = value.to_raw();
    let data = unsafe {
        let data = sys::ethereum__encode(value.as_ptr());
        if data.is_null() {
            return None;
        }
        &*data
    };
    Some(data.as_slice().to_owned())
}

/// ABI-decode bytes for the specified signature.
pub fn decode(signature: impl AsRef<str>, data: impl AsRef<[u8]>) -> Option<Value> {
    let signature = AscString::new(signature.as_ref());
    let data = AscBytes::from_bytes(data.as_ref());
    let value = unsafe {
        let value = sys::ethereum__decode(signature.as_ptr(), data.as_ptr());
        if value.is_null() {
            return None;
        }
        &*value
    };
    Some(Value::from_raw(value))
}

/// An Ethereum value.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Value {
    Address(Address),
    FixedBytes(Vec<u8>),
    Bytes(Vec<u8>),
    Int(BigInt),
    Uint(BigInt),
    Bool(bool),
    String(String),
    FixedArray(Vec<Value>),
    Array(Vec<Value>),
    Tuple(Vec<Value>),
}

impl Value {
    /// Creates a new instance from a raw value.
    fn from_raw(raw: &'static AscRef<AscEthereumValue>) -> Self {
        match raw.data() {
            AscEthereumValueData::Address(value) => Self::Address(Address::from_raw(value)),
            AscEthereumValueData::FixedBytes(value) => {
                Self::FixedBytes(value.as_slice().to_owned())
            }
            AscEthereumValueData::Bytes(value) => Self::Bytes(value.as_slice().to_owned()),
            AscEthereumValueData::Int(value) => Self::Int(BigInt::from_raw(value)),
            AscEthereumValueData::Uint(value) => Self::Uint(BigInt::from_raw(value)),
            AscEthereumValueData::Bool(value) => Self::Bool(value),
            AscEthereumValueData::String(value) => Self::String(value.to_string_lossy()),
            AscEthereumValueData::FixedArray(value) => Self::FixedArray(
                value
                    .as_slice()
                    .iter()
                    .map(|value| Value::from_raw(value.as_asc_ref()))
                    .collect(),
            ),
            AscEthereumValueData::Array(value) => Self::Array(
                value
                    .as_slice()
                    .iter()
                    .map(|value| Value::from_raw(value.as_asc_ref()))
                    .collect(),
            ),
            AscEthereumValueData::Tuple(value) => Self::Tuple(
                value
                    .as_slice()
                    .iter()
                    .map(|value| Value::from_raw(value.as_asc_ref()))
                    .collect(),
            ),
        }
    }

    /// Creates a raw AssemblyScript value.
    fn to_raw(&self) -> AscBox<AscEthereumValue> {
        match self {
            Self::Address(value) => AscEthereumValue::address(value.to_raw()),
            Self::FixedBytes(value) => AscEthereumValue::fixedbytes(AscBytes::from_bytes(value)),
            Self::Bytes(value) => AscEthereumValue::bytes(AscBytes::from_bytes(value)),
            Self::Int(value) => AscEthereumValue::int(value.as_raw().to_owned()),
            Self::Uint(value) => AscEthereumValue::uint(value.as_raw().to_owned()),
            Self::Bool(value) => AscEthereumValue::bool(*value),
            Self::String(value) => AscEthereumValue::string(AscString::new(value)),
            Self::FixedArray(value) => AscEthereumValue::fixedarray(AscArray::new(
                value.iter().map(|value| value.to_raw()).collect(),
            )),
            Self::Array(value) => AscEthereumValue::array(AscArray::new(
                value.iter().map(|value| value.to_raw()).collect(),
            )),
            Self::Tuple(value) => AscEthereumValue::tuple(AscArray::new(
                value.iter().map(|value| value.to_raw()).collect(),
            )),
        }
    }
}

/// An Ethereum contract reference.
#[derive(Clone, Copy, Debug)]
pub struct Contract<'a> {
    /// The name of the contract. This is used by the host for matching with an
    /// ABI for call encoding and decoding.
    pub name: &'a str,
    /// The contract address.
    pub address: &'a Address,
}

/// A contract fuction.
#[derive(Clone, Copy, Debug)]
pub struct Function<'a> {
    /// The function name.
    pub name: &'a str,
    /// The function signature.
    pub signature: &'a str,
}

/// An Ethereum call.
#[derive(Clone, Copy, Debug)]
pub struct SmartContractCall<'a> {
    pub contract: Contract<'a>,
    pub function: Function<'a>,
    pub params: &'a [Value],
}

impl SmartContractCall<'_> {
    /// Converts a call into an [`AscEthereumSmartContractCall`].
    fn to_raw(self) -> AscBox<AscEthereumSmartContractCall> {
        AscEthereumSmartContractCall::new(
            AscString::new(self.contract.name),
            self.contract.address.to_raw(),
            AscString::new(self.function.name),
            AscString::new(self.function.signature),
            AscArray::new(self.params.iter().map(|value| value.to_raw()).collect()),
        )
    }
}
