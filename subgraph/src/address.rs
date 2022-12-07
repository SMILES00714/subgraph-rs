//! 20-byte Ethereum addresses.

use crate::{
    conv,
    ffi::{
        str::AscString,
        sys::{self, AscAddress},
    },
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

    /// Returns a new address from its string reprensentation.
    pub fn parse(str: impl AsRef<str>) -> Self {
        str.as_ref().parse().unwrap()
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
        let str = AscString::new(s);
        let bytes = unsafe { &*sys::type_conversion__string_to_h160(str.as_ptr()) };
        Ok(Self::from_raw(bytes))
    }
}
