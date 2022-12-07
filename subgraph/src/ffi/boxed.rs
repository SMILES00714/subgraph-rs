//! AssemblyScript object boxing.

use std::{
    alloc::{self, Layout},
    borrow::{Borrow, Cow},
    fmt::{self, Debug, Formatter},
    iter::FromIterator,
    mem,
    ops::Deref,
    ptr::{self, NonNull},
    slice,
};

/// FIXME(nlordell): We currently don't use AssemblyScript type IDs at all
/// internally in the module. Additionally, the Subgraph host only uses these to
/// be compatible with the AssemblyScript runtime, which isn't a concern for us.
/// For completeness, this should be implemented in the future in case the host
/// starts requiring these values to be correct.
pub const TYPE_ID: u32 = 0;

/// The default alignment to use for AssemblyScript allocations.
///
/// Note that we **over-align** things. This just makes our life easier in
/// various places in order to avoid dealing with UB from reading from unaligned
/// pointers.
pub const ALIGN: usize = 16;

/// A boxed AssemblyScript value.
///
/// This represents values as pointers into an AssemblyScript wrapped values
/// which are preceeded by an object header and are FFI safe.
#[repr(transparent)]
pub struct AscBox<T>
where
    T: AscBoxed + ?Sized,
{
    data: NonNull<T::Target>,
}

/// A boxed value.
///
/// This trait is used internally to specialize AssemblyScript boxed value
/// implementations in order to support unsized types without requiring DSTs
/// (which are not FFI safe).
///
/// # Safety
///
/// This trait is an implementation detail of [`AscBox`] and should not be
/// implemented. That being said, implementors of this trait need to guarantee
/// a couple of things:
/// - A [`AscBox`] value of `Self` points to a `Self::Target`. For sized types,
///   this means `Self::Target = Self` and for arrays, this means `Self::Target`
///   is the array item (so `[T]::Target = T`).
/// - That `AscBox` is transmutable to `&Self::Ref`.
pub unsafe trait AscBoxed {
    type Target: Sized;
    type Ref: Borrow<Self> + Sized;
}

unsafe impl<T> AscBoxed for T
where
    T: Sized,
{
    type Target = T;
    type Ref = AscRef<T>;
}

unsafe impl<T> AscBoxed for [T]
where
    T: Sized,
{
    type Target = T;
    type Ref = AscSlice<T>;
}

impl<T> AscBox<T> {
    /// Creates a new boxed AssemblyScript value.
    pub fn new(value: T) -> Self {
        let data = unsafe {
            let data = alloc_box::<T>(TYPE_ID, 1);
            data.as_ptr().write(value);
            data
        };

        Self { data }
    }
}

impl<T> AscBox<[T]> {
    /// Creates a new boxed array value for a slice of T.
    pub fn from_slice(items: &[T]) -> Self
    where
        T: Copy,
    {
        // SAFETY: `T` is copy, and the pointers are non-overalapping.
        let data = unsafe {
            let data = alloc_box::<T>(TYPE_ID, items.len());
            ptr::copy_nonoverlapping(items.as_ptr(), data.as_ptr(), items.len());
            data
        };

        Self { data }
    }

    /// Creates a new boxed array value with the specified length.
    ///
    /// # Panics
    ///
    /// Panics if the iterator is not of the specified length.
    pub fn with_len(len: usize, items: impl IntoIterator<Item = T>) -> Self {
        let mut items = items.into_iter();
        let data = unsafe {
            let data = alloc_box::<T>(TYPE_ID, len);
            let drop_array_and_panic = |count| {
                drop_box::<T>(data, count);
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

        Self { data }
    }
}

impl<T> AscBox<T>
where
    T: AscBoxed + ?Sized,
{
    /// Returns a reference to the AssemblyScript value.
    pub fn as_asc_ref(&self) -> &T::Ref {
        // SAFETY: [`AscBoxed`] trait implementation guarantees that `self` is
        // transmutable to `&T::Ref`.
        unsafe { &*self.data.as_ptr().cast() }
    }

    /// Returns a pointer to the AssemblyScript value reference.
    pub fn as_ptr(&self) -> *const T::Ref {
        self.as_asc_ref() as _
    }

    /// Returns the AssemblyScript slice as a borrowed copy-on-write pointer.
    pub fn borrowed(&self) -> AscCow<T>
    where
        T::Ref: ToOwned<Owned = Self>,
    {
        Cow::Borrowed(self.as_asc_ref())
    }

    /// Returns the AssemblyScript slice as an owned copy-on-write pointer.
    pub fn owned(self) -> AscCow<'static, T>
    where
        T::Ref: ToOwned<Owned = Self>,
    {
        Cow::Owned(self)
    }
}

impl<T> Borrow<AscRef<T>> for AscBox<T> {
    fn borrow(&self) -> &AscRef<T> {
        self.as_asc_ref()
    }
}

impl<T> Borrow<AscSlice<T>> for AscBox<[T]> {
    fn borrow(&self) -> &AscSlice<T> {
        self.as_asc_ref()
    }
}

impl<T> Clone for AscBox<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        self.as_asc_ref().to_owned()
    }
}

impl<T> Clone for AscBox<[T]>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        self.as_asc_ref().to_owned()
    }
}

impl<T> Debug for AscBox<T>
where
    T: AscBoxed + Debug + ?Sized,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_tuple("AscBox")
            .field(&self.as_asc_ref().borrow())
            .finish()
    }
}

impl<T> Drop for AscBox<T>
where
    T: AscBoxed + ?Sized,
{
    fn drop(&mut self) {
        unsafe {
            // SAFETY: `data` is a valid data pointer and outlives the header
            // reference we are creating here.
            let len = {
                let header = AscHeader::for_data(self.data.as_ptr());
                header.len::<T::Target>()
            };
            drop_box(self.data, len);
        }
    }
}

impl<T> FromIterator<T> for AscBox<[T]> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let iter = iter.into_iter();
        let (len, max_len) = iter.size_hint();
        if Some(len) != max_len {
            panic!("cannot collect iterator with unknown size");
        }

        Self::with_len(len, iter)
    }
}

/// A nullable boxed AssemblyScript value.
///
/// This type is largely equivalent to an `Option<AscBox>` but is FFI-safe.
#[repr(transparent)]
pub struct AscNullableBox<T>
where
    T: AscBoxed + ?Sized,
{
    data: *mut T::Target,
}

impl<T> AscNullableBox<T>
where
    T: AscBoxed + ?Sized,
{
    /// Returns a reference to the data if it is non-null.
    pub fn as_asc_ref(&self) -> Option<&T::Ref> {
        if self.data.is_null() {
            return None;
        }

        // SAFETY: [`AscBoxed`] trait implementation guarantees that `self` is
        // transmutable to `&T::Ref`.
        Some(unsafe { &*self.data.cast() })
    }
}

impl<T> Drop for AscNullableBox<T>
where
    T: AscBoxed + ?Sized,
{
    fn drop(&mut self) {
        if let Some(data) = NonNull::new(self.data) {
            drop(AscBox::<T> { data })
        }
    }
}

/// Copy-on-write AssemblyScript boxed value.
pub type AscCow<'a, T> = Cow<'a, <T as AscBoxed>::Ref>;

/// A reference to an AssemblyScript value.
#[repr(transparent)]
pub struct AscRef<T> {
    inner: T,
}

impl<T> AscRef<T> {
    /// Returns the AssemblyScript slice as a copy-on-write pointer.
    pub fn borrowed(&self) -> AscCow<T>
    where
        T: Clone,
    {
        Cow::Borrowed(self)
    }

    /// Returns the AssemblyScript value reference as a pointer.
    pub fn as_ptr(&self) -> *const Self {
        self as _
    }
}

impl<T> Borrow<T> for AscRef<T> {
    fn borrow(&self) -> &T {
        &self.inner
    }
}

impl<T> Debug for AscRef<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_tuple("AscRef").field(&self.inner).finish()
    }
}

impl<T> Deref for AscRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> ToOwned for AscRef<T>
where
    T: Clone,
{
    type Owned = AscBox<T>;

    fn to_owned(&self) -> Self::Owned {
        AscBox::new(self.inner.clone())
    }
}

/// A reference to an AssemblyScript array value.
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

        // SAFETY: `data` is a valid data pointer and outlives the header
        // reference we are creating here.
        let len = unsafe {
            let header = AscHeader::for_data(this);
            header.len::<T>()
        };

        // SAFETY: `data` points to an allocated array where all elements are
        // initialized - this is ensured as part of its construction.
        unsafe { slice::from_raw_parts(this, len) }
    }
}

impl<T> Borrow<[T]> for AscSlice<T> {
    fn borrow(&self) -> &[T] {
        self.as_slice()
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
    type Owned = AscBox<[T]>;

    fn to_owned(&self) -> Self::Owned {
        self.as_slice().iter().cloned().collect()
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
    /// - This is only called on data pointers allocated with [`alloc_box`].
    /// - That the reference to the header does not outlive the data pointer. In
    ///   particular, until [`drop_box`] is called.
    unsafe fn for_data<'a, T>(data: *const T) -> &'a AscHeader {
        let header_size = mem::size_of::<AscHeader>();

        // SAFETY: `data` was allocated with [`alloc_box`] and so is prepended
        // with a header. Therefore, doing this pointer arithmetic is valid and
        // will be well-aligned. Additionally, we initialized all header fields
        // so it is safe to take a reference to it.
        &*data.cast::<u8>().sub(header_size).cast::<AscHeader>()
    }

    /// Returns the length of items followed by this header for the specified
    /// type.
    fn len<T>(&self) -> usize {
        (self.rt_size as usize) / mem::size_of::<T>()
    }
}

/// Allocates an AssemblyScript value and returns a pointer to **uninitialized**
/// data for the specified layout.
///
/// Data values allocated this way are always proceeded in memory by a valid
/// [`AscHeader`].
fn alloc_box<T>(type_id: u32, len: usize) -> NonNull<T> {
    let header_size = mem::size_of::<AscHeader>();
    let data_layout = match Layout::array::<T>(len) {
        Ok(value) => value,
        Err(_) => alloc::handle_alloc_error(Layout::new::<T>()),
    };

    // NOTE: We create a layout that guarantees:
    // - None of the header fields are mis-aligned
    // - The data that comes after the header is also not mis-aligned
    //
    // layout:
    // +---------+--------+-----------
    // | padding | header | data ...
    // +---------+--------+-----------
    //                    ^
    //                  offset (align(16))
    let (layout, offset) = {
        // SAFETY: `AscHeader` has a non-zero size and the default alignment
        // is valid and doesn't cause size overflows.
        let layout =
            unsafe { Layout::from_size_align_unchecked(header_size, ALIGN).pad_to_align() };

        let (layout, offset) = match layout.extend(data_layout) {
            Ok(value) => value,
            Err(_) => alloc::handle_alloc_error(data_layout),
        };

        // NOTE: Pad the final layout to ensure C ABI compatibility.
        let layout = layout.pad_to_align();
        (layout, offset)
    };

    // SAFETY: Layout is guaranteed to have a non-zero size because of the
    // object header.
    let root = unsafe { alloc::alloc(layout) };
    if root.is_null() {
        alloc::handle_alloc_error(layout);
    }

    // SAFETY: Pointer was just allocated so it is valid, and is guaranteed to
    // be well-aligned because of our padding strategy.
    unsafe {
        let header = root.add(offset - header_size).cast::<AscHeader>();
        ptr::addr_of_mut!((*header).mm_info).write(layout.size());
        ptr::addr_of_mut!((*header).gc_info).write(layout.align());
        ptr::addr_of_mut!((*header).gc_info2).write(offset);
        ptr::addr_of_mut!((*header).rt_id).write(type_id);
        ptr::addr_of_mut!((*header).rt_size).write(data_layout.size() as _);
    }

    // SAFETY: Data pointer is valid, non-null and well-aligned.
    unsafe { NonNull::new_unchecked(root.add(offset).cast()) }
}

/// Drops an AssemblyScript box pointer created with the [`alloc_box`] function.
///
/// # Safety
///
/// Callers must ensure two things:
/// - This is only ever called on data pointers allocated with [`alloc_box`].
/// - That `len` items pointed to by `data` are initialized.
unsafe fn drop_box<T>(data: NonNull<T>, len: usize) {
    // SAFETY: `data` is a valid object (ensured by the caller) and the header
    // reference does not outlive the data pointer, as it is dropped before we
    // actually de-allocate the pointer.
    let (size, align, offset) = {
        let header = AscHeader::for_data(data.as_ptr());
        (header.mm_info, header.gc_info, header.gc_info2)
    };

    if mem::needs_drop::<T>() {
        for i in 0..len {
            let item = data.as_ptr().add(i);
            ptr::drop_in_place(item);
        }
    }

    // SAFETY: Layout is valid because we used it for allocation!
    let layout = Layout::from_size_align_unchecked(size, align);

    // SAFETY: The root pointer is valid as that was the original allocation
    // with the layout that we just computed.
    let root = data.cast::<u8>().as_ptr().sub(offset);
    alloc::dealloc(root, layout)
}
