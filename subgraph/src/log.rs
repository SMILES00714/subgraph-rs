//! Subgraph logging.

use crate::ffi::{str::AscString, sys};

/// Log level.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u32)]
pub enum Level {
    Critical = 0,
    Error = 1,
    Warning = 2,
    Info = 3,
    Debug = 4,
}

/// Log a message at the specified level.
pub fn log(level: Level, message: &str) {
    let message = AscString::new(message);
    unsafe { sys::log__log(level as _, message.as_ptr()) }
}
