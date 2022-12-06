//! Subgraph type to string conversions.

use crate::ffi::{buf::AscTypedArray, sys};

/// Encodes a slice of bytes as a hex string.
pub fn hex(bytes: impl AsRef<[u8]>) -> String {
    let bytes = AscTypedArray::from_bytes(bytes.as_ref());
    let str = unsafe { &*sys::type_conversion__bytes_to_hex(bytes.as_ptr()) };
    str.to_string_lossy()
}

/// Encodes a slice of bytes as a hex string.
pub fn base58(bytes: impl AsRef<[u8]>) -> String {
    let bytes = AscTypedArray::from_bytes(bytes.as_ref());
    let str = unsafe { &*sys::type_conversion__bytes_to_base58(bytes.as_ptr()) };
    str.to_string_lossy()
}

/// Encodes a slice of bytes as a string.
///
/// This host provided function is included for completeness. However the
/// standard library UTF-8 byte to string conversion methods are preferable.
#[deprecated]
pub fn string(bytes: impl AsRef<[u8]>) -> String {
    let bytes = AscTypedArray::from_bytes(bytes.as_ref());
    let str = unsafe { &*sys::type_conversion__bytes_to_string(bytes.as_ptr()) };
    str.to_string_lossy()
}
