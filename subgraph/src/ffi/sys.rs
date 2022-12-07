//! Subgraph host imports.
//!
//! This module just declares the "raw" host methods for WASM imports.

use super::{
    boxed::{AscBox, AscRef},
    buf::AscTypedArray,
    num::{AscBigDecimal, AscBigInt},
    str::{AscStr, AscString},
    value::{AscArray, AscJsonValue, AscResult, AscValueMap},
};

pub type AscUint8Array = AscTypedArray<u8>;
pub type AscByteArray = AscUint8Array;
pub type AscBytes = AscByteArray;
pub type AscAddress = AscBytes;

#[link(wasm_import_module = "index")]
extern "C" {
    #[link_name = "abort"]
    pub fn abort(
        message: *const AscStr,
        file_name: *const AscStr,
        line_number: u32,
        column_number: u32,
    ) -> !;

    #[link_name = "bigDecimal.toString"]
    pub fn big_decimal__to_string(data: *const AscRef<AscBigDecimal>) -> *const AscStr;

    #[link_name = "crypto.keccak256"]
    pub fn crypto__keccak256(data: *const AscRef<AscByteArray>) -> *const AscRef<AscByteArray>;

    #[link_name = "dataSource.address"]
    pub fn data_source__address() -> *const AscRef<AscAddress>;
    #[link_name = "dataSource.context"]
    pub fn data_source__context() -> *const AscRef<AscValueMap>;
    #[link_name = "dataSource.create"]
    pub fn data_source__create(name: *const AscStr, params: *const AscRef<AscArray<AscString>>);
    #[link_name = "dataSource.createWithContext"]
    pub fn data_source__create_with_context(
        name: *const AscStr,
        params: *const AscRef<AscArray<AscString>>,
        context: *const AscRef<AscValueMap>,
    );
    #[link_name = "dataSource.network"]
    pub fn data_source__network() -> *const AscStr;

    #[link_name = "json.fromBytes"]
    pub fn json__from_bytes(data: *const AscRef<AscBytes>) -> *const AscRef<AscJsonValue>;
    #[link_name = "json.toBigInt"]
    pub fn json__to_big_int(data: *const AscStr) -> *const AscRef<AscBigInt>;
    #[link_name = "json.toF64"]
    pub fn json__to_f64(data: *const AscStr) -> f64;
    #[link_name = "json.toI64"]
    pub fn json__to_i64(data: *const AscStr) -> i64;
    #[link_name = "json.toU64"]
    pub fn json__to_u64(data: *const AscStr) -> u64;
    #[link_name = "json.try_fromBytes"]
    pub fn json__try_from_bytes(
        data: *const AscRef<AscBytes>,
    ) -> *const AscRef<AscResult<AscBox<AscJsonValue>, bool>>;

    #[link_name = "log.log"]
    pub fn log__log(level: u32, message: *const AscStr);

    #[link_name = "typeConversion.bigIntToHex"]
    pub fn type_conversion__big_int_to_hex(big_int: *const AscRef<AscBigInt>) -> *const AscStr;
    #[link_name = "typeConversion.bigIntToString"]
    pub fn type_conversion__big_int_to_string(big_int: *const AscRef<AscBigInt>) -> *const AscStr;
    #[link_name = "typeConversion.bytesToBase58"]
    pub fn type_conversion__bytes_to_base58(bytes: *const AscRef<AscUint8Array>) -> *const AscStr;
    #[link_name = "typeConversion.bytesToHex"]
    pub fn type_conversion__bytes_to_hex(bytes: *const AscRef<AscUint8Array>) -> *const AscStr;
    #[link_name = "typeConversion.bytesToString"]
    pub fn type_conversion__bytes_to_string(bytes: *const AscRef<AscUint8Array>) -> *const AscStr;
    #[link_name = "typeConversion.stringToH160"]
    pub fn type_conversion__string_to_h160(bytes: *const AscStr) -> *const AscRef<AscUint8Array>;
}

/// List of linked imports for Ethereum:
/// - [x] abort
/// - [ ] bigDecimal.dividedBy
/// - [ ] bigDecimal.equals
/// - [ ] bigDecimal.fromString
/// - [ ] bigDecimal.minus
/// - [ ] bigDecimal.plus
/// - [ ] bigDecimal.times
/// - [x] bigDecimal.toString
/// - [ ] bigInt.bitAnd
/// - [ ] bigInt.bitOr
/// - [ ] bigInt.dividedBy
/// - [ ] bigInt.dividedByDecimal
/// - [ ] bigInt.fromString
/// - [ ] bigInt.leftShift
/// - [ ] bigInt.minus
/// - [ ] bigInt.mod
/// - [ ] bigInt.plus
/// - [ ] bigInt.pow
/// - [ ] bigInt.rightShift
/// - [ ] bigInt.times
/// - [x] crypto.keccak256
/// - [x] dataSource.address
/// - [x] dataSource.context
/// - [x] dataSource.create
/// - [x] dataSource.createWithContext
/// - [x] dataSource.network
/// - [ ] ens.nameByHash
/// - [ ] ethereum.decode
/// - [ ] ethereum.encode
/// - [ ] ipfs.cat
/// - [ ] ipfs.getBlock
/// - [ ] ipfs.map
/// - [x] json.fromBytes
/// - [x] json.toBigInt
/// - [x] json.toF64
/// - [x] json.toI64
/// - [x] json.toU64
/// - [x] json.try_fromBytes
/// - [x] log.log
/// - [ ] store.get
/// - [ ] store.remove
/// - [ ] store.set
/// - [x] typeConversion.bigIntToHex
/// - [x] typeConversion.bigIntToString
/// - [x] typeConversion.bytesToBase58
/// - [x] typeConversion.bytesToHex
/// - [x] typeConversion.bytesToString
/// - [x] typeConversion.stringToH160
mod missing {}
