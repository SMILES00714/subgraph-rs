//! Subgraph host imports.
//!
//! This module just declares the "raw" host methods for WASM imports.

use super::boxed::AscSlicePtr;

#[link(wasm_import_module = "index")]
extern "C" {
    #[link_name = "abort"]
    pub fn abort(
        message: AscSlicePtr<u16>,
        file_name: AscSlicePtr<u16>,
        line_number: u32,
        column_number: u32,
    ) -> !;

    #[link_name = "log.log"]
    pub fn log(level: u32, message: AscSlicePtr<u16>);
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
/// - [ ] json.fromBytes
/// - [ ] json.toBigInt
/// - [ ] json.toF64
/// - [ ] json.toI64
/// - [ ] json.toU64
/// - [ ] json.try_fromBytes
/// - [x] log.log
/// - [ ] store.get
/// - [ ] store.remove
/// - [ ] store.set
/// - [ ] typeConversion.bigIntToHex
/// - [ ] typeConversion.bigIntToString
/// - [ ] typeConversion.bytesToBase58
/// - [ ] typeConversion.bytesToHex
/// - [ ] typeConversion.bytesToString
/// - [ ] typeConversion.stringToH160
pub mod missing {}
