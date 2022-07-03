use super::super::unsigned_number::UnsignedNumber;
use super::super::OperationError;
use super::super::Sign;
use super::Fraction;

impl<N: UnsignedNumber> Fraction<N> {
  /// Returns an absolute value of the given number
  ///
  /// ### Examples
  ///
  /// ```rust
  /// use rfraction::{Fraction, Sign};
  /// let sign = Sign::Negative;
  /// let numerator = 42;
  /// let denominator = 43;
  /// let number = Fraction::<u128>::new(sign, numerator, denominator);
  /// let another_number = Fraction::<u128>::new(sign.inverse(), numerator, denominator);
  /// assert_eq!(number.abs(), another_number.abs());
  /// ```
  pub fn abs(&self) -> Fraction<N> {
    self.try_abs().unwrap()
  }

  /// Returns an absolute value of the given number or could fail with an [`OperationError`]
  ///
  /// ### Examples
  ///
  /// ```rust
  /// use rfraction::{Fraction, Sign};
  /// let sign = Sign::Negative;
  /// let numerator = 42;
  /// let denominator = 43;
  /// let number = Fraction::<u128>::new(sign, numerator, denominator);
  /// let another_number = Fraction::<u128>::new(sign.inverse(), numerator, denominator);
  /// assert_eq!(number.abs(), another_number.abs());
  /// ```
  ///
  /// ### Errors
  ///
  /// If there are some issues with the number,
  /// it will return a [`OperationError`]
  pub fn try_abs(&self) -> Result<Fraction<N>, OperationError> {
    Fraction::try_new(Sign::Positive, self.numerator(), self.denominator())
  }
}
