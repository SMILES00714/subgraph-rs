//! Subgraph host imports.
//!
//! This module just declares the "raw" host methods for WASM imports.

use super::{boxed::AscValue, buf::AscTypedArray, str::AscStr, value::AscJsonValue};

pub type AscByteArray = AscValue<AscTypedArray<u8>>;
pub type AscBytes = AscByteArray;
pub type AscBigInt = AscBytes;

#[link(wasm_import_module = "index")]
extern "C" {
    #[link_name = "abort"]
    pub fn abort(
        message: *const AscStr,
        file_name: *const AscStr,
        line_number: u32,
        column_number: u32,
    ) -> !;

    #[link_name = "crypto.keccak256"]
    pub fn crypto__keccak256(data: *const AscByteArray) -> *const AscByteArray;

    #[link_name = "json.fromBytes"]
    pub fn json__from_bytes(data: *const AscBytes) -> *const AscValue<AscJsonValue>;

    #[link_name = "log.log"]
    pub fn log__log(level: u32, message: *const AscStr);

    #[link_name = "typeConversion.bigIntToHex"]
    pub fn type_conversion__big_int_to_hex(big_int: *const AscBigInt) -> *const AscStr;
    #[link_name = "typeConversion.bigIntToString"]
    pub fn type_conversion__big_int_to_string(big_int: *const AscBigInt) -> *const AscStr;
}

/// List of linked imports for Ethereum:
/// - [x] abort
/// - [ ] bigDecimal.dividedBy
/// - [ ] bigDecimal.equals
/// - [ ] bigDecimal.fromString
/// - [ ] bigDecimal.minus
/// - [ ] bigDecimal.plus
/// - [ ] bigDecimal.times
/// - [ ] bigDecimal.toString
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
/// - [ ] crypto.keccak256
/// - [ ] dataSource.address
/// - [ ] dataSource.context
/// - [ ] dataSource.create
/// - [ ] dataSource.createWithContext
/// - [ ] dataSource.network
/// - [ ] ens.nameByHash
/// - [ ] ethereum.decode
/// - [ ] ethereum.encode
/// - [ ] ipfs.cat
/// - [ ] ipfs.getBlock
/// - [ ] ipfs.map
/// - [x] json.fromBytes
/// - [ ] json.toBigInt
/// - [ ] json.toF64
/// - [ ] json.toI64
/// - [ ] json.toU64
/// - [ ] json.try_fromBytes
/// - [x] log.log
/// - [ ] store.get
/// - [ ] store.remove
/// - [ ] store.set
/// - [x] typeConversion.bigIntToHex
/// - [x] typeConversion.bigIntToString
/// - [ ] typeConversion.bytesToBase58
/// - [ ] typeConversion.bytesToHex
/// - [ ] typeConversion.bytesToString
/// - [ ] typeConversion.stringToH160
pub mod missing {}
