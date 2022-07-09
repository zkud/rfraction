use super::super::unsigned_number::UnsignedNumber;
use super::super::OperationError;
use super::Fraction;
use std::ops;

impl<N: UnsignedNumber> Fraction<N> {
  /// Divides a number by another number
  ///
  /// ### Examples
  /// ```rust
  /// use rfraction::{Fraction, Sign};
  ///
  /// let sign = Sign::Positive;
  /// let numerator = 10;
  /// let denominator = 140;
  /// let a = Fraction::<u128>::new(sign, numerator, denominator);
  /// let numerator = 15;
  /// let denominator = 280;
  /// let b = Fraction::<u128>::new(sign, numerator, denominator);
  ///
  /// let result = a.div(&b);
  ///
  /// assert!(result.is_positive());
  /// assert_eq!(result.numerator(), 4);
  /// assert_eq!(result.denominator(), 3);
  /// ```
  ///
  /// ### Panics
  ///
  /// If some errors occur during division, for example the denominator is zero,
  /// it will panic
  pub fn div(&self, other: &Fraction<N>) -> Fraction<N> {
    self.try_div(other).unwrap()
  }

  /// Divides a number by another number  or could fail with an [`OperationError`]
  ///
  /// ### Examples
  /// ```rust
  /// use rfraction::{Fraction, Sign, OperationError};
  ///
  /// let sign = Sign::Positive;
  /// let numerator = 10;
  /// let denominator = 140;
  /// let a = Fraction::<u8>::new(sign, numerator, denominator);
  /// let numerator = 0;
  /// let denominator = 14;
  /// let b = Fraction::<u8>::new(sign, numerator, denominator);
  ///
  /// let result = a.add(&b);
  ///
  /// assert!(matches!(result, OperationError));
  /// ```
  ///
  /// ### Errors
  ///
  /// If some errors occur during division, for example the denominator is zero,
  /// it will return a [`OperationError`]
  pub fn try_div(&self, other: &Fraction<N>) -> Result<Fraction<N>, OperationError> {
    self.try_mul(&other.try_reverse()?)
  }
}

impl<N: UnsignedNumber> ops::Div for &Fraction<N> {
  type Output = Fraction<N>;

  fn div(self, other: Self) -> Fraction<N> {
    self.div(other)
  }
}

impl<N: UnsignedNumber> ops::Div for Fraction<N> {
  type Output = Self;

  fn div(self, other: Self) -> Self {
    (&self).div(&other)
  }
}

impl<N: UnsignedNumber> ops::DivAssign<&Self> for Fraction<N> {
  fn div_assign(&mut self, other: &Self) {
    *self = self.div(other)
  }
}

impl<N: UnsignedNumber> ops::DivAssign<Self> for Fraction<N> {
  fn div_assign(&mut self, other: Self) {
    *self = self.div(&other)
  }
}
