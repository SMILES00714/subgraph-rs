//! AssemblyScript string.

use super::boxed::{AscBox, AscNullableBox, AscSlice};
use std::{
    borrow::Borrow,
    fmt::{self, Debug, Formatter},
    mem,
    ops::Deref,
};

/// A reference to an AssemblyScript string.
#[repr(C)]
pub struct AscStr {
    inner: AscSlice<u16>,
}

impl AscStr {
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
        f.debug_tuple("AscStr")
            .field(&self.to_string_lossy())
            .finish()
    }
}

impl ToOwned for AscStr {
    type Owned = AscString;

    fn to_owned(&self) -> Self::Owned {
        AscString {
            inner: self.inner.to_owned(),
        }
    }
}

/// An owned AssemblyScript string.
#[derive(Clone)]
#[repr(transparent)]
pub struct AscString {
    inner: AscBox<[u16]>,
}

impl AscString {
    /// Creates a new AssemblyScript string from a Rust string slice.
    pub fn new(s: &str) -> Self {
        let len = s.encode_utf16().count();
        let inner = AscBox::with_len(len, s.encode_utf16());

        Self { inner }
    }

    /// Returns a reference to a borrowed AssemblyScript string.
    pub fn as_asc_str(&self) -> &AscStr {
        // SAFETY: `AscStr` is a transparent wrapper around `AscSlice`.
        unsafe { &*self.inner.as_ptr().cast() }
    }

    /// Returns a pointer to an AssemblyScript string.
    pub fn as_ptr(&self) -> *const AscStr {
        self.as_asc_str() as _
    }
}

impl Borrow<AscStr> for AscString {
    fn borrow(&self) -> &AscStr {
        self.as_asc_str()
    }
}

impl Debug for AscString {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_tuple("AscString")
            .field(&self.to_string_lossy())
            .finish()
    }
}

impl Deref for AscString {
    type Target = AscStr;

    fn deref(&self) -> &Self::Target {
        self.as_asc_str()
    }
}

/// A nullable AssemblyScript string.
#[repr(transparent)]
pub struct AscNullableString {
    inner: AscNullableBox<[u16]>,
}

impl AscNullableString {
    /// Returns a reference to a borrowed AssemblyScript string.
    pub fn as_asc_str(&self) -> Option<&AscStr> {
        let slice = self.inner.as_asc_ref()?;
        // SAFETY: `AscStr` is a transparent wrapper around `AscSlice`.
        Some(unsafe { mem::transmute(slice) })
    }
}
