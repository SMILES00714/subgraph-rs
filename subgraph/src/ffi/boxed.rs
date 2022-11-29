//! AssemblyScript object boxing.

use super::sys::AscObject;
use std::{
    alloc::{self, Layout},
    marker::PhantomData,
    mem::{self, MaybeUninit},
    ptr::{self, NonNull},
};

/// The default alignment to use for AssemblyScript allocations.
///
/// Note that we **over-align** things. This just makes our life easier in
/// various places in order to avoid dealing with UB from reading from unaligned
/// pointers.
pub const ALIGN: usize = 16;

/// An pointer to an AssemblyScript object's data.
///
/// AssemblyScript requires pointers to managed data to be proceeded by the
/// object header. This type can only be constructed from a boxed
/// AssemblyScript value, which guarantees that the data is correctly
/// prepended by the required header.
#[repr(transparent)]
pub struct AscPtr<T>(*const T);

/// A boxed AssemblyScript value.
///
/// This represents values as pointers into an AssemblyScript object's data.
/// This also implies that Rust DSTs are represented as "regular pointers" as
/// the size information is stored in the object header.
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
    pub fn as_asc_ptr(&self) -> AscPtr<T> {
        AscPtr(self.ptr.as_ptr().cast())
    }
}

impl<T> AscBox<[T]> {
    /// Creates a new boxed array value for the exact sized iterator.
    pub fn new_array<I>(items: I) -> Self
    where
        I: IntoIterator<Item = T>,
        I::IntoIter: ExactSizeIterator,
    {
        let items = items.into_iter();
        Self::array_with_len(items.len(), items)
    }

    /// Creates a new boxed array value with the specified length.
    pub fn array_with_len(len: usize, items: impl IntoIterator<Item = T>) -> Self {
        todo!()
    }

    /// Returns the AssemblyScript managed object as an array.
    ///
    /// This is a work around for their not being a stable way for constructing
    /// DST values.
    pub fn as_array_object(&self) -> AscPtr<[T; 0]> {
        AscPtr(self.ptr.as_ptr().cast())
    }
}

/// AssemblyScript object header.
#[repr(C)]
struct AscHeader {
    mm_info: usize,
    gc_info: usize,
    gc_info2: usize,
    rt_id: u32,
    rt_size: u32,
}

/// Allocates an AssemblyScript object and returns a mutable reference to the
/// object header, as well as a pointer to **uninitialized** data for the
/// specified layout.
///
/// # Safety
///
/// ?
unsafe fn alloc_object(type_id: u32, data_layout: Layout) -> NonNull<u8> {
    let header_size = mem::size_of::<AscHeader>();

    // SAFETY: `AscHeader` has a non-zero size and the default alignment
    // is valid and doesn't cause size overflows.
    let layout = Layout::from_size_align_unchecked(header_size, ALIGN);

    // NOTE: We pad the layout to a large default alignment ensuring that:
    // - None of the header fields are mis-aligned
    // - The data that comes after the header is also not mis-aligned
    let layout = layout.pad_to_align();

    let (layout, offset) = match layout.extend(data_layout) {
        Ok(value) => value,
        Err(_) => alloc::handle_alloc_error(data_layout),
    };

    // NOTE: Pad the final layout to ensure C ABI compatibility.
    let layout = layout.pad_to_align();

    // SAFETY: Layout is guaranteed to have a non-zero size because of the
    // object header.
    let root = alloc::alloc(layout);
    if root.is_null() {
        alloc::handle_alloc_error(layout);
    }

    // SAFETY: Pointer was just allocated so it is valid, and is guaranteed to
    // be well-aligned because of our padding strategy.
    let header = root.add(offset - header_size).cast::<AscHeader>();
    ptr::addr_of_mut!((*header).mm_info).write(layout.size());
    ptr::addr_of_mut!((*header).gc_info).write(layout.align());
    ptr::addr_of_mut!((*header).mm_info).write(offset);
    ptr::addr_of_mut!((*header).rt_id).write(type_id);
    ptr::addr_of_mut!((*header).rt_size).write(data_layout.size() as _);

    // SAFETY: Data pointer is valid, non-null and well-aligned.
    NonNull::new_unchecked(root.add(offset))
}

/// Deallocates an AssemblyScript object created with the [`alloc_object`]
/// function.
///
/// # Safety
///
/// This method only works for data pointers allocated with [`alloc_object`].
unsafe fn dealloc_object(data: NonNull<u8>) {
    let header_size = mem::size_of::<AscHeader>();

    // SAFETY: `data` was allocated with [`alloc_object`] and so is prepended
    // with a header. Therefore, doing this pointer arithmetic is valid and
    // will be well-aligned. Additionally, we initialized all header fields so
    // it is safe to take a reference to it.
    let header = &*data.as_ptr().sub(header_size).cast::<AscHeader>();
    let (size, align, offset) = (header.mm_info, header.gc_info, header.gc_info2);

    // SAFETY: Layout is valid because we used it for allocation!
    let layout = Layout::from_size_align_unchecked(size, align);

    // SAFETY: The root pointer is valid as that was the original allocation
    // with the layout that we just computed.
    let root = data.as_ptr().sub(offset);
    alloc::dealloc(root, layout)
}
