//! Ethereum-specific type definition.

use super::{
    boxed::AscBox,
    str::AscString,
    types::AscAddress,
    value::{AscArray, AscEthereumValue},
};

/*
  /**
   * An Ethereum block.
   */
  export class Block {
    constructor(
      public hash: Bytes,
      public parentHash: Bytes,
      public unclesHash: Bytes,
      public author: Address,
      public stateRoot: Bytes,
      public transactionsRoot: Bytes,
      public receiptsRoot: Bytes,
      public number: BigInt,
      public gasUsed: BigInt,
      public gasLimit: BigInt,
      public timestamp: BigInt,
      public difficulty: BigInt,
      public totalDifficulty: BigInt,
      public size: BigInt | null,
      public baseFeePerGas: BigInt | null,
    ) {}
  }

  /**
   * An Ethereum transaction.
   */
  export class Transaction {
    constructor(
      public hash: Bytes,
      public index: BigInt,
      public from: Address,
      public to: Address | null,
      public value: BigInt,
      public gasLimit: BigInt,
      public gasPrice: BigInt,
      public input: Bytes,
      public nonce: BigInt,
    ) {}
  }

  /**
   * Common representation for Ethereum smart contract calls.
   */
  export class Call {
    constructor(
      public to: Address,
      public from: Address,
      public block: Block,
      public transaction: Transaction,
      public inputValues: Array<EventParam>,
      public outputValues: Array<EventParam>,
    ) {}
  }

  /**
   * Common representation for Ethereum smart contract events.
   */
  export class Event {
    constructor(
      public address: Address,
      public logIndex: BigInt,
      public transactionLogIndex: BigInt,
      public logType: string | null,
      public block: Block,
      public transaction: Transaction,
      public parameters: Array<EventParam>,
    ) {}
  }

  /**
   * A dynamically-typed Ethereum event parameter.
   */
  export class EventParam {
    constructor(public name: string, public value: Value) {}
  }
*/

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
