//! FFI-safe AssemblyScript dynamic values.

use super::{
    boxed::AscObject,
    buf::AscArrayBuffer,
    str::{AscStr, AscString},
};
use std::{mem::ManuallyDrop, ptr, slice};

/// An array of AssemblyScript values.
#[repr(C)]
pub struct AscValueArray<T> {
    buffer: AscArrayBuffer,
    data_start: *mut AscObject<T>,
    byte_length: usize,
    length: usize,
}

impl<T> AscValueArray<T> {
    /// Returns the array as a slice.
    pub fn as_slice(&self) -> &[&T] {
        // SAFETY: `data` points to an allocated value array of known length
        // where all elements are initialized. Additionally, `AscObject<T>` is a
        // transparent representation of a pointer to `T`.
        unsafe { slice::from_raw_parts(self.data_start.cast(), self.length) }
    }
}

impl<T> Drop for AscValueArray<T> {
    fn drop(&mut self) {
        // SAFETY: Array is fully initialized by construction an no longer
        // accessible as it is being dropped.
        unsafe { drop_array(self.length, self.data_start) }
    }
}

/// Implementation helper for dropping a value array with `count` initialized
/// items.
///
/// # Safety
///
/// Callers must ensure that `count` items from the array are initialized and
/// that the array items are no longer used.
unsafe fn drop_array<T>(count: usize, data: *mut AscObject<T>) {
    for i in 0..count {
        ptr::drop_in_place(data.add(i));
    }
}

/// The kind of JSON value.
#[allow(clippy::manual_non_exhaustive, dead_code)]
#[derive(Clone, Copy, Debug)]
#[repr(u32)]
enum AscJsonValueKind {
    Null = 0,
    Bool = 1,
    Number = 2,
    String = 3,
    Array = 4,
    Object = 5,
    #[doc(hidden)]
    _NonExhaustive,
}

/// JSON value data payload.
#[repr(C)]
union AscJsonValuePayload {
    null: u64,
    bool: bool,
    number: ManuallyDrop<AscString>,
    string: ManuallyDrop<AscString>,
    array: ManuallyDrop<AscObject<AscJsonArray>>,
    object: ManuallyDrop<AscObject<AscJsonObject>>,
}

/// A JSON array.
type AscJsonArray = AscValueArray<AscJsonValue>;

/// A JSON object.
#[repr(C)]
struct AscJsonObject {
    entries: AscObject<AscValueArray<AscJsonObjectEntry>>,
}

/// An entry in a JSON object.
#[repr(C)]
pub struct AscJsonObjectEntry {
    pub key: AscString,
    pub value: AscObject<AscJsonValue>,
}

/// An enum representing data held in a JSON value.
pub enum AscJsonValueData<'a> {
    Null,
    Bool(bool),
    Number(&'a AscStr),
    String(&'a AscStr),
    Array(&'a [&'a AscJsonValue]),
    Object(&'a [&'a AscJsonObjectEntry]),
}

/// An AssemblyScript JSON value.
#[repr(C)]
pub struct AscJsonValue {
    kind: AscJsonValueKind,
    data: AscJsonValuePayload,
}

impl AscJsonValue {
    /// Returns the inner JSON data for this value.
    pub fn data(&self) -> AscJsonValueData {
        unsafe {
            match self.kind {
                AscJsonValueKind::Null => AscJsonValueData::Null,
                AscJsonValueKind::Bool => AscJsonValueData::Bool(self.data.bool),
                AscJsonValueKind::Number => AscJsonValueData::Number(&self.data.string),
                AscJsonValueKind::String => AscJsonValueData::String(&self.data.string),
                AscJsonValueKind::Array => {
                    AscJsonValueData::Array(self.data.array.data().as_slice())
                }
                AscJsonValueKind::Object => {
                    AscJsonValueData::Object(self.data.object.data().entries.data().as_slice())
                }
                _ => panic!("unknown JSON value kind {:#x}", self.kind as u32),
            }
        }
    }
}

impl Drop for AscJsonValue {
    fn drop(&mut self) {
        // SAFETY: By construction, we are using the right union variant and we
        // only ever drop when the container is dropping, meening the field will
        // no longer be accessed.
        match self.kind {
            AscJsonValueKind::String => unsafe { ManuallyDrop::drop(&mut self.data.string) },
            AscJsonValueKind::Array => unsafe { ManuallyDrop::drop(&mut self.data.array) },
            AscJsonValueKind::Object => unsafe { ManuallyDrop::drop(&mut self.data.object) },
            _ => (),
        }
    }
}
