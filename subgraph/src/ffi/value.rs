//! FFI-safe AssemblyScript dynamic values.

use super::{
    boxed::{AscBox, AscNullableBox, AscRef},
    buf::AscTypedArray,
    num::{AscBigDecimal, AscBigInt},
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

impl<T> Clone for AscArray<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        let buffer = self.buffer.clone();
        let data_start = buffer.as_ptr().cast();

        Self {
            buffer,
            data_start,
            byte_length: self.byte_length,
            length: self.length,
        }
    }
}

/// A AssemblyScript map with string keys.
#[repr(C)]
pub struct AscMap<T> {
    entries: AscBox<AscArray<AscBox<AscMapEntry<T>>>>,
}

/// An entry in a JSON object.
#[repr(C)]
pub struct AscMapEntry<T> {
    key: AscString,
    value: T,
}

impl<T> AscMap<T> {
    /// Returns entries as a slice.
    pub fn new(entries: Vec<AscBox<AscMapEntry<T>>>) -> AscBox<Self> {
        AscBox::new(Self {
            entries: AscArray::new(entries),
        })
    }

    /// Returns entries as a slice.
    pub fn entries(&self) -> &[AscBox<AscMapEntry<T>>] {
        self.entries.as_asc_ref().as_slice()
    }
}

impl<T> AscMapEntry<T> {
    /// Creates a new AssemblyScript map entry.
    pub fn new(key: AscString, value: T) -> AscBox<Self> {
        AscBox::new(Self { key, value })
    }

    /// Returns the entry key.
    pub fn key(&self) -> &AscStr {
        self.key.as_asc_str()
    }

    /// Returns the entry value.
    pub fn value(&self) -> &T {
        &self.value
    }
}

/// A Subgraph value kind.
#[allow(clippy::manual_non_exhaustive, dead_code)]
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
#[repr(u32)]
enum AscValueKind {
    String = 0,
    Int = 1,
    BigDecimal = 2,
    Bool = 3,
    Array = 4,
    Null = 5,
    Bytes = 6,
    BigInt = 7,
    #[doc(hidden)]
    _NonExhaustive,
}

/// Subgraph value data payload.
#[repr(C)]
union AscValuePayload {
    string: ManuallyDrop<AscString>,
    int: i32,
    bigdecimal: ManuallyDrop<AscBox<AscBigDecimal>>,
    bool: bool,
    array: ManuallyDrop<AscBox<AscArray<AscBox<AscValue>>>>,
    null: u64,
    bytes: ManuallyDrop<AscBox<AscTypedArray<u8>>>,
    bigint: ManuallyDrop<AscBox<AscBigInt>>,
}

/// An enum representing data held in a Subgraph value.
pub enum AscValueData<'a> {
    String(&'a AscStr),
    Int(i32),
    BigDecimal(&'a AscRef<AscBigDecimal>),
    Bool(bool),
    Array(&'a AscRef<AscArray<AscBox<AscValue>>>),
    Null,
    Bytes(&'a AscRef<AscTypedArray<u8>>),
    BigInt(&'a AscRef<AscBigInt>),
}

/// An AssemblyScript Subgraph key-value map.
pub type AscValueMap = AscMap<AscBox<AscValue>>;

/// An AssemblyScript Subgraph value.
#[repr(C)]
pub struct AscValue {
    kind: AscValueKind,
    data: AscValuePayload,
}

impl AscValue {
    /// Creates a new string value.
    pub fn string(value: AscString) -> AscBox<Self> {
        AscBox::new(Self {
            kind: AscValueKind::String,
            data: AscValuePayload {
                string: ManuallyDrop::new(value),
            },
        })
    }

    /// Creates a new integer value.
    pub fn int(value: i32) -> AscBox<Self> {
        AscBox::new(Self {
            kind: AscValueKind::Int,
            data: AscValuePayload { int: value },
        })
    }

    /// Creates a new big decimal value.
    pub fn bigdecimal(value: AscBox<AscBigDecimal>) -> AscBox<Self> {
        AscBox::new(Self {
            kind: AscValueKind::BigDecimal,
            data: AscValuePayload {
                bigdecimal: ManuallyDrop::new(value),
            },
        })
    }

    /// Creates a new boolean value.
    pub fn bool(value: bool) -> AscBox<Self> {
        AscBox::new(Self {
            kind: AscValueKind::Bool,
            data: AscValuePayload { bool: value },
        })
    }

    /// Creates a new array value.
    pub fn array(value: AscBox<AscArray<AscBox<AscValue>>>) -> AscBox<Self> {
        AscBox::new(Self {
            kind: AscValueKind::Array,
            data: AscValuePayload {
                array: ManuallyDrop::new(value),
            },
        })
    }

    /// Creates a new null value.
    pub fn null() -> AscBox<Self> {
        AscBox::new(Self {
            kind: AscValueKind::Null,
            data: AscValuePayload { null: 0 },
        })
    }

    /// Creates a new bytes value.
    pub fn bytes(value: AscBox<AscTypedArray<u8>>) -> AscBox<Self> {
        AscBox::new(Self {
            kind: AscValueKind::Bytes,
            data: AscValuePayload {
                bytes: ManuallyDrop::new(value),
            },
        })
    }

    /// Creates a new big integer value.
    pub fn bigint(value: AscBox<AscBigInt>) -> AscBox<Self> {
        AscBox::new(Self {
            kind: AscValueKind::BigInt,
            data: AscValuePayload {
                bigint: ManuallyDrop::new(value),
            },
        })
    }

    /// Returns the inner JSON data for this value.
    pub fn data(&self) -> AscValueData {
        unsafe {
            match self.kind {
                AscValueKind::String => AscValueData::String(&self.data.string),
                AscValueKind::Int => AscValueData::Int(self.data.int),
                AscValueKind::BigDecimal => {
                    AscValueData::BigDecimal(self.data.bigdecimal.as_asc_ref())
                }
                AscValueKind::Bool => AscValueData::Bool(self.data.bool),
                AscValueKind::Array => AscValueData::Array(self.data.array.as_asc_ref()),
                AscValueKind::Null => AscValueData::Null,
                AscValueKind::Bytes => AscValueData::Bytes(self.data.bytes.as_asc_ref()),
                AscValueKind::BigInt => AscValueData::BigInt(self.data.bigint.as_asc_ref()),
                _ => panic!("unknown JSON value kind {:#x}", self.kind as u32),
            }
        }
    }
}

impl Drop for AscValue {
    fn drop(&mut self) {
        // SAFETY: By construction, we are using the right union variant and we
        // only ever drop when the container is dropping, meening the field will
        // no longer be accessed.
        unsafe {
            match self.kind {
                AscValueKind::String => ManuallyDrop::drop(&mut self.data.string),
                AscValueKind::BigDecimal => ManuallyDrop::drop(&mut self.data.bigdecimal),
                AscValueKind::Array => ManuallyDrop::drop(&mut self.data.array),
                AscValueKind::Bytes => ManuallyDrop::drop(&mut self.data.bytes),
                AscValueKind::BigInt => ManuallyDrop::drop(&mut self.data.bigint),
                _ => (),
            }
        }
    }
}

/// The kind of JSON value.
#[allow(clippy::manual_non_exhaustive, dead_code)]
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
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
type AscJsonObject = AscMap<AscBox<AscJsonValue>>;

/// An enum representing data held in a JSON value.
pub enum AscJsonValueData<'a> {
    Null,
    Bool(bool),
    Number(&'a AscStr),
    String(&'a AscStr),
    Array(&'a AscRef<AscJsonArray>),
    Object(&'a AscRef<AscJsonObject>),
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
                AscJsonValueKind::Array => AscJsonValueData::Array(self.data.array.as_asc_ref()),
                AscJsonValueKind::Object => AscJsonValueData::Object(self.data.object.as_asc_ref()),
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
