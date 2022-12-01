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
/// - [ ] ethereum.encode
/// - [ ] ethereum.decode
/// - [ ] abort
/// - [ ] store.get
/// - [ ] store.set
/// - [ ] ipfs.cat
/// - [ ] ipfs.map
/// - [ ] ipfs.getBlock
/// - [ ] store.remove
/// - [ ] typeConversion.bytesToString
/// - [ ] typeConversion.bytesToHex
/// - [ ] typeConversion.bigIntToString
/// - [ ] typeConversion.bigIntToHex
/// - [ ] typeConversion.stringToH160
/// - [ ] typeConversion.bytesToBase58
/// - [ ] json.fromBytes
/// - [ ] json.try_fromBytes
/// - [ ] json.toI64
/// - [ ] json.toU64
/// - [ ] json.toF64
/// - [ ] json.toBigInt
/// - [ ] crypto.keccak256
/// - [ ] bigInt.plus
/// - [ ] bigInt.minus
/// - [ ] bigInt.times
/// - [ ] bigInt.dividedBy
/// - [ ] bigInt.dividedByDecimal
/// - [ ] bigInt.mod
/// - [ ] bigInt.pow
/// - [ ] bigInt.fromString
/// - [ ] bigInt.bitOr
/// - [ ] bigInt.bitAnd
/// - [ ] bigInt.leftShift
/// - [ ] bigInt.rightShift
/// - [ ] bigDecimal.toString
/// - [ ] bigDecimal.fromString
/// - [ ] bigDecimal.plus
/// - [ ] bigDecimal.minus
/// - [ ] bigDecimal.times
/// - [ ] bigDecimal.dividedBy
/// - [ ] bigDecimal.equals
/// - [ ] dataSource.create
/// - [ ] dataSource.createWithContext
/// - [ ] dataSource.address
/// - [ ] dataSource.network
/// - [ ] dataSource.context
/// - [ ] ens.nameByHash
/// - [x] log.log
pub mod missing {}
