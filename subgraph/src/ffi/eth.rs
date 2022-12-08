//! Ethereum-specific type definition.

use super::{
    boxed::{AscBox, AscNullableBox, AscRef},
    num::AscBigInt,
    str::{AscNullableString, AscStr, AscString},
    types::{AscAddress, AscBytes},
    value::{AscArray, AscEthereumValue},
};

macro_rules! handler_type {
    (
        $(#[$attr:meta])*
        pub struct $name:ident {$(
            $field:ident : $owned:ty => $ref:ty $([$met:ident])?
                ,
        )*}
    ) => {
        $(#[$attr])*
        pub struct $name {$(
            $field: $owned,
        )*}

        impl $name {$(
            handler_type_field! {
                $field: $owned => $ref $([$met])*
            }
        )*}
    };
}

macro_rules! handler_type_field {
    ($field:ident : $owned:ty => $ref:ty) => {
        handler_type_field! { $field: $owned => $ref [as_asc_ref] }
    };
    ($field:ident : $owned:ty => $ref:ty [$met:ident]) => {
        pub(crate) fn $field(&self) -> $ref {
            self.$field.$met()
        }
    };
}

handler_type! {
    /// Ethereum block data.
    pub struct AscBlock {
        hash: AscBox<AscBytes> => &AscRef<AscBytes>,
        parent_hash: AscBox<AscBytes> => &AscRef<AscBytes>,
        uncles_hash: AscBox<AscBytes> => &AscRef<AscBytes>,
        author: AscBox<AscAddress> => &AscRef<AscAddress>,
        state_root: AscBox<AscBytes> => &AscRef<AscBytes>,
        transactions_root: AscBox<AscBytes> => &AscRef<AscBytes>,
        receipts_root: AscBox<AscBytes> => &AscRef<AscBytes>,
        number: AscBox<AscBigInt> => &AscRef<AscBigInt>,
        gas_used: AscBox<AscBigInt> => &AscRef<AscBigInt>,
        gas_limit: AscBox<AscBigInt> => &AscRef<AscBigInt>,
        timestamp: AscBox<AscBigInt> => &AscRef<AscBigInt>,
        difficulty: AscBox<AscBigInt> => &AscRef<AscBigInt>,
        total_difficulty: AscBox<AscBigInt> => &AscRef<AscBigInt>,
        size: AscNullableBox<AscBigInt> => Option<&AscRef<AscBigInt>>,
        base_fee_per_gas: AscNullableBox<AscBigInt> => Option<&AscRef<AscBigInt>>,
    }
}

handler_type! {
    /// An Ethereum transaction.
    pub struct AscTransaction {
        hash: AscBox<AscBytes> => &AscRef<AscBytes>,
        index: AscBox<AscBigInt> => &AscRef<AscBigInt>,
        from: AscBox<AscAddress> => &AscRef<AscAddress>,
        to: AscNullableBox<AscAddress> => Option<&AscRef<AscAddress>>,
        value: AscBox<AscBigInt> => &AscRef<AscBigInt>,
        gas_limit: AscBox<AscBigInt> => &AscRef<AscBigInt>,
        gas_price: AscBox<AscBigInt> => &AscRef<AscBigInt>,
        input: AscBox<AscBytes> => &AscRef<AscBytes>,
        nonce: AscBox<AscBigInt> => &AscRef<AscBigInt>,
    }
}

handler_type! {
    /// Common representation for Ethereum smart contract calls.
    pub struct AscCall {
        to: AscBox<AscAddress> => &AscRef<AscAddress>,
        from: AscBox<AscAddress> => &AscRef<AscAddress>,
        block: AscBox<AscBlock> => &AscRef<AscBlock>,
        transaction: AscBox<AscTransaction> => &AscRef<AscTransaction>,
        input_values: AscBox<AscArray<AscBox<AscEventParam>>>
            => &AscRef<AscArray<AscBox<AscEventParam>>>,
        output_values: AscBox<AscArray<AscBox<AscEventParam>>>
            => &AscRef<AscArray<AscBox<AscEventParam>>>,
    }
}

handler_type! {
    /// Common representation for Ethereum smart contract events.
    pub struct AscEvent {
        address: AscBox<AscAddress> => &AscRef<AscAddress>,
        log_index: AscBox<AscBigInt> => &AscRef<AscBigInt>,
        transaction_log_index: AscBox<AscBigInt> => &AscRef<AscBigInt>,
        log_type: AscNullableString => Option<&AscStr> [as_asc_str],
        block: AscBox<AscBlock> => &AscRef<AscBlock>,
        transaction: AscBox<AscTransaction> => &AscRef<AscTransaction>,
        parameters: AscBox<AscArray<AscBox<AscEventParam>>>
            => &AscRef<AscArray<AscBox<AscEventParam>>>,
    }
}

handler_type! {
    /// A dynamically-typed Ethereum event parameter.
    pub struct AscEventParam {
        name: AscString => &AscStr [as_asc_str],
        value: AscBox<AscEthereumValue> => &AscRef<AscEthereumValue>,
    }
}

/// An Ethereum smart contract call.
#[repr(C)]
pub struct AscEthereumSmartContractCall {
    contract_name: AscString,
    contract_address: AscBox<AscAddress>,
    function_name: AscString,
    function_signature: AscString,
    function_params: AscBox<AscArray<AscBox<AscEthereumValue>>>,
}

impl AscEthereumSmartContractCall {
    /// Creates a new Ethereum smart contract call.
    pub fn new(
        contract_name: AscString,
        contract_address: AscBox<AscAddress>,
        function_name: AscString,
        function_signature: AscString,
        function_params: AscBox<AscArray<AscBox<AscEthereumValue>>>,
    ) -> AscBox<Self> {
        AscBox::new(Self {
            contract_name,
            contract_address,
            function_name,
            function_signature,
            function_params,
        })
    }
}
