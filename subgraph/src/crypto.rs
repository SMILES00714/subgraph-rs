//! Module containing cryptographic utility functions.

use crate::ffi::{buf::AscTypedArray, sys};

/// Computes the Keccak-256 hash of the specified input bytes.
pub fn keccak256(data: impl AsRef<[u8]>) -> [u8; 32] {
    let data = data.as_ref();
    let array = AscTypedArray::from_bytes(data);
    let digest = unsafe { &*sys::crypto__keccak256(array.data() as *const _) };
    digest.as_slice().try_into().unwrap()
}
