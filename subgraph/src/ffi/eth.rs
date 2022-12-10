//! Ethereum-specific type definition.

use super::{
    boxed::{AscBox, AscNullableBox, AscRef},
    num::AscBigInt,
    str::{AscNullableString, AscStr, AscString},
    types::{AscAddress, AscBytes},
    value::{AscArray, AscEthereumValue},
};

/// Ethereum block data.
pub struct AscBlock {
    hash: AscBox<AscBytes>,
    parent_hash: AscBox<AscBytes>,
    uncles_hash: AscBox<AscBytes>,
    author: AscBox<AscAddress>,
    state_root: AscBox<AscBytes>,
    transactions_root: AscBox<AscBytes>,
    receipts_root: AscBox<AscBytes>,
    number: AscBox<AscBigInt>,
    gas_used: AscBox<AscBigInt>,
    gas_limit: AscBox<AscBigInt>,
    timestamp: AscBox<AscBigInt>,
    difficulty: AscBox<AscBigInt>,
    total_difficulty: AscBox<AscBigInt>,
    size: AscNullableBox<AscBigInt>,
    base_fee_per_gas: AscNullableBox<AscBigInt>,
}

impl AscBlock {
    pub(crate) fn hash(&self) -> &AscRef<AscBytes> {
        self.hash.as_asc_ref()
    }

    pub(crate) fn parent_hash(&self) -> &AscRef<AscBytes> {
        self.parent_hash.as_asc_ref()
    }

    pub(crate) fn uncles_hash(&self) -> &AscRef<AscBytes> {
        self.uncles_hash.as_asc_ref()
    }

    pub(crate) fn author(&self) -> &AscRef<AscAddress> {
        self.author.as_asc_ref()
    }

    pub(crate) fn state_root(&self) -> &AscRef<AscBytes> {
        self.state_root.as_asc_ref()
    }

    pub(crate) fn transactions_root(&self) -> &AscRef<AscBytes> {
        self.transactions_root.as_asc_ref()
    }

    pub(crate) fn receipts_root(&self) -> &AscRef<AscBytes> {
        self.receipts_root.as_asc_ref()
    }

    pub(crate) fn number(&self) -> &AscRef<AscBigInt> {
        self.number.as_asc_ref()
    }

    pub(crate) fn gas_used(&self) -> &AscRef<AscBigInt> {
        self.gas_used.as_asc_ref()
    }

    pub(crate) fn gas_limit(&self) -> &AscRef<AscBigInt> {
        self.gas_limit.as_asc_ref()
    }

    pub(crate) fn timestamp(&self) -> &AscRef<AscBigInt> {
        self.timestamp.as_asc_ref()
    }

    pub(crate) fn difficulty(&self) -> &AscRef<AscBigInt> {
        self.difficulty.as_asc_ref()
    }

    pub(crate) fn total_difficulty(&self) -> &AscRef<AscBigInt> {
        self.total_difficulty.as_asc_ref()
    }

    pub(crate) fn size(&self) -> Option<&AscRef<AscBigInt>> {
        self.size.as_asc_ref()
    }

    pub(crate) fn base_fee_per_gas(&self) -> Option<&AscRef<AscBigInt>> {
        self.base_fee_per_gas.as_asc_ref()
    }
}

/// An Ethereum transaction.
pub struct AscTransaction {
    hash: AscBox<AscBytes>,
    index: AscBox<AscBigInt>,
    from: AscBox<AscAddress>,
    to: AscNullableBox<AscAddress>,
    value: AscBox<AscBigInt>,
    gas_limit: AscBox<AscBigInt>,
    gas_price: AscBox<AscBigInt>,
    input: AscBox<AscBytes>,
    nonce: AscBox<AscBigInt>,
}

impl AscTransaction {
    pub(crate) fn hash(&self) -> &AscRef<AscBytes> {
        self.hash.as_asc_ref()
    }

    pub(crate) fn index(&self) -> &AscRef<AscBigInt> {
        self.index.as_asc_ref()
    }

    pub(crate) fn from(&self) -> &AscRef<AscAddress> {
        self.from.as_asc_ref()
    }

    pub(crate) fn to(&self) -> Option<&AscRef<AscAddress>> {
        self.to.as_asc_ref()
    }

    pub(crate) fn value(&self) -> &AscRef<AscBigInt> {
        self.value.as_asc_ref()
    }

    pub(crate) fn gas_limit(&self) -> &AscRef<AscBigInt> {
        self.gas_limit.as_asc_ref()
    }

    pub(crate) fn gas_price(&self) -> &AscRef<AscBigInt> {
        self.gas_price.as_asc_ref()
    }

    pub(crate) fn input(&self) -> &AscRef<AscBytes> {
        self.input.as_asc_ref()
    }

    pub(crate) fn nonce(&self) -> &AscRef<AscBigInt> {
        self.nonce.as_asc_ref()
    }
}

/// An Ethereum transaction receipt.
pub struct AscTransactionReceipt {
    transaction_hash: AscBox<AscBytes>,
    transaction_index: AscBox<AscBigInt>,
    block_hash: AscBox<AscBytes>,
    block_number: AscBox<AscBigInt>,
    cumulative_gas_used: AscBox<AscBigInt>,
    gas_used: AscBox<AscBigInt>,
    contract_address: AscNullableBox<AscAddress>,
    logs: AscBox<AscArray<AscBox<AscLog>>>,
    status: AscBox<AscBigInt>,
    root: AscNullableBox<AscBytes>,
    logs_bloom: AscBox<AscBytes>,
}

impl AscTransactionReceipt {
    pub(crate) fn transaction_hash(&self) -> &AscRef<AscBytes> {
        self.transaction_hash.as_asc_ref()
    }

    pub(crate) fn transaction_index(&self) -> &AscRef<AscBigInt> {
        self.transaction_index.as_asc_ref()
    }

    pub(crate) fn block_hash(&self) -> &AscRef<AscBytes> {
        self.block_hash.as_asc_ref()
    }

    pub(crate) fn block_number(&self) -> &AscRef<AscBigInt> {
        self.block_number.as_asc_ref()
    }

    pub(crate) fn cumulative_gas_used(&self) -> &AscRef<AscBigInt> {
        self.cumulative_gas_used.as_asc_ref()
    }

    pub(crate) fn gas_used(&self) -> &AscRef<AscBigInt> {
        self.gas_used.as_asc_ref()
    }

    pub(crate) fn contract_address(&self) -> Option<&AscRef<AscAddress>> {
        self.contract_address.as_asc_ref()
    }

    pub(crate) fn logs(&self) -> &AscRef<AscArray<AscBox<AscLog>>> {
        self.logs.as_asc_ref()
    }

    pub(crate) fn status(&self) -> &AscRef<AscBigInt> {
        self.status.as_asc_ref()
    }

    pub(crate) fn root(&self) -> Option<&AscRef<AscBytes>> {
        self.root.as_asc_ref()
    }

    pub(crate) fn logs_bloom(&self) -> &AscRef<AscBytes> {
        self.logs_bloom.as_asc_ref()
    }
}

/// An Ethereum log.
pub struct AscLog {
    address: AscBox<AscAddress>,
    topics: AscBox<AscArray<AscBox<AscBytes>>>,
    data: AscBox<AscBytes>,
    block_hash: AscBox<AscBytes>,
    block_number: AscBox<AscBytes>,
    transaction_hash: AscBox<AscBytes>,
    transaction_index: AscBox<AscBigInt>,
    log_index: AscBox<AscBigInt>,
    transaction_log_index: AscBox<AscBigInt>,
    log_type: AscNullableString,
    removed: AscNullableBox<bool>,
}

impl AscLog {
    pub(crate) fn address(&self) -> &AscRef<AscAddress> {
        self.address.as_asc_ref()
    }

    pub(crate) fn topics(&self) -> &AscRef<AscArray<AscBox<AscBytes>>> {
        self.topics.as_asc_ref()
    }

    pub(crate) fn data(&self) -> &AscRef<AscBytes> {
        self.data.as_asc_ref()
    }

    pub(crate) fn block_hash(&self) -> &AscRef<AscBytes> {
        self.block_hash.as_asc_ref()
    }

    pub(crate) fn block_number(&self) -> &AscRef<AscBytes> {
        self.block_number.as_asc_ref()
    }

    pub(crate) fn transaction_hash(&self) -> &AscRef<AscBytes> {
        self.transaction_hash.as_asc_ref()
    }

    pub(crate) fn transaction_index(&self) -> &AscRef<AscBigInt> {
        self.transaction_index.as_asc_ref()
    }

    pub(crate) fn log_index(&self) -> &AscRef<AscBigInt> {
        self.log_index.as_asc_ref()
    }

    pub(crate) fn transaction_log_index(&self) -> &AscRef<AscBigInt> {
        self.transaction_log_index.as_asc_ref()
    }

    pub(crate) fn log_type(&self) -> Option<&AscStr> {
        self.log_type.as_asc_str()
    }

    pub(crate) fn removed(&self) -> Option<&AscRef<bool>> {
        self.removed.as_asc_ref()
    }
}

/// Common representation for Ethereum smart contract calls.
pub struct AscCall {
    to: AscBox<AscAddress>,
    from: AscBox<AscAddress>,
    block: AscBox<AscBlock>,
    transaction: AscBox<AscTransaction>,
    input_values: AscBox<AscArray<AscBox<AscEventParam>>>,
    output_values: AscBox<AscArray<AscBox<AscEventParam>>>,
}

impl AscCall {
    pub(crate) fn to(&self) -> &AscRef<AscAddress> {
        self.to.as_asc_ref()
    }

    pub(crate) fn from(&self) -> &AscRef<AscAddress> {
        self.from.as_asc_ref()
    }

    pub(crate) fn block(&self) -> &AscRef<AscBlock> {
        self.block.as_asc_ref()
    }

    pub(crate) fn transaction(&self) -> &AscRef<AscTransaction> {
        self.transaction.as_asc_ref()
    }

    pub(crate) fn input_values(&self) -> &AscRef<AscArray<AscBox<AscEventParam>>> {
        self.input_values.as_asc_ref()
    }

    pub(crate) fn output_values(&self) -> &AscRef<AscArray<AscBox<AscEventParam>>> {
        self.output_values.as_asc_ref()
    }
}

/// Common representation for Ethereum smart contract events.
pub struct AscEvent {
    address: AscBox<AscAddress>,
    log_index: AscBox<AscBigInt>,
    transaction_log_index: AscBox<AscBigInt>,
    log_type: AscNullableString,
    block: AscBox<AscBlock>,
    transaction: AscBox<AscTransaction>,
    parameters: AscBox<AscArray<AscBox<AscEventParam>>>,
    receipt: AscNullableBox<AscTransactionReceipt>,
}

impl AscEvent {
    pub(crate) fn address(&self) -> &AscRef<AscAddress> {
        self.address.as_asc_ref()
    }

    pub(crate) fn log_index(&self) -> &AscRef<AscBigInt> {
        self.log_index.as_asc_ref()
    }

    pub(crate) fn transaction_log_index(&self) -> &AscRef<AscBigInt> {
        self.transaction_log_index.as_asc_ref()
    }

    pub(crate) fn log_type(&self) -> Option<&AscStr> {
        self.log_type.as_asc_str()
    }

    pub(crate) fn block(&self) -> &AscRef<AscBlock> {
        self.block.as_asc_ref()
    }

    pub(crate) fn transaction(&self) -> &AscRef<AscTransaction> {
        self.transaction.as_asc_ref()
    }

    pub(crate) fn parameters(&self) -> &AscRef<AscArray<AscBox<AscEventParam>>> {
        self.parameters.as_asc_ref()
    }

    pub(crate) fn receipt(&self) -> Option<&AscRef<AscTransactionReceipt>> {
        self.receipt.as_asc_ref()
    }
}

/// A dynamically-typed Ethereum event parameter.
pub struct AscEventParam {
    name: AscString,
    value: AscBox<AscEthereumValue>,
}

impl AscEventParam {
    pub(crate) fn name(&self) -> &AscStr {
        self.name.as_asc_str()
    }

    pub(crate) fn value(&self) -> &AscRef<AscEthereumValue> {
        self.value.as_asc_ref()
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
