//! Ethereum-specific type definition.

use super::{
    boxed::AscBox,
    str::AscString,
    types::AscAddress,
    value::{AscArray, AscEthereumValue},
};

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
