use super::super::unsigned_number::UnsignedNumber;
use super::super::OperationError;
use super::Fraction;

impl<N: UnsignedNumber> Fraction<N> {
  /// Returns the number with the sign inverted
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
  /// assert_eq!(number.neg(), another_number);
  /// ```
  ///
  /// ### Panics
  ///
  /// If there are some issues with the number it will panic
  pub fn neg(&self) -> Fraction<N> {
    self.try_neg().unwrap()
  }

  /// Returns the number with the sign inverted or could fail with an [`OperationError`]
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
  /// assert_eq!(number.neg(), another_number);
  /// ```
  ///
  /// ### Errors
  ///
  /// If there are some issues with the number, it will return a [`OperationError`]
  pub fn try_neg(&self) -> Result<Fraction<N>, OperationError> {
    Fraction::try_new(self.sign.inverse(), self.numerator(), self.denominator())
  }
}
