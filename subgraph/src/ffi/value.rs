//! FFI-safe AssemblyScript dynamic values.

use super::{
    boxed::{AscBox, AscNullableBox, AscRef},
    num::{AscBigDecimal, AscBigInt},
    str::{AscStr, AscString},
    types::{AscAddress, AscBytes},
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

/// Generate code for a tagged union.
macro_rules! asc_tagged_union {
    (
        $(#[$attr:meta])*
        $value:ident, $kind:ident, $payload:ident, $data:ident {$(
            $variant:ident , $field:ident ($($type:tt)*) = $tag:literal ,

        )*}
    ) => {
        $(#[$attr])*
        #[repr(C)]
        pub struct $value {
            kind: $kind,
            data: $payload,
        }

        #[allow(clippy::manual_non_exhaustive, dead_code)]
        #[derive(Clone, Copy, Debug)]
        #[non_exhaustive]
        #[repr(u32)]
        enum $kind {
            $(
                $variant = $tag,
            )*
            #[doc(hidden)]
            _NonExhaustive,
        }

        #[repr(C)]
        union $payload {
            $(
                $field: asc_tagged_union_field!(field: $($type)*),
            )*
            _padding: u64,
        }

        pub enum $data<'a> {
            $(
                $variant(asc_tagged_union_field!(ref 'a: $($type)*)),
            )*
        }

        #[allow(dead_code, unused_variables)]
        impl $value {
            $(
                /// Creates a new value.
                pub fn $field(
                    value: asc_tagged_union_field!(owned: $($type)*),
                ) -> AscBox<Self> {
                    AscBox::new(Self {
                        kind: $kind::$variant,
                        data: $payload {
                            $field: asc_tagged_union_field!(new(value): $($type)*),
                        },
                    })
                }
            )*

            /// Returns a reference to the inner data for this value.
            pub fn data(&self) -> $data {
                match self.kind {
                    $(
                        $kind::$variant => $data::$variant(
                            asc_tagged_union_field!(data(self.data.$field): $($type)*),
                        ),
                    )*
                    _ => panic!("unknown value kind {:#x}", self.kind as u32),
                }
            }
        }

        impl Drop for $value {
            fn drop(&mut self) {
                // SAFETY: By construction, we are using the right union variant
                // and we only ever drop when the container is dropping, meening
                // the field will no longer be accessed.
                match self.kind {
                    $(
                        $kind::$variant =>
                            asc_tagged_union_field!(drop(self.data.$field): $($type)*),
                    )*
                    _ => ()
                }
            }
        }
    };
}

#[rustfmt::skip]
macro_rules! asc_tagged_union_field {
    (owned: null) => { () };
    (ref $a:lifetime: null) => { () };
    (field: null) => { u64 };
    (new($f:expr): null) => { 0 };
    (data($f:expr): null) => { () };
    (drop($f:expr): null) => { () };

    (owned: string) => { AscString };
    (ref $a:lifetime: string) => { &$a AscStr };
    (field: string) => { ManuallyDrop<AscString> };
    (new($f:expr): string) => { ManuallyDrop::new($f) };
    (data($f:expr): string) => { unsafe { $f.as_asc_str() } };
    (drop($f:expr): string) => { unsafe { ManuallyDrop::drop(&mut $f) } };

    (owned: value $type:ty) => { $type };
    (ref $a:lifetime: value $type:ty) => { $type };
    (field: value $type:ty) => { $type };
    (new($f:expr): value $type:ty) => { $f };
    (data($f:expr): value $type:ty) => { unsafe { $f } };
    (drop($f:expr): value $type:ty) => { () };

    (owned: boxed $type:ty) => { AscBox<$type> };
    (ref $a:lifetime: boxed $type:ty) => { &$a AscRef<$type> };
    (field: boxed $type:ty) => { ManuallyDrop<AscBox<$type>> };
    (new($f:expr): boxed $type:ty) => { ManuallyDrop::new($f) };
    (data($f:expr): boxed $type:ty) => { unsafe { $f.as_asc_ref() } };
    (drop($f:expr): boxed $type:ty) => { unsafe { ManuallyDrop::drop(&mut $f) } };
}

/// An AssemblyScript Subgraph key-value map.
pub type AscEntity = AscMap<AscBox<AscEntityValue>>;

asc_tagged_union! {
    /// An AssemblyScript JSON dynamic value.
    AscEntityValue,
    AscEntityValueKind,
    AscEntityValuePayload,
    AscEntityValueData {
        String, string (string) = 0,
        Int, int (value i32) = 1,
        BigDecimal, bigdecimal (boxed AscBigDecimal) = 2,
        Bool, bool (value bool) = 3,
        Array, array (boxed AscArray<AscBox<AscEntityValue>>) = 4,
        Null, null (null) = 5,
        Bytes, bytes (boxed AscBytes) = 6,
        BigInt, bigint (boxed AscBigInt) = 7,
    }
}

asc_tagged_union! {
    /// An AssemblyScript JSON dynamic value.
    AscJsonValue,
    AscJsonValueKind,
    AscJsonValuePayload,
    AscJsonValueData {
        Null, null (null) = 0,
        Bool, bool (value bool) = 1,
        Number, number (string) = 2,
        String, string (string) = 3,
        Array, array (boxed AscArray<AscBox<AscJsonValue>>) = 4,
        Object, object (boxed AscMap<AscBox<AscJsonValue>>) = 5,
    }
}

asc_tagged_union! {
    /// An AssemblyScript Ethereum dynamic value.
    AscEthereumValue,
    AscEthereumValueKind,
    AscEthereumValuePayload,
    AscEthereumValueData {
        Address, address (boxed AscAddress) = 0,
        FixedBytes, fixedbytes (boxed AscBytes) = 1,
        Bytes, bytes (boxed AscBytes) = 2,
        Int, int (boxed AscBigInt) = 3,
        Uint, uint (boxed AscBigInt) = 4,
        Bool, bool (value bool) = 5,
        String, string (string) = 6,
        FixedArray, fixedarray (boxed AscArray<AscBox<AscEthereumValue>>) = 7,
        Array, array (boxed AscArray<AscBox<AscEthereumValue>>) = 8,
        Tuple, tuple (boxed AscArray<AscBox<AscEthereumValue>>) = 9,
    }
}
