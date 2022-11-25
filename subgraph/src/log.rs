//! Subgraph logging.

/// Log level.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(C)]
pub enum Level {
    Critical = 0,
    Error = 1,
    Warning = 2,
    Info = 3,
    Debug = 4,
}

/// Log a message at the specified level.
pub fn log(level: Level, message: &str) {
    let _ = (level, message);
    todo!()
}
