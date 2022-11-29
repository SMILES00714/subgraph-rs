//! Subgraph host imports.
//!
//! This module just declares the "raw" host methods for WASM imports.

/// A AssemblyScript object header.

/// A managed AssemblyScript object.
#[repr(C)]
pub struct AscObject<T>
where
    T: ?Sized,
{
    _padding: [u32; 0],

    pub mm_info: usize,
    pub gc_info: usize,
    pub gc_info2: usize,
    pub rt_id: u32,
    pub rt_size: u32,
    pub data: T,
}

impl<T> AscObject<T> {
    pub fn data_ptr(&self) -> AscPtr<T> {
        AscPtr(&self.data as _)
    }
}

impl<T> AscObject<[T]> {
    pub fn data_slice_ptr(&self) -> AscPtr<T> {
        AscPtr(self.data.as_ptr())
    }
}

/// An pointer to an AssemblyScript object's data.
///
/// AssemblyScript requires pointers to managed data to be proceeded by the
/// object header, so we can't use raw pointers for FFI.
#[repr(transparent)]
pub struct AscPtr<T>(*const T);

#[link(wasm_import_module = "log")]
extern "C" {
    pub fn log(level: u32, message: AscPtr<u16>);
}
