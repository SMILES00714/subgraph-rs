//! AssemblyScript string.

use super::boxed::{AscArray, AscSlicePtr};
use std::{
    fmt::{self, Debug, Formatter},
    mem,
    ops::Deref,
    string::FromUtf16Error,
};

/// A reference to an AssemblyScript string.
pub struct AscStr {
    inner: [u16],
}

impl AscStr {
    /// Converts the AssemblyScript string into a Rust `String`.
    pub fn to_string(&self) -> Result<String, FromUtf16Error> {
        String::from_utf16(self.as_code_points())
    }

    /// Converts the AssemblyScript string into a Rust `String`, replacing
    /// invalid data with the replacement character (`U+FFFD`).
    pub fn to_string_lossy(&self) -> String {
        String::from_utf16_lossy(self.as_code_points())
    }

    /// Returns a slice of the utf-16 code points for this string.
    pub fn as_code_points(&self) -> &[u16] {
        &self.inner
    }
}

impl Debug for AscStr {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self.to_string_lossy(), f)
    }
}

/// An owned AssemblyScript string.
pub struct AscString {
    inner: AscArray<u16>,
}

impl AscString {
    /// Creates a new AssemblyScript string from a Rust string slice.
    pub fn new(s: impl AsRef<str>) -> Self {
        let s = s.as_ref();

        let len = s.encode_utf16().count();
        let inner = AscArray::with_len(len, s.encode_utf16());

        Self { inner }
    }

    /// Returns a reference to a borrowed AssemblyScript string.
    pub fn as_asc_str(&self) -> &AscStr {
        // SAFETY: DSTs are a bit of a mystery to me... To my understanding
        // this should be safe because `Bytes` has a transparent layout, so
        // `&[u16]` and `&AscStr`. Either way, we should get a fat pointer
        // with the correct length and pointing to the start of the slice and
        // transmuting between them should be safe.
        unsafe { mem::transmute(self.inner.as_slice()) }
    }

    /// Returns the [`AscStr`] as a pointer.
    ///
    /// TODO(nlordell): Move this to `AscStr`.
    pub fn as_asc_ptr(&self) -> AscSlicePtr<u16> {
        self.inner.as_asc_ptr()
    }
}

impl Deref for AscString {
    type Target = AscStr;

    fn deref(&self) -> &Self::Target {
        self.as_asc_str()
    }
}

impl Debug for AscString {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self.as_asc_str(), f)
    }
}
