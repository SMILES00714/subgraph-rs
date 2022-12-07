//! Miscellaneous FFI type definitions.

use super::buf::AscTypedArray;

/// Alias for a `Uint8Array` type.
pub type AscUint8Array = AscTypedArray<u8>;

/// Alias for a `ByteArray` type.
pub type AscByteArray = AscUint8Array;

/// Alias for a `Bytes` type.
pub type AscBytes = AscByteArray;

/// Alias for an `Address` type.
pub type AscAddress = AscBytes;
