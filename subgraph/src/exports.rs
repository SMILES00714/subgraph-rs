//! Required WASM exports for Subgraphs.
//!
//! Specifically, Subgraph host requires a small runtime to be exposed from the
//! mapping module for it to function correctly. This module implements the
//! necessary runtime exports.

#![doc(hidden)]

use crate::ffi::boxed::ALIGN;
use std::{
    alloc::{self, Layout},
    ptr,
};

#[export_name = "_start"]
pub extern "C" fn start() {
    // TODO(nlordell):
    // #[cfg(feature = "log")]
    // install_subgraph_logger();
    // #[cfg(feature = "allocator")]
    // install_custom_allocator();
    // logging_panic_hook();
}

#[export_name = "allocate"]
pub extern "C" fn allocate(size: usize) -> *mut u8 {
    let layout = match Layout::from_size_align(ALIGN, size) {
        Ok(value) => value,
        Err(_) => {
            // NOTE: Since `ALIGN` is guaranteed to be valid, this can only
            // happen if `size` overflows when padding to `ALIGN`. Return
            // null to signal that the allocation failed.
            return ptr::null_mut();
        }
    };

    unsafe { alloc::alloc(layout) }
}

#[repr(u32)]
pub enum TypeId {
    String = 0,
    ArrayBuffer = 1,
    Int8Array = 2,
    Int16Array = 3,
    Int32Array = 4,
    Int64Array = 5,
    Uint8Array = 6,
    Uint16Array = 7,
    Uint32Array = 8,
    Uint64Array = 9,
    Float32Array = 10,
    Float64Array = 11,
    BigDecimal = 12,
    ArrayBool = 13,
    ArrayUint8Array = 14,
    ArrayEthereumValue = 15,
    ArrayStoreValue = 16,
    ArrayJsonValue = 17,
    ArrayString = 18,
    ArrayEventParam = 19,
    ArrayTypedMapEntryStringJsonValue = 20,
    ArrayTypedMapEntryStringStoreValue = 21,
    SmartContractCall = 22,
    EventParam = 23,
    EthereumTransaction = 24,
    EthereumBlock = 25,
    EthereumCall = 26,
    WrappedTypedMapStringJsonValue = 27,
    WrappedBool = 28,
    WrappedJsonValue = 29,
    EthereumValue = 30,
    StoreValue = 31,
    JsonValue = 32,
    EthereumEvent = 33,
    TypedMapEntryStringStoreValue = 34,
    TypedMapEntryStringJsonValue = 35,
    TypedMapStringStoreValue = 36,
    TypedMapStringJsonValue = 37,
    TypedMapStringTypedMapStringJsonValue = 38,
    ResultTypedMapStringJsonValueBool = 39,
    ResultJsonValueBool = 40,
    ArrayU8 = 41,
    ArrayU16 = 42,
    ArrayU32 = 43,
    ArrayU64 = 44,
    ArrayI8 = 45,
    ArrayI16 = 46,
    ArrayI32 = 47,
    ArrayI64 = 48,
    ArrayF32 = 49,
    ArrayF64 = 50,
    ArrayBigDecimal = 51,

    // Near types
    NearArrayDataReceiver = 52,
    NearArrayCryptoHash = 53,
    NearArrayActionValue = 54,
    NearMerklePath = 55, // or NearArrayMerklePathItem
    NearArrayValidatorStake = 56,
    NearArraySlashedValidator = 57,
    NearArraySignature = 58,
    NearArrayChunkHeader = 59,
    NearAccessKeyPermissionValue = 60,
    NearActionValue = 61,
    NearDirection = 62, // not used in graph-node anymore. Can be ignored.
    NearPublicKey = 63,
    NearSignature = 64,
    NearFunctionCallPermission = 65,
    NearFullAccessPermission = 66,
    NearAccessKey = 67,
    NearDataReceiver = 68,
    NearCreateAccountAction = 69,
    NearDeployContractAction = 70,
    NearFunctionCallAction = 71,
    NearTransferAction = 72,
    NearStakeAction = 73,
    NearAddKeyAction = 74,
    NearDeleteKeyAction = 75,
    NearDeleteAccountAction = 76,
    NearActionReceipt = 77,
    NearSuccessStatus = 78,
    NearMerklePathItem = 79,
    NearExecutionOutcome = 80,
    NearSlashedValidator = 81,
    NearBlockHeader = 82,
    NearValidatorStake = 83,
    NearChunkHeader = 84,
    NearBlock = 85,
    NearReceiptWithOutcome = 86,
    TransactionReceipt = 1000,
    Log = 1001,
    ArrayH256 = 1002,
    ArrayLog = 1003,

    // Cosmos types
    CosmosAny = 1500,
    CosmosAnyArray = 1501,
    CosmosBytesArray = 1502,
    CosmosCoinArray = 1503,
    CosmosCommitSigArray = 1504,
    CosmosEventArray = 1505,
    CosmosEventAttributeArray = 1506,
    CosmosEvidenceArray = 1507,
    CosmosModeInfoArray = 1508,
    CosmosSignerInfoArray = 1509,
    CosmosTxResultArray = 1510,
    CosmosValidatorArray = 1511,
    CosmosValidatorUpdateArray = 1512,
    CosmosAuthInfo = 1513,
    CosmosBlock = 1514,
    CosmosBlockId = 1515,
    CosmosBlockIdFlagEnum = 1516,
    CosmosBlockParams = 1517,
    CosmosCoin = 1518,
    CosmosCommit = 1519,
    CosmosCommitSig = 1520,
    CosmosCompactBitArray = 1521,
    CosmosConsensus = 1522,
    CosmosConsensusParams = 1523,
    CosmosDuplicateVoteEvidence = 1524,
    CosmosDuration = 1525,
    CosmosEvent = 1526,
    CosmosEventAttribute = 1527,
    CosmosEventData = 1528,
    CosmosEventVote = 1529,
    CosmosEvidence = 1530,
    CosmosEvidenceList = 1531,
    CosmosEvidenceParams = 1532,
    CosmosFee = 1533,
    CosmosHeader = 1534,
    CosmosHeaderOnlyBlock = 1535,
    CosmosLightBlock = 1536,
    CosmosLightClientAttackEvidence = 1537,
    CosmosModeInfo = 1538,
    CosmosModeInfoMulti = 1539,
    CosmosModeInfoSingle = 1540,
    CosmosPartSetHeader = 1541,
    CosmosPublicKey = 1542,
    CosmosResponseBeginBlock = 1543,
    CosmosResponseDeliverTx = 1544,
    CosmosResponseEndBlock = 1545,
    CosmosSignModeEnum = 1546,
    CosmosSignedHeader = 1547,
    CosmosSignedMsgTypeEnum = 1548,
    CosmosSignerInfo = 1549,
    CosmosTimestamp = 1550,
    CosmosTip = 1551,
    CosmosTransactionData = 1552,
    CosmosTx = 1553,
    CosmosTxBody = 1554,
    CosmosTxResult = 1555,
    CosmosValidator = 1556,
    CosmosValidatorParams = 1557,
    CosmosValidatorSet = 1558,
    CosmosValidatorSetUpdates = 1559,
    CosmosValidatorUpdate = 1560,
    CosmosVersionParams = 1561,
    CosmosMessageData = 1562,
    CosmosTransactionContext = 1563,

    // Arweave types
    ArweaveBlock = 2500,
    ArweaveProofOfAccess = 2501,
    ArweaveTag = 2502,
    ArweaveTagArray = 2503,
    ArweaveTransaction = 2504,
    ArweaveTransactionArray = 2505,
    ArweaveTransactionWithBlockPtr = 2506,
}

#[export_name = "id_of_type"]
pub extern "C" fn id_of_type(type_id: TypeId) -> usize {
    (type_id as u32) as _
}
