//! Subgraph host imports.
//!
//! This module just declares the "raw" host methods for WASM imports.

use super::{
    boxed::{AscBox, AscRef},
    eth::AscEthereumSmartContractCall,
    num::{AscBigDecimal, AscBigInt},
    str::{AscStr, AscString},
    types::{AscAddress, AscByteArray, AscBytes, AscUint8Array},
    value::{AscArray, AscEntity, AscEntityValue, AscEthereumValue, AscJsonValue, AscResult},
};

#[link(wasm_import_module = "index")]
extern "C" {
    #[link_name = "abort"]
    pub fn abort(
        message: *const AscStr,
        file_name: *const AscStr,
        line_number: u32,
        column_number: u32,
    ) -> !;

    #[link_name = "bigDecimal.dividedBy"]
    pub fn big_decimal__divided_by(
        x: *const AscRef<AscBigDecimal>,
        y: *const AscRef<AscBigDecimal>,
    ) -> *const AscRef<AscBigDecimal>;
    #[link_name = "bigDecimal.equals"]
    pub fn big_decimal__equals(
        x: *const AscRef<AscBigDecimal>,
        y: *const AscRef<AscBigDecimal>,
    ) -> bool;
    #[link_name = "bigDecimal.fromString"]
    pub fn big_decimal__from_string(s: *const AscStr) -> *const AscRef<AscBigDecimal>;
    #[link_name = "bigDecimal.minus"]
    pub fn big_decimal__minus(
        x: *const AscRef<AscBigDecimal>,
        y: *const AscRef<AscBigDecimal>,
    ) -> *const AscRef<AscBigDecimal>;
    #[link_name = "bigDecimal.plus"]
    pub fn big_decimal__plus(
        x: *const AscRef<AscBigDecimal>,
        y: *const AscRef<AscBigDecimal>,
    ) -> *const AscRef<AscBigDecimal>;
    #[link_name = "bigDecimal.times"]
    pub fn big_decimal__times(
        x: *const AscRef<AscBigDecimal>,
        y: *const AscRef<AscBigDecimal>,
    ) -> *const AscRef<AscBigDecimal>;
    #[link_name = "bigDecimal.toString"]
    pub fn big_decimal__to_string(value: *const AscRef<AscBigDecimal>) -> *const AscStr;

    #[link_name = "bigInt.bitAnd"]
    pub fn big_int__bit_and(
        x: *const AscRef<AscBigInt>,
        y: *const AscRef<AscBigInt>,
    ) -> *const AscRef<AscBigInt>;
    #[link_name = "bigInt.bitOr"]
    pub fn big_int__bit_or(
        x: *const AscRef<AscBigInt>,
        y: *const AscRef<AscBigInt>,
    ) -> *const AscRef<AscBigInt>;
    #[link_name = "bigInt.dividedBy"]
    pub fn big_int__divided_by(
        x: *const AscRef<AscBigInt>,
        y: *const AscRef<AscBigInt>,
    ) -> *const AscRef<AscBigInt>;
    #[link_name = "bigInt.dividedByDecimal"]
    pub fn big_int__divided_by_decimal(
        x: *const AscRef<AscBigInt>,
        y: *const AscRef<AscBigDecimal>,
    ) -> *const AscRef<AscBigDecimal>;
    #[link_name = "bigInt.fromString"]
    pub fn big_int__from_string(s: *const AscStr) -> *const AscRef<AscBigInt>;
    #[link_name = "bigInt.leftShift"]
    pub fn big_int__left_shift(x: *const AscRef<AscBigInt>, y: u8) -> *const AscRef<AscBigInt>;
    #[link_name = "bigInt.minus"]
    pub fn big_int__minus(
        x: *const AscRef<AscBigInt>,
        y: *const AscRef<AscBigInt>,
    ) -> *const AscRef<AscBigInt>;
    #[link_name = "bigInt.mod"]
    pub fn big_int__mod(
        x: *const AscRef<AscBigInt>,
        y: *const AscRef<AscBigInt>,
    ) -> *const AscRef<AscBigInt>;
    #[link_name = "bigInt.plus"]
    pub fn big_int__plus(
        x: *const AscRef<AscBigInt>,
        y: *const AscRef<AscBigInt>,
    ) -> *const AscRef<AscBigInt>;
    #[link_name = "bigInt.pow"]
    pub fn big_int__pow(x: *const AscRef<AscBigInt>, y: u8) -> *const AscRef<AscBigInt>;
    #[link_name = "bigInt.rightShift"]
    pub fn big_int__right_shift(x: *const AscRef<AscBigInt>, y: u8) -> *const AscRef<AscBigInt>;
    #[link_name = "bigInt.times"]
    pub fn big_int__times(
        x: *const AscRef<AscBigInt>,
        y: *const AscRef<AscBigInt>,
    ) -> *const AscRef<AscBigInt>;

    #[link_name = "crypto.keccak256"]
    pub fn crypto__keccak256(data: *const AscRef<AscByteArray>) -> *const AscRef<AscByteArray>;

    #[link_name = "dataSource.address"]
    pub fn data_source__address() -> *const AscRef<AscAddress>;
    #[link_name = "dataSource.context"]
    pub fn data_source__context() -> *const AscRef<AscEntity>;
    #[link_name = "dataSource.create"]
    pub fn data_source__create(name: *const AscStr, params: *const AscRef<AscArray<AscString>>);
    #[link_name = "dataSource.createWithContext"]
    pub fn data_source__create_with_context(
        name: *const AscStr,
        params: *const AscRef<AscArray<AscString>>,
        context: *const AscRef<AscEntity>,
    );
    #[link_name = "dataSource.network"]
    pub fn data_source__network() -> *const AscStr;

    #[link_name = "ens.nameByHash"]
    pub fn ens__name_by_hash(hash: *const AscStr) -> *const AscStr;

    #[link_name = "ethereum.call"]
    pub fn ethereum__call(
        call: *const AscRef<AscEthereumSmartContractCall>,
    ) -> *const AscRef<AscArray<AscBox<AscEthereumValue>>>;
    #[link_name = "ethereum.decode"]
    pub fn ethereum__decode(
        signature: *const AscStr,
        data: *const AscRef<AscBytes>,
    ) -> *const AscRef<AscEthereumValue>;
    #[link_name = "ethereum.encode"]
    pub fn ethereum__encode(value: *const AscRef<AscEthereumValue>) -> *const AscRef<AscBytes>;

    #[link_name = "ipfs.cat"]
    pub fn ipfs__cat(hash: *const AscStr) -> *const AscBytes;
    #[link_name = "ipfs.map"]
    pub fn ipfs__map(
        hash: *const AscStr,
        callback: *const AscStr,
        user_data: *const AscRef<AscEntityValue>,
        flags: *const AscRef<AscArray<AscString>>,
    );

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

    #[link_name = "store.get"]
    pub fn store__get(entity: *const AscStr, id: *const AscStr) -> *const AscRef<AscEntity>;
    #[link_name = "store.remove"]
    pub fn store__remove(entity: *const AscStr, id: *const AscStr);
    #[link_name = "store.set"]
    pub fn store__set(entity: *const AscStr, id: *const AscStr, data: *const AscRef<AscEntity>);

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
