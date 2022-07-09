use super::super::unsigned_number::UnsignedNumber;
use super::super::OperationError;
use super::super::OperationErrorType;
use super::Fraction;

impl<N: UnsignedNumber> Fraction<N> {
  /// Swaps numerator and denominator
  ///
  /// ### Examples
  /// ```rust
  /// use rfraction::{Fraction, Sign};
  ///
  /// let sign = Sign::Positive;
  /// let numerator = 1;
  /// let denominator = 14;
  /// let a = Fraction::<u128>::new(sign, numerator, denominator);
  ///
  /// let result = a.reverse();
  ///
  /// assert!(result.is_positive());
  /// assert_eq!(result.numerator(), denominator);
  /// assert_eq!(result.denominator(), numerator);
  /// ```
  ///
  /// ### Panics
  ///
  /// If some errors occur during swapping, for example the numerator is zero,
  /// it will panic
  pub fn reverse(&self) -> Fraction<N> {
    self.try_reverse().unwrap()
  }

  /// Swaps numerator and denominator or could fail with an [`OperationError`]
  ///
  /// ### Examples
  /// ```rust
  /// use rfraction::{Fraction, Sign};
  ///
  /// let sign = Sign::Positive;
  /// let numerator = 0;
  /// let denominator = 14;
  /// let a = Fraction::<u128>::new(sign, numerator, denominator);
  ///
  /// let result = a.try_reverse();
  ///
  /// assert!(matches!(result, OperationError));
  /// ```
  ///
  /// ### Errors
  ///
  /// If some errors occur during swapping, for example the numerator is zero,
  /// it will return a [`OperationError`]
  pub fn try_reverse(&self) -> Result<Fraction<N>, OperationError> {
    if self.is_zero() {
      Err(OperationError::new(
        "Numerator is zero, can't divide by zero",
        OperationErrorType::DivisionByZero,
      ))
    } else {
      Fraction::try_new(self.sign, self.denominator.clone(), self.numerator.clone())
    }
  }
}
