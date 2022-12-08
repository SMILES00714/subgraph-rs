//! Ethereum, in all its glory.
//!
//! TODO(nlordell): Type-safe `FixedBytes`, and `Array` values.

use self::ptr::*;
use crate::{
    address::Address,
    crypto::Hash,
    ffi::{
        boxed::{AscBox, AscRef},
        eth::{AscEthereumSmartContractCall, AscEventParam, AscTransaction},
        str::AscString,
        sys,
        types::AscBytes,
        value::{AscArray, AscEthereumValue, AscEthereumValueData},
    },
    num::BigInt,
};
use indexmap::IndexMap;

/// Re-exported pointer types used at handler entry points.
#[doc(hidden)]
pub mod ptr {
    pub use crate::ffi::eth::{AscBlock, AscCall, AscEvent};
}

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

/// Ethereum block data.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Block {
    pub hash: Hash,
    pub parent_hash: Hash,
    pub uncles_hash: Hash,
    pub author: Address,
    pub state_root: Hash,
    pub transactions_root: Hash,
    pub receipts_root: Hash,
    pub number: BigInt,
    pub gas_used: BigInt,
    pub gas_limit: BigInt,
    pub timestamp: BigInt,
    pub difficulty: BigInt,
    pub total_difficulty: BigInt,
    pub size: Option<BigInt>,
    pub base_fee_per_gas: Option<BigInt>,
}

impl Block {
    fn from_raw(b: &'static AscRef<AscBlock>) -> Self {
        Self {
            hash: b.hash().as_slice().try_into().unwrap(),
            parent_hash: b.parent_hash().as_slice().try_into().unwrap(),
            uncles_hash: b.uncles_hash().as_slice().try_into().unwrap(),
            author: Address::from_raw(b.author()),
            state_root: b.state_root().as_slice().try_into().unwrap(),
            transactions_root: b.transactions_root().as_slice().try_into().unwrap(),
            receipts_root: b.receipts_root().as_slice().try_into().unwrap(),
            number: BigInt::from_raw(b.number()),
            gas_used: BigInt::from_raw(b.gas_used()),
            gas_limit: BigInt::from_raw(b.gas_limit()),
            timestamp: BigInt::from_raw(b.timestamp()),
            difficulty: BigInt::from_raw(b.difficulty()),
            total_difficulty: BigInt::from_raw(b.total_difficulty()),
            size: b.size().map(BigInt::from_raw),
            base_fee_per_gas: b.base_fee_per_gas().map(BigInt::from_raw),
        }
    }

    /// Creates a block from a raw pointer.
    ///
    /// # Safety
    ///
    /// This must be a pointer passed into a block handler.
    pub unsafe fn from_ptr(ptr: *const AscBlock) -> Self {
        Self::from_raw(&*ptr.cast())
    }
}

/// An Ethereum transaction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Transaction {
    pub hash: Hash,
    pub index: BigInt,
    pub from: Address,
    pub to: Option<Address>,
    pub value: BigInt,
    pub gas_limit: BigInt,
    pub gas_price: BigInt,
    pub input: Vec<u8>,
    pub nonce: BigInt,
}

impl Transaction {
    fn from_raw(t: &'static AscTransaction) -> Self {
        Self {
            hash: t.hash().as_slice().try_into().unwrap(),
            index: BigInt::from_raw(t.index()),
            from: Address::from_raw(t.from()),
            to: t.to().map(Address::from_raw),
            value: BigInt::from_raw(t.value()),
            gas_limit: BigInt::from_raw(t.gas_limit()),
            gas_price: BigInt::from_raw(t.gas_price()),
            input: t.input().as_slice().to_owned(),
            nonce: BigInt::from_raw(t.nonce()),
        }
    }
}

/// Common representation for Ethereum smart contract calls.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Call {
    pub to: Address,
    pub from: Address,
    pub block: Block,
    pub transaction: Transaction,
    pub input_values: IndexMap<String, Value>,
    pub output_values: IndexMap<String, Value>,
}

impl Call {
    fn from_raw(c: &'static AscCall) -> Self {
        Self {
            to: Address::from_raw(c.to()),
            from: Address::from_raw(c.from()),
            block: Block::from_raw(c.block()),
            transaction: Transaction::from_raw(c.transaction()),
            input_values: params(c.input_values()),
            output_values: params(c.output_values()),
        }
    }

    /// Creates a call from a raw pointer.
    ///
    /// # Safety
    ///
    /// This must be a pointer passed into a call handler.
    pub unsafe fn from_ptr(ptr: *const AscCall) -> Self {
        Self::from_raw(&*ptr.cast())
    }
}

/// Common representation for Ethereum smart contract events.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Event {
    pub address: Address,
    pub log_index: BigInt,
    pub transaction_log_index: BigInt,
    pub log_type: Option<String>,
    pub block: Block,
    pub transaction: Transaction,
    pub parameters: IndexMap<String, Value>,
}

impl Event {
    fn from_raw(e: &'static AscEvent) -> Self {
        Self {
            address: Address::from_raw(e.address()),
            log_index: BigInt::from_raw(e.log_index()),
            transaction_log_index: BigInt::from_raw(e.transaction_log_index()),
            log_type: e
                .log_type()
                .map(|l: &crate::ffi::str::AscStr| l.to_string_lossy()),
            block: Block::from_raw(e.block()),
            transaction: Transaction::from_raw(e.transaction()),
            parameters: params(e.parameters()),
        }
    }

    /// Creates an event from a raw pointer.
    ///
    /// # Safety
    ///
    /// This must be a pointer passed into a call handler.
    pub unsafe fn from_ptr(ptr: *const AscEvent) -> Self {
        Self::from_raw(&*ptr.cast())
    }
}

/// Converts a vector of event parameters to an index map.
pub fn params(p: &'static AscRef<AscArray<AscBox<AscEventParam>>>) -> IndexMap<String, Value> {
    p.as_slice()
        .iter()
        .map(|p| {
            let p = p.as_asc_ref();
            (p.name().to_string_lossy(), Value::from_raw(p.value()))
        })
        .collect()
}

/// An Ethereum contract reference.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Contract<'a> {
    /// The name of the contract. This is used by the host for matching with an
    /// ABI for call encoding and decoding.
    pub name: &'a str,
    /// The contract address.
    pub address: &'a Address,
}

/// A contract fuction.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Function<'a> {
    /// The function name.
    pub name: &'a str,
    /// The function signature.
    pub signature: &'a str,
}

/// An Ethereum call.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
