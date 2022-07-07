use super::super::unsigned_number::UnsignedNumber;
use super::super::OperationError;
use super::Fraction;
use std::ops;

impl<N: UnsignedNumber> Fraction<N> {
  /// Multiplies a number by another number
  ///
  /// ### Examples
  /// ```rust
  /// use rfraction::{Fraction, Sign};
  ///
  /// let sign = Sign::Positive;
  /// let numerator = 17;
  /// let denominator = 2;
  /// let a = Fraction::<u128>::new(sign, numerator, denominator);
  /// let numerator = 3;
  /// let denominator = 4;
  /// let b = Fraction::<u128>::new(sign, numerator, denominator);
  ///
  /// let result = a.mul(&b);
  ///
  /// assert!(result.is_positive());
  /// assert_eq!(result.numerator(), 51);
  /// assert_eq!(result.denominator(), 8);
  /// ```
  ///
  /// ### Panics
  ///
  /// If some errors occur during multiplication, for example an overflow,
  /// it will panic
  pub fn mul(&self, other: &Fraction<N>) -> Fraction<N> {
    self.try_mul(other).unwrap()
  }

  /// Multiplies a number by another number
  ///
  /// ### Examples
  /// ```rust
  /// use rfraction::{Fraction, Sign};
  ///
  /// let sign = Sign::Positive;
  /// let numerator = 1;
  /// let denominator = 14;
  /// let a = Fraction::<u8>::new(sign, numerator, denominator);
  /// let numerator = 15;
  /// let denominator = 28;
  /// let b = Fraction::<u8>::new(sign, numerator, denominator);
  ///
  /// let result = a.try_mul(&b);
  ///
  /// assert!(matches!(result, OperationError));
  /// ```
  ///
  /// ### Errors
  ///
  /// If some errors occur during multiplication, for example an overflow,
  /// it will return a [`OperationError`]
  pub fn try_mul(&self, other: &Fraction<N>) -> Result<Fraction<N>, OperationError> {
    let sign = self.sign.mul(&other.sign);
    let numerator = self.numerator().try_mul(other.numerator())?;
    let denominator = self.denominator().try_mul(other.denominator())?;
    Fraction::try_new(sign, numerator, denominator)
  }
}

impl<N: UnsignedNumber> ops::Mul for &Fraction<N> {
  type Output = Fraction<N>;

  fn mul(self, other: Self) -> Fraction<N> {
    self.mul(other)
  }
}

impl<N: UnsignedNumber> ops::Mul for Fraction<N> {
  type Output = Self;

  fn mul(self, other: Self) -> Self {
    (&self).mul(&other)
  }
}

impl<N: UnsignedNumber> ops::MulAssign<&Self> for Fraction<N> {
  fn mul_assign(&mut self, other: &Self) {
    *self = self.mul(other)
  }
}

impl<N: UnsignedNumber> ops::MulAssign<Self> for Fraction<N> {
  fn mul_assign(&mut self, other: Self) {
    *self = self.mul(&other)
  }
}
