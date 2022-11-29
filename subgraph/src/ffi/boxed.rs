//! AssemblyScript object boxing.

use super::sys::AscObject;
use std::{
    marker::PhantomData,
    mem::MaybeUninit,
    ptr::{self, NonNull},
};

/// A boxed AssemblyScript value.
///
/// This represents values as pointers into [`AscObject`] data. This means that
/// Rust DSTs are represented as "regular pointers" as the size information is
/// stored in the object header.
#[repr(transparent)]
pub struct AscBox<T>
where
    T: ?Sized,
{
    ptr: NonNull<u8>,
    _marker: PhantomData<*const T>,
}

impl<T> AscBox<T> {
    /// Returns the AssemblyScript managed object for the current boxed value.
    pub fn as_object(&self) -> &AscObject<T> {
        header_ref(self.ptr.as_ptr())
    }
}

impl<T> AscBox<[T]> {
    /// Creates a new boxed array value for the exact sized iterator.
    pub fn new<I>(items: I) -> Self
    where
        I: IntoIterator<Item = T>,
        I::IntoIter: ExactSizeIterator,
    {
        let items = items.into_iter();
        Self::with_len(items.len(), items)
    }

    /// Creates a new boxed array value with the specified length.
    pub fn with_len(len: usize, items: impl IntoIterator<Item = T>) -> Self {
        let mut string = unsafe {
            alloc_array(len)
                .expect("attempted to allocate a string that is larger than the address space.")
        };
        string.inner.len = len;
        for (i, c) in s.encode_utf16().enumerate() {
            string.inner.buf[i] = c;
        }

        string
    }

    /// Returns the AssemblyScript managed object as an array.
    ///
    /// This is a work around for their not being a stable way for constructing
    /// DST values.
    pub fn as_array_object(&self) -> &AscObject<[T; 0]> {
        header_ref(self.ptr.as_ptr())
    }
}

#[inline]
fn header_ref<'a, T>(data: *const u8) -> &'a AscObject<T> {
    // SAFETY: `AscBox` is always allocated with an object header and then made
    // to point to its data. This means that offsetting is safe since both
    // locations are valid in memory.
    unsafe { &*data.offset(header_offset::<T>()).cast::<AscObject<T>>() }
}

#[inline]
fn header_offset<T>() -> isize {
    // SAFETY: `addr_of` creates a pointer to a field without an intermidiary
    // reference, meaning this is safe. For example of how this is used:
    // <https://doc.rust-lang.org/beta/std/mem/union.MaybeUninit.html#initializing-a-struct-field-by-field>
    unsafe {
        let header = MaybeUninit::<AscObject<T>>::uninit().as_ptr();
        let field = ptr::addr_of!((*header).data);
        (header as *const u8).offset_from(field as *const u8)
    }
}
