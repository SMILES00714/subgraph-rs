//! AssemblyScript object boxing.

use std::{
    alloc::{self, Layout},
    borrow::Borrow,
    fmt::{self, Debug, Formatter},
    marker::PhantomData,
    mem,
    ops::Deref,
    ptr::{self, NonNull},
    slice,
};

/// TODO(nlordell):
const TODO_TYPE_ID: u32 = 42;

/// The default alignment to use for AssemblyScript allocations.
///
/// Note that we **over-align** things. This just makes our life easier in
/// various places in order to avoid dealing with UB from reading from unaligned
/// pointers.
pub const ALIGN: usize = 16;

/// A boxed AssemblyScript object with a value.
///
/// This represents values as pointers into an AssemblyScript object's data and
/// is FFI safe.
#[repr(transparent)]
pub struct AscObject<T> {
    data: NonNull<u8>,
    _marker: PhantomData<*const T>,
}

impl<T> AscObject<T> {
    /// Creates a new boxed AssemblyScript value.
    pub fn new(value: T) -> Self {
        let data = unsafe {
            let data = alloc_object(TODO_TYPE_ID, Layout::new::<T>());
            data.cast::<T>().as_ptr().write(value);
            data
        };

        Self {
            data,
            _marker: PhantomData,
        }
    }

    /// Returns a reference to the inner data.
    pub fn data(&self) -> &AscValue<T> {
        // SAFETY: data points to a valid, aligned and initialized value.
        // Additionally, `AscRef` is a transparent wrapper around `T`.
        unsafe { &*self.data.as_ptr().cast() }
    }
}

impl<T> Borrow<AscValue<T>> for AscObject<T> {
    fn borrow(&self) -> &AscValue<T> {
        self.data()
    }
}

impl<T> Clone for AscObject<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        self.data().to_owned()
    }
}

impl<T> Debug for AscObject<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_tuple("AscObject")
            .field(&self.data().inner)
            .finish()
    }
}

impl<T> Drop for AscObject<T> {
    fn drop(&mut self) {
        unsafe {
            ptr::drop_in_place(self.data.as_ptr().cast::<T>());
            dealloc_object(self.data);
        }
    }
}

/// A nullable boxed AssemblyScript object.
#[repr(transparent)]
pub struct AscNullableObject<T> {
    data: *mut u8,
    _marker: PhantomData<*const T>,
}

impl<T> AscNullableObject<T> {
    /// Returns a reference to the data if it is non-null.
    pub fn data(&self) -> Option<&AscValue<T>> {
        if self.data.is_null() {
            return None;
        }

        // SAFETY: data points to a valid, aligned and initialized value.
        // Additionally, `AscValue` is a transparent wrapper around `T`.
        Some(unsafe { &*self.data.cast() })
    }
}

impl<T> Drop for AscNullableObject<T> {
    fn drop(&mut self) {
        if let Some(data) = NonNull::new(self.data) {
            drop(AscObject {
                data,
                _marker: self._marker,
            })
        }
    }
}

/// A reference to an AssemblyScript object.
#[repr(transparent)]
pub struct AscValue<T> {
    inner: T,
}

impl<T> Debug for AscValue<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_tuple("AscRef").field(&self.inner).finish()
    }
}

impl<T> Deref for AscValue<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> ToOwned for AscValue<T>
where
    T: Clone,
{
    type Owned = AscObject<T>;

    fn to_owned(&self) -> Self::Owned {
        AscObject::new(self.inner.clone())
    }
}

/// A boxed AssemblyScript array.
///
/// This is largely equivalent to an `AscObject<[T]>`. The additional type is
/// needed in order to specialize `Drop` implementations for the array items.
#[repr(transparent)]
pub struct AscArray<T> {
    data: NonNull<u8>,
    _marker: PhantomData<*const [T]>,
}

impl<T> AscArray<T> {
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
    ///
    /// # Panics
    ///
    /// Panics if the iterator is not of the specified length.
    pub fn with_len(len: usize, items: impl IntoIterator<Item = T>) -> Self {
        let layout = match Layout::array::<T>(len) {
            Ok(value) => value,
            Err(_) => alloc::handle_alloc_error(Layout::new::<T>()),
        };
        let mut items = items.into_iter();

        let data = unsafe {
            let data = alloc_object(TODO_TYPE_ID, layout);
            let drop_array_and_panic = |count| {
                drop_array::<T>(count, data);
                panic!("iterator does not match specified length");
            };

            for i in 0..len {
                let item = items
                    .next()
                    // SAFETY: The array is initalized with `i` items at this
                    // point.
                    .unwrap_or_else(|| drop_array_and_panic(i));

                data.as_ptr().cast::<T>().add(i).write(item);
            }

            if items.next().is_some() {
                drop_array_and_panic(len);
            }

            data
        };

        Self {
            data,
            _marker: PhantomData,
        }
    }

    /// Returns the array as a slice.
    pub fn data(&self) -> &AscSlice<T> {
        // SAFETY: `data` points to an allocated array where all elements are
        // initialized - this is ensured as part of its construction.
        // Additionally, `AscSlice` is a transparent wrapper around `[T; 0]`.
        unsafe { &*self.data.as_ptr().cast() }
    }
}

impl<T> Borrow<AscSlice<T>> for AscArray<T> {
    fn borrow(&self) -> &AscSlice<T> {
        self.data()
    }
}

impl<T> Clone for AscArray<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        self.data().to_owned()
    }
}

impl<T> Debug for AscArray<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_tuple("AscArray")
            .field(&self.data().as_slice())
            .finish()
    }
}

impl<T> Drop for AscArray<T> {
    fn drop(&mut self) {
        // SAFETY: `data` is a valid array pointer and is fully initialized by
        // construction.
        unsafe {
            drop_array::<T>(self.data().len(), self.data);
        }
    }
}

/// Implementation helper for dropping an array with `count` initialized items.
///
/// # Safety
///
/// Callers must ensure two things:
/// - This is only ever called on data array pointers allocated with
///   [`alloc_object`] with an array layout.
/// - That `count` items from the array are initialized.
unsafe fn drop_array<T>(count: usize, data: NonNull<u8>) {
    if mem::needs_drop::<T>() {
        for i in 0..count {
            let item = data.as_ptr().cast::<T>().add(i);
            ptr::drop_in_place(item);
        }
    }
    dealloc_object(data);
}

/// A reference to an AssemblyScript object.
#[repr(transparent)]
pub struct AscSlice<T> {
    // FIXME(nlordell): Slices are technically "unsized" types. Unfortunately,
    // unsized types are not FFI safe, and we want to use this over FFI
    // boundries.
    inner: [T; 0],
}

impl<T> AscSlice<T> {
    /// Returns the array as a slice.
    pub fn as_slice(&self) -> &[T] {
        let this = self.inner.as_ptr();

        // SAFETY: `data` is a valid object data pointer and outlives the header
        // reference we are creating here.
        let size = unsafe {
            let data = NonNull::new_unchecked(this as *mut T as *mut u8);
            let header = AscHeader::for_data(data);
            header.rt_size as usize
        };
        let len = size / mem::size_of::<T>();

        // SAFETY: `data` points to an allocated array where all elements are
        // initialized - this is ensured as part of its construction.
        unsafe { slice::from_raw_parts(this, len) }
    }
}

impl<T> Debug for AscSlice<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_tuple("AscSlice").field(&self.as_slice()).finish()
    }
}

impl<T> Deref for AscSlice<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T> ToOwned for AscSlice<T>
where
    T: Clone,
{
    type Owned = AscArray<T>;

    fn to_owned(&self) -> Self::Owned {
        AscArray::new(self.as_slice().iter().cloned())
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

impl AscHeader {
    /// Returns the object header for the specified data pointer.
    ///
    /// # Safety
    ///
    /// Callers must ensure two things:
    /// - This is only ever called on data pointers allocated with
    ///   [`alloc_object`].
    /// - That the reference to the header does not outlive the data pointer. In
    ///   particular, until [`dealloc_object`] is called.
    unsafe fn for_data<'a>(data: NonNull<u8>) -> &'a AscHeader {
        let header_size = mem::size_of::<AscHeader>();

        // SAFETY: `data` was allocated with [`alloc_object`] and so is
        // prepended with a header. Therefore, doing this pointer arithmetic is
        // valid and will be well-aligned. Additionally, we initialized all
        // header fields so it is safe to take a reference to it.
        &*data.as_ptr().sub(header_size).cast::<AscHeader>()
    }
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
    ptr::addr_of_mut!((*header).gc_info2).write(offset);
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
    // SAFETY: `data` is a valid object (ensured by the caller) and the header
    // reference does not outlive the data pointer, as it is dropped before we
    // actually de-allocate the pointer.
    let (size, align, offset) = {
        let header = AscHeader::for_data(data);
        (header.mm_info, header.gc_info, header.gc_info2)
    };

    // SAFETY: Layout is valid because we used it for allocation!
    let layout = Layout::from_size_align_unchecked(size, align);

    // SAFETY: The root pointer is valid as that was the original allocation
    // with the layout that we just computed.
    let root = data.as_ptr().sub(offset);
    alloc::dealloc(root, layout)
}
