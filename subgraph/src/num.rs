use crate::ffi::{
    boxed::{AscCow, AscRef},
    num::{AscBigDecimal, AscBigInt},
    str::AscString,
    sys,
};
use std::{
    cmp::Ordering,
    convert::Infallible,
    fmt::{self, Debug, Display, Formatter, LowerHex, UpperHex},
    str::FromStr,
};

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

    /// Creates a new big integer.
    pub fn new(x: i128) -> Self {
        Self::from_signed_bytes_le(x.to_le_bytes().as_slice())
    }

    /// Creates a new big integer value from it signed little-endian
    /// reprensentation. Unsigned integers need to ensure that the most
    /// significant bit is **not** set.
    pub fn from_signed_bytes_le(bytes: impl AsRef<[u8]>) -> Self {
        Self {
            inner: AscBigInt::from_bytes(bytes.as_ref()).owned(),
        }
    }

    /// Returns the sign of the integer.
    pub fn signum(&self) -> i32 {
        signum_le(self.inner.as_slice())
    }

    /// Parses a big integer from a string.
    pub fn parse(s: impl AsRef<str>) -> Self {
        let s = AscString::new(s.as_ref());
        let result = unsafe { &*sys::big_int__from_string(s.as_ptr()) };
        Self::from_raw(result)
    }

    /// Returns the sum of two big integers.
    pub fn plus(&self, rhs: &Self) -> Self {
        self.op(rhs, sys::big_int__plus)
    }

    /// Returns the difference of two big integers.
    pub fn minus(&self, rhs: &Self) -> Self {
        self.op(rhs, sys::big_int__minus)
    }

    /// Returns the product of two big integers.
    pub fn times(&self, rhs: &Self) -> Self {
        self.op(rhs, sys::big_int__times)
    }

    /// Returns the division of two big integers.
    pub fn divided_by(&self, rhs: &Self) -> Self {
        self.op(rhs, sys::big_int__divided_by)
    }

    /// Returns the division of a big integer by a big decimal.
    pub fn divided_by_decimal(&self, rhs: &BigDecimal) -> BigDecimal {
        BigDecimal::from_raw(unsafe {
            &*sys::big_int__divided_by_decimal(self.as_raw().as_ptr(), rhs.as_raw().as_ptr())
        })
    }

    /// Returns the remainder of two big integers.
    pub fn rem(&self, rhs: &Self) -> Self {
        self.op(rhs, sys::big_int__mod)
    }

    /// function pow(x: BigInt, exp: u8): BigInt
    pub fn pow(&self, exp: u8) -> Self {
        Self::from_raw(unsafe { &*sys::big_int__pow(self.as_raw().as_ptr(), exp) })
    }

    /// Returns the bit-wise or of two big integers.
    pub fn bit_or(&self, rhs: &Self) -> Self {
        self.op(rhs, sys::big_int__bit_or)
    }

    /// Returns the bit-wise and of two big integers.
    pub fn bit_and(&self, rhs: &Self) -> Self {
        self.op(rhs, sys::big_int__bit_and)
    }

    /// Returns the left shift by `rhs` bits.
    pub fn left_shift(&self, rhs: u8) -> Self {
        Self::from_raw(unsafe { &*sys::big_int__left_shift(self.as_raw().as_ptr(), rhs) })
    }

    /// Returns the arithmetic right shift by `rhs` bits.
    pub fn right_shift(&self, rhs: u8) -> Self {
        Self::from_raw(unsafe { &*sys::big_int__right_shift(self.as_raw().as_ptr(), rhs) })
    }

    fn op(
        &self,
        other: &BigInt,
        op: unsafe extern "C" fn(
            *const AscRef<AscBigInt>,
            *const AscRef<AscBigInt>,
        ) -> *const AscRef<AscBigInt>,
    ) -> Self {
        Self::from_raw(unsafe { &*op(self.as_raw().as_ptr(), other.as_raw().as_ptr()) })
    }
}

fn signum_le(bytes: &[u8]) -> i32 {
    if bytes.last().copied().unwrap_or_default() > 0x7f {
        -1
    } else if bytes.iter().copied().all(|b| b == 0) {
        0
    } else {
        1
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

impl Default for BigInt {
    fn default() -> Self {
        Self::new(0)
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

impl Eq for BigInt {}

impl FromStr for BigInt {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::parse(s))
    }
}

impl LowerHex for BigInt {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        fmt_hex(self, f, str::make_ascii_lowercase)
    }
}

impl Ord for BigInt {
    fn cmp(&self, other: &Self) -> Ordering {
        self.minus(other).signum().cmp(&0)
    }
}

impl PartialEq for BigInt {
    fn eq(&self, other: &Self) -> bool {
        matches!(self.cmp(other), Ordering::Equal)
    }
}

impl PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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

    /// Creates a new big decimal value.
    pub fn new(value: i128) -> Self {
        Self::from_big_int(BigInt::new(value))
    }

    /// Creates a new decimal value from the specified [`BigInt`].
    pub fn from_big_int(value: BigInt) -> Self {
        let value = AscBigDecimal::new(value.inner.into_owned(), AscBigInt::from_bytes(&[0]));
        Self {
            inner: value.owned(),
        }
    }

    /// Parses a big decimal from a string.
    pub fn parse(s: impl AsRef<str>) -> Self {
        let s = AscString::new(s.as_ref());
        let result = unsafe { &*sys::big_decimal__from_string(s.as_ptr()) };
        Self::from_raw(result)
    }

    /// Returns the addition of two big decimals.
    pub fn plus(&self, rhs: &Self) -> Self {
        self.op(rhs, sys::big_decimal__plus)
    }

    /// Returns the difference of two big decimals.
    pub fn minus(&self, rhs: &Self) -> Self {
        self.op(rhs, sys::big_decimal__minus)
    }

    /// Returns the product of two big decimals.
    pub fn times(&self, rhs: &Self) -> Self {
        self.op(rhs, sys::big_decimal__times)
    }

    /// Returns the division of two big decimals.
    pub fn divided_by(&self, rhs: &Self) -> Self {
        self.op(rhs, sys::big_decimal__divided_by)
    }

    fn op(
        &self,
        other: &BigDecimal,
        op: unsafe extern "C" fn(
            *const AscRef<AscBigDecimal>,
            *const AscRef<AscBigDecimal>,
        ) -> *const AscRef<AscBigDecimal>,
    ) -> Self {
        Self::from_raw(unsafe { &*op(self.as_raw().as_ptr(), other.as_raw().as_ptr()) })
    }
}

impl Debug for BigDecimal {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl Default for BigDecimal {
    fn default() -> Self {
        Self::new(0)
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

impl Eq for BigDecimal {}

impl FromStr for BigDecimal {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::parse(s))
    }
}

impl Ord for BigDecimal {
    fn cmp(&self, other: &Self) -> Ordering {
        let diff = self.minus(other);
        signum_le(diff.as_raw().digits().as_slice()).cmp(&0)
    }
}

impl PartialEq for BigDecimal {
    fn eq(&self, other: &Self) -> bool {
        unsafe { sys::big_decimal__equals(self.as_raw().as_ptr(), other.as_raw().as_ptr()) }
    }
}

impl PartialOrd for BigDecimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
