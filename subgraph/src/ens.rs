//! ENS resolution.

use crate::ffi::{str::AscString, sys};

/// Resolves an ENS name by a name hash.
pub fn name_by_hash(hash: impl AsRef<str>) -> Option<String> {
    let hash = AscString::new(hash.as_ref());
    let name = unsafe {
        let name = sys::ens__name_by_hash(hash.as_ptr());
        if name.is_null() {
            return None;
        }
        &*name
    };
    Some(name.to_string_lossy())
}
