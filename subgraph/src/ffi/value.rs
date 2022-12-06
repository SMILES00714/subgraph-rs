//! FFI-safe AssemblyScript dynamic values.

use super::{
    boxed::{AscBox, AscNullableBox},
    str::{AscStr, AscString},
};
use std::{
    mem::{self, ManuallyDrop},
    slice,
};

/// An array of AssemblyScript values.
#[repr(C)]
pub struct AscArray<T> {
    buffer: AscBox<[T]>,
    data_start: *const T,
    byte_length: usize,
    length: usize,
}

impl<T> AscArray<T> {
    /// Creates a new AssemblyScript array from the specificed vector.
    pub fn new(items: Vec<T>) -> AscBox<Self> {
        let length = items.len();

        let buffer = items.into_iter().collect::<AscBox<_>>();
        let data_start = buffer.as_ptr().cast();
        let byte_length = length * mem::size_of::<T>();

        AscBox::new(Self {
            buffer,
            data_start,
            byte_length,
            length,
        })
    }

    /// Returns the array as a slice.
    pub fn as_slice(&self) -> &[T] {
        // SAFETY: `data` points to an allocated value array of known length
        // where all elements are initialized. Additionally, `AscObject<T>` is a
        // transparent representation of a pointer to `T`.
        unsafe { slice::from_raw_parts(self.data_start, self.length) }
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
    array: ManuallyDrop<AscBox<AscJsonArray>>,
    object: ManuallyDrop<AscBox<AscJsonObject>>,
}

/// A JSON array.
type AscJsonArray = AscArray<AscBox<AscJsonValue>>;

/// A JSON object.
#[repr(C)]
struct AscJsonObject {
    entries: AscBox<AscArray<AscBox<AscJsonObjectEntry>>>,
}

/// An entry in a JSON object.
#[repr(C)]
pub struct AscJsonObjectEntry {
    pub key: AscString,
    pub value: AscBox<AscJsonValue>,
}

/// An enum representing data held in a JSON value.
pub enum AscJsonValueData<'a> {
    Null,
    Bool(bool),
    Number(&'a AscStr),
    String(&'a AscStr),
    Array(&'a [AscBox<AscJsonValue>]),
    Object(&'a [AscBox<AscJsonObjectEntry>]),
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
                    AscJsonValueData::Array(self.data.array.as_asc_ref().as_slice())
                }
                AscJsonValueKind::Object => AscJsonValueData::Object(
                    self.data
                        .object
                        .as_asc_ref()
                        .entries
                        .as_asc_ref()
                        .as_slice(),
                ),
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

/// An AssemblyScript result type.
#[repr(C)]
pub struct AscResult<T, E> {
    ok: AscNullableBox<T>,
    err: AscNullableBox<E>,
}

impl<T, E> AscResult<T, E> {
    /// Converst the AssemblyScript result wrapper into a Rust standard library
    /// [`Result`].
    pub fn as_std_result(&self) -> Result<&T, &E> {
        match (self.ok.as_asc_ref(), self.err.as_asc_ref()) {
            (Some(ok), None) => Ok(ok),
            (None, Some(err)) => Err(err),
            _ => panic!("inconsistent result"),
        }
    }
}
