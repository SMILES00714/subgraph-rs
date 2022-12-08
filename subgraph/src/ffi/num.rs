//! FFI numerical types.

use super::{
    boxed::{AscBox, AscRef},
    buf::AscTypedArray,
};

/// A big integer.
pub type AscBigInt = AscTypedArray<u8>;

/// A big decimal value.
#[derive(Clone)]
#[repr(C)]
pub struct AscBigDecimal {
    // FIXME(nlordell): In theory, this is a reference to a big int. However,
    // dealing with lifetimes of sharing references is not worth the effort.
    digits: AscBox<AscBigInt>,
    exp: AscBox<AscBigInt>,
}

impl AscBigDecimal {
    /// Create a new FFI big decimal value.
    pub fn new(digits: AscBox<AscBigInt>, exp: AscBox<AscBigInt>) -> AscBox<Self> {
        AscBox::new(Self { digits, exp })
    }

    /// Gets the digits.
    pub fn digits(&self) -> &AscRef<AscBigInt> {
        self.digits.as_asc_ref()
    }
}
