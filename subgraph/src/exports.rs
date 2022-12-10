//! Required WASM exports for Subgraphs.
//!
//! Specifically, Subgraph host requires a small runtime to be exposed from the
//! mapping module for it to function correctly. This module implements the
//! necessary runtime exports.

#![doc(hidden)]

use crate::ffi::{
    boxed::{ALIGN, TYPE_ID},
    str::AscString,
    sys,
};
use std::{
    alloc::{self, Layout},
    panic, ptr,
};

#[export_name = "_start"]
pub extern "C" fn start() {
    panic::set_hook(Box::new(|info| {
        let message = info
            .payload()
            .downcast_ref::<String>()
            .map(|value| &**value)
            .or_else(|| info.payload().downcast_ref::<&str>().copied())
            .unwrap_or("panic occured");

        let (file, line, column) = info
            .location()
            .map(|location| (location.file(), location.line(), location.column()))
            .unwrap_or_default();

        let message = AscString::new(message);
        let file = AscString::new(file);
        unsafe { sys::abort(message.as_ptr(), file.as_ptr(), line, column) }
    }));

    // TODO(nlordell):
    // #[cfg(feature = "log")]
    // install_subgraph_logger();
    // #[cfg(feature = "allocator")]
    // install_custom_allocator();
}

#[export_name = "allocate"]
pub extern "C" fn allocate(size: usize) -> *mut u8 {
    let layout = match Layout::from_size_align(size, ALIGN) {
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

#[export_name = "id_of_type"]
pub extern "C" fn id_of_type(_type_index: u32) -> u32 {
    TYPE_ID
}
