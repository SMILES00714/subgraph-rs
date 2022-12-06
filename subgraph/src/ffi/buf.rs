//! AssemblyScript buffer and typed array definitions.

use super::boxed::AscBox;
use std::{mem, slice};

/// An AssemblyScript ArrayBuffer.
#[derive(Clone)]
#[repr(transparent)]
pub struct AscArrayBuffer {
    inner: AscBox<[u8]>,
}

impl AscArrayBuffer {
    /// Create a new array buffer with the specified data.
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            inner: AscBox::from_slice(bytes),
        }
    }

    /// Returns an array buffer as a slice of bytes.
    pub fn as_bytes(&self) -> &[u8] {
        self.inner.as_asc_ref().as_slice()
    }

    /// Returns a pointer to the buffer data.
    pub fn as_ptr(&self) -> *const u8 {
        self.as_bytes().as_ptr()
    }
}

/// A typed array view into an array buffer.
#[repr(C)]
pub struct AscTypedArray<T> {
    // FIXME(nlordell): In theory, this is a reference to an array buffer.
    // However, we currently don't share array buffer data, and having the view
    // own the buffer simplifies things in a lot of places.
    buffer: AscArrayBuffer,
    data_start: *const T,
    byte_length: usize,
}

impl<T> AscTypedArray<T>
where
    T: AscTypedArrayItem,
{
    /// Creates a new typed array
    pub fn new(buffer: AscArrayBuffer) -> AscBox<Self> {
        let len = buffer.as_bytes().len();
        let trailing = len % mem::size_of::<T>();

        // `data_start` is an absolute pointer to the start of the data and not
        // relative to `buffer`. In other words, `data_start == buffer` when
        // specifyging a typed array that starts at the beginning of the array
        // buffer.
        let data_start = buffer.as_ptr().cast();

        AscBox::new(Self {
            buffer,
            data_start,
            byte_length: len - trailing,
        })
    }

    /// Returns a slice view into the AssemblyScript typed array.
    pub fn as_slice(&self) -> &[T] {
        // SAFETY: Bounds checks for slicing is verified at construction, and
        // transmutability and alignment are guaranteed by `AscTypedArrayItem`.
        unsafe { slice::from_raw_parts(self.data_start, self.byte_length / mem::size_of::<T>()) }
    }
}

impl AscTypedArray<u8> {
    /// Creates a `u8` view into an array buffer.
    pub fn from_bytes(bytes: &[u8]) -> AscBox<Self> {
        AscTypedArray::new(AscArrayBuffer::new(bytes))
    }
}

impl<T> Clone for AscTypedArray<T>
where
    T: AscTypedArrayItem,
{
    fn clone(&self) -> Self {
        let buffer = self.buffer.clone();
        let data_start = buffer.as_ptr().cast();

        Self {
            buffer,
            data_start,
            byte_length: self.byte_length,
        }
    }
}

/// A marker trait indicating that a type can be used by a typed array view.
///
/// # Safety
///
/// `T` must be transmutable from `[u8; size_of::<T>()]` and
/// `align_of::<T>() <= 16`.
pub unsafe trait AscTypedArrayItem {}

// SAFETY: u8 can be transmuted from [u8; 1].
unsafe impl AscTypedArrayItem for u8 {}
