//! Subgraph logging.
//!
//! TODO(nlordell): Add internal debugging mechanism that can be used without
//! AssemblyScript boxing (manually encode to a vector for example). This
//! can be used for internal debugging of `AscBox: Drop` implementation for
//! example.

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
