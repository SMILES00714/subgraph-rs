//! 20-byte Ethereum addresses.

use crate::{
    conv,
    ffi::{boxed::AscBox, str::AscString, sys, types::AscAddress},
};
use std::{
    convert::Infallible,
    fmt::{self, Debug, Display, Formatter},
    str::FromStr,
};

/// An Ethereum address.
#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Address(pub [u8; 20]);

impl Address {
    /// Returns an address from its raw byte representation.
    pub(crate) fn from_raw(bytes: &AscAddress) -> Self {
        Self(bytes.as_slice().try_into().unwrap())
    }

    /// Returns the address as an AssemblyScript value.
    pub(crate) fn to_raw(self) -> AscBox<AscAddress> {
        AscAddress::from_bytes(&self.0)
    }

    /// Returns a new address from its string reprensentation.
    pub fn parse(s: impl AsRef<str>) -> Self {
        let str = AscString::new(s.as_ref());
        let bytes = unsafe { &*sys::type_conversion__string_to_h160(str.as_ptr()) };
        Self::from_raw(bytes)
    }
}

impl Debug for Address {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_tuple("Address")
            .field(&format_args!("{self}"))
            .finish()
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let hex = conv::hex(self.0.as_slice());
        f.write_str(&hex)
    }
}

impl FromStr for Address {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::parse(s))
    }
}
