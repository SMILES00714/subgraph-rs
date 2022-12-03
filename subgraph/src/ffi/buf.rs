//! AssemblyScript buffer and typed array definitions.

use super::boxed::{AscArray, AscObject, AscSlice, AscValue};
use std::{borrow::Borrow, marker::PhantomData, mem, slice};

/// An AssemblyScript ArrayBuffer.
#[repr(transparent)]
pub struct AscArrayBuffer {
    inner: AscArray<u8>,
}

impl AscArrayBuffer {
    /// Create a new array buffer with the specified data.
    pub fn new(bytes: impl AsRef<[u8]>) -> Self {
        Self {
            inner: AscArray::new(bytes.as_ref().iter().copied()),
        }
    }

    /// Returns an array buffer as a slice of bytes.
    pub fn as_bytes(&self) -> &[u8] {
        self.inner.data().as_slice()
    }
}

/// A reference to a typed array.
#[repr(transparent)]
pub struct AscTypedSlice<T> {
    inner: AscValue<View<T>>,
}

impl<T> AscTypedSlice<T>
where
    T: AscTypedArrayItem,
{
    /// Returns a slice view into the AssemblyScript typed array reference.
    pub fn as_slice(&self) -> &[T] {
        self.inner.as_slice()
    }
}

impl<T> ToOwned for AscTypedSlice<T>
where
    T: AscTypedArrayItem,
{
    type Owned = AscTypedArray<T>;

    fn to_owned(&self) -> Self::Owned {
        AscTypedArray::new(AscArrayBuffer::new(self.inner.as_bytes()))
    }
}

/// A typed array view into an array buffer.
#[repr(transparent)]
pub struct AscTypedArray<T> {
    inner: AscObject<View<T>>,
}

impl<T> AscTypedArray<T>
where
    T: AscTypedArrayItem,
{
    /// Creates a new typed array
    pub fn new(buffer: AscArrayBuffer) -> AscTypedArray<T> {
        let len = buffer.as_bytes().len();
        let trailing = len % mem::size_of::<T>();

        // `data_start` is an absolute pointer to the start of the data and not
        // relative to `buffer`. In other words, `data_start == buffer` when
        // specifyging a typed array that starts at the beginning of the array
        // buffer.
        let data_start = (buffer.inner.data() as *const AscSlice<u8>).cast();

        Self {
            inner: AscObject::new(View {
                _buffer: buffer,
                data_start,
                byte_length: len - trailing,
                _marker: PhantomData,
            }),
        }
    }

    /// Return a reference to the AssemblyScript typed array.
    pub fn as_asc_typed_slice(&self) -> &AscTypedSlice<T> {
        // SAFETY: `AscTypedSlice` has a transparent representation around an
        // `AscValue`, so it is safe to cast references to one another.
        unsafe { &*(self.inner.data() as *const AscValue<View<T>>).cast() }
    }
}

impl<T> Borrow<AscTypedSlice<T>> for AscTypedArray<T>
where
    T: AscTypedArrayItem,
{
    fn borrow(&self) -> &AscTypedSlice<T> {
        self.as_asc_typed_slice()
    }
}

#[repr(C)]
struct View<T> {
    // FIXME(nlordell): In theory, this is a reference to an array buffer.
    // However, we currently don't share array buffer data, and having the view
    // own the buffer simplifies things in a lot of places.
    _buffer: AscArrayBuffer,
    data_start: *const u8,
    byte_length: usize,
    _marker: PhantomData<*const [T]>,
}

impl<T> View<T> {
    fn as_bytes(&self) -> &[u8] {
        // SAFETY: Bounds checks for slicing is verified at construction.
        unsafe { slice::from_raw_parts(self.data_start, self.byte_length) }
    }

    fn as_slice(&self) -> &[T]
    where
        T: AscTypedArrayItem,
    {
        // SAFETY: Bounds checks for slicing is verified at construction, and
        // transmutability and alignment are guaranteed by `AscTypedArrayItem`.
        unsafe {
            slice::from_raw_parts(
                self.data_start.cast(),
                self.byte_length / mem::size_of::<T>(),
            )
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
