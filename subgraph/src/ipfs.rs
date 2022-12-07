//! IPFS bindings.

use crate::{
    entity::Value,
    ffi::{str::AscString, sys, value::AscArray},
};

/// Gets an entity by name and ID.
pub fn cat(hash: impl AsRef<str>) -> Option<Vec<u8>> {
    let hash = AscString::new(hash.as_ref());
    let data = unsafe {
        let data = sys::ipfs__cat(hash.as_ptr());
        if data.is_null() {
            return None;
        }
        &*data
    };
    Some(data.as_slice().to_owned())
}

/// Queues a callback for when an IPFS hash resolves.
pub fn map(hash: impl AsRef<str>, callback: impl AsRef<str>, user_data: Value, flags: &[&str]) {
    let hash = AscString::new(hash.as_ref());
    let callback = AscString::new(callback.as_ref());
    let user_data = user_data.to_raw();
    let flags = AscArray::new(flags.iter().copied().map(AscString::new).collect());
    unsafe {
        sys::ipfs__map(
            hash.as_ptr(),
            callback.as_ptr(),
            user_data.as_ptr(),
            flags.as_ptr(),
        );
    }
}
