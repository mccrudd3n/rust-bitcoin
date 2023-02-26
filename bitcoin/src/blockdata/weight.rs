//! Implements `Weight` and associated features.

use core::fmt;
use core::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

use crate::prelude::*;

/// Represents block weight - the weight of a transaction or block.
///
/// This is an integer newtype representing weigth in `wu`. It provides protection against mixing
/// up the types as well as basic formatting features.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(crate = "actual_serde"))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct Weight(u64);

impl Weight {
    /// 0 wu.
    ///
    /// Equivalent to [`MIN`](Self::MIN), may better express intent in some contexts.
    pub const ZERO: Weight = Weight(0);

    /// Minimum possible value (0 wu).
    ///
    /// Equivalent to [`ZERO`](Self::ZERO), may better express intent in some contexts.
    pub const MIN: Weight = Weight(u64::min_value());

    /// Maximum possible value.
    pub const MAX: Weight = Weight(u64::max_value());

    /// Directly constructs `Weight` from weight units.
    pub const fn from_wu(wu: u64) -> Self {
        Weight(wu)
    }

    /// Constructs `Weight` from virtual bytes.
    ///
    /// # Errors
    ///
    /// Returns `None` on overflow.
    pub fn from_vb(vb: u64) -> Option<Self> {
        vb.checked_mul(4).map(Weight::from_wu)
    }

    /// Constructs `Weight` from virtual bytes without overflow check.
    pub const fn from_vb_unchecked(vb: u64) -> Self {
        Weight::from_wu(vb * 4)
    }

    /// Constructs `Weight` from witness size.
    pub const fn from_witness_data_size(witness_size: u64) -> Self {
        Weight(witness_size)
    }

    /// Constructs `Weight` from non-witness size.
    pub const fn from_non_witness_data_size(non_witness_size: u64) -> Self {
        Weight(non_witness_size * 4)
    }

    /// Returns raw weight units.
    ///
    /// Can be used instead of `into()` to avoid inference issues.
    pub const fn to_wu(self) -> u64 {
        self.0
    }

    /// Converts to vB rounding down.
    pub const fn to_vbytes_floor(self) -> u64 {
        self.0 / 4
    }

    /// Converts to vB rounding up.
    pub const fn to_vbytes_ceil(self) -> u64 {
        (self.0 + 3) / 4
    }

    /// Checked addition.
    ///
    /// Computes `self + rhs` returning `None` if overflow occurred.
    pub fn checked_add(self, rhs: Self) -> Option<Self> {
        self.0.checked_add(rhs.0).map(Self)
    }

    /// Checked subtraction.
    ///
    /// Computes `self - rhs` returning `None` if overflow occurred.
    pub fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_add(rhs.0).map(Self)
    }

    /// Checked multiplication.
    ///
    /// Computes `self * rhs` returning `None` if overflow occurred.
    pub fn checked_mul(self, rhs: u64) -> Option<Self> {
        self.0.checked_mul(rhs).map(Self)
    }

    /// Checked division.
    ///
    /// Computes `self / rhs` returning `None` if `rhs == 0`.
    pub fn checked_div(self, rhs: u64) -> Option<Self> {
        self.0.checked_div(rhs).map(Self)
    }
}

/// Alternative will display the unit.
impl fmt::Display for Weight {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            write!(f, "{} wu", self.0)
        } else {
            fmt::Display::fmt(&self.0, f)
        }
    }
}

impl From<Weight> for u64 {
    fn from(value: Weight) -> Self {
        value.to_wu()
    }
}

impl Add for Weight {
    type Output = Weight;

    fn add(self, rhs: Weight) -> Self::Output {
        Weight(self.0 + rhs.0)
    }
}

impl AddAssign for Weight {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl Sub for Weight {
    type Output = Weight;

    fn sub(self, rhs: Weight) -> Self::Output {
        Weight(self.0 - rhs.0)
    }
}

impl SubAssign for Weight {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0
    }
}

impl Mul<u64> for Weight {
    type Output = Weight;

    fn mul(self, rhs: u64) -> Self::Output {
        Weight(self.0 * rhs)
    }
}

impl Mul<Weight> for u64 {
    type Output = Weight;

    fn mul(self, rhs: Weight) -> Self::Output {
        Weight(self * rhs.0)
    }
}

impl MulAssign<u64> for Weight {
    fn mul_assign(&mut self, rhs: u64) {
        self.0 *= rhs
    }
}

impl Div<u64> for Weight {
    type Output = Weight;

    fn div(self, rhs: u64) -> Self::Output {
        Weight(self.0 / rhs)
    }
}

impl DivAssign<u64> for Weight {
    fn div_assign(&mut self, rhs: u64) {
        self.0 /= rhs
    }
}

impl core::iter::Sum for Weight {
    fn sum<I>(iter: I) -> Self where I: Iterator<Item = Self> {
        Weight(iter.map(Weight::to_wu).sum())
    }
}

impl<'a> core::iter::Sum<&'a Weight> for Weight {
    fn sum<I>(iter: I) -> Self where I: Iterator<Item = &'a Weight> {
        iter.cloned().sum()
    }
}

crate::parse::impl_parse_str_from_int_infallible!(Weight, u64, from_wu);
