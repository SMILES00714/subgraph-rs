use crate::ffi::{
    boxed::{AscCow, AscRef},
    num::{AscBigDecimal, AscBigInt},
    sys,
};
use std::fmt::{self, Debug, Display, Formatter, LowerHex, UpperHex};

/// A arbitrarily sized integer type.
#[derive(Clone)]
pub struct BigInt {
    inner: AscCow<'static, AscBigInt>,
}

impl BigInt {
    pub(crate) fn from_raw(raw: &'static AscRef<AscBigInt>) -> Self {
        Self {
            inner: raw.borrowed(),
        }
    }

    pub(crate) fn as_raw(&self) -> &AscRef<AscBigInt> {
        &self.inner
    }

    // TODO(nlordell): Implement proper `BigInt` construction.
    pub fn temp_new(x: i8) -> Self {
        Self {
            inner: AscBigInt::from_bytes([x as _].as_slice()).owned(),
        }
    }

    /// Returns the sign of the integer.
    pub fn signum(&self) -> i32 {
        let last = self.inner.as_slice().last().copied().unwrap_or_default();
        ((last as i8) as i32).signum()
    }
}

impl Debug for BigInt {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // NOTE: Work around `Formatter::debug_{lower,upper}_hex` being private
        // and not stabilized.
        #[allow(deprecated)]
        let flags = f.flags();
        const DEBUG_LOWER_HEX: u32 = 1 << 4;
        const DEBUG_UPPER_HEX: u32 = 1 << 5;

        if flags & DEBUG_LOWER_HEX != 0 {
            LowerHex::fmt(self, f)
        } else if flags & DEBUG_UPPER_HEX != 0 {
            UpperHex::fmt(self, f)
        } else {
            Display::fmt(self, f)
        }
    }
}

impl Display for BigInt {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let str = unsafe { &*sys::type_conversion__big_int_to_string(self.inner.as_ptr()) };

        let str = str.to_string_lossy();
        let (is_non_negative, abs) = match str.strip_prefix('-') {
            Some(abs) => (false, abs),
            None => (true, str.as_str()),
        };

        f.pad_integral(is_non_negative, "", abs)
    }
}

impl LowerHex for BigInt {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        fmt_hex(self, f, str::make_ascii_lowercase)
    }
}

impl UpperHex for BigInt {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        fmt_hex(self, f, str::make_ascii_uppercase)
    }
}

fn fmt_hex(value: &BigInt, f: &mut Formatter, transform: impl FnOnce(&mut str)) -> fmt::Result {
    let str = unsafe { &*sys::type_conversion__big_int_to_hex(&*value.inner as _) };

    let mut str = str.to_string_lossy();
    let str = match str.starts_with("0x") {
        true => unsafe { str.get_unchecked_mut(2..) },
        false => str.as_mut_str(),
    };

    transform(str);
    // NOTE: Unexpectedly, negative numbers are being encoded as the hex of
    // their absolute value. This means we manually want to check whether or not
    // the number is negative in Rust and not rely on the host.
    let is_non_negative = value.signum() >= 0;
    let abs = str.strip_prefix('-').unwrap_or(str);

    f.pad_integral(is_non_negative, "0x", abs)
}

/// An arbitrary precision decimal number.
#[derive(Clone)]
pub struct BigDecimal {
    inner: AscCow<'static, AscBigDecimal>,
}

impl BigDecimal {
    pub(crate) fn from_raw(raw: &'static AscRef<AscBigDecimal>) -> Self {
        Self {
            inner: raw.borrowed(),
        }
    }

    pub(crate) fn as_raw(&self) -> &AscRef<AscBigDecimal> {
        &self.inner
    }

    /// Creates a new integer value from the specified [`BigInt`].
    pub fn new(value: BigInt) -> Self {
        let value = AscBigDecimal::new(value.inner.into_owned(), AscBigInt::from_bytes(&[0]));
        Self {
            inner: value.owned(),
        }
    }
}

impl Debug for BigDecimal {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for BigDecimal {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let str = unsafe { &*sys::big_decimal__to_string(self.inner.as_ptr()) };

        let str = str.to_string_lossy();
        let (is_non_negative, abs) = match str.strip_prefix('-') {
            Some(abs) => (false, abs),
            None => (true, str.as_str()),
        };

        f.pad_integral(is_non_negative, "", abs)
    }
}
