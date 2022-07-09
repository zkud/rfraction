use super::super::unsigned_number::UnsignedNumber;
use super::super::Sign;
use super::Fraction;

impl<N: UnsignedNumber> Fraction<N> {
  /// Returns the numerator of a number
  ///
  /// ### Examples
  ///
  /// ```rust
  /// use rfraction::{Fraction, Sign};
  /// let sign = Sign::Positive;
  /// let numerator = 14;
  /// let denominator = 5;
  /// let number = Fraction::<u128>::new(sign, numerator, denominator);
  /// assert_eq!(number.numerator(), numerator);
  /// ```
  pub fn numerator(&self) -> N {
    self.numerator.clone()
  }

  /// Returns the denominator of a number
  ///
  /// ### Examples
  ///
  /// ```rust
  /// use rfraction::{Fraction, Sign};
  /// let sign = Sign::Positive;
  /// let numerator = 14;
  /// let denominator = 5;
  /// let number = Fraction::<u128>::new(sign, numerator, denominator);
  /// assert_eq!(number.denominator(), denominator);
  /// ```
  pub fn denominator(&self) -> N {
    self.denominator.clone()
  }

  /// Returns the sign of a number
  ///
  /// ### Examples
  ///
  /// ```rust
  /// use rfraction::{Fraction, Sign};
  /// let sign = Sign::Positive;
  /// let numerator = 14;
  /// let denominator = 5;
  /// let number = Fraction::<u128>::new(sign, numerator, denominator);
  /// assert_eq!(number.denominator(), denominator);
  /// ```
  pub fn sign(&self) -> Sign {
    self.sign
  }

  /// Returns the remainder of a number
  ///
  /// ### Examples
  ///
  /// ```rust
  /// use rfraction::{Fraction, Sign};
  /// let sign = Sign::Positive;
  /// let numerator = 14;
  /// let denominator = 5;
  /// let number = Fraction::<u128>::new(sign, numerator, denominator);
  /// let remainder = number.remainder();
  /// assert_eq!(remainder.sign(), sign);
  /// assert_eq!(remainder.numerator(), 4);
  /// assert_eq!(remainder.denominator(), denominator);
  /// ```
  pub fn remainder(&self) -> Fraction<N> {
    Fraction::new(
      self.sign(),
      self.numerator() % self.denominator(),
      self.denominator(),
    )
  }

  /// Returns a number with the remainder dropped
  ///
  /// ### Examples
  ///
  /// ```rust
  /// use rfraction::{Fraction, Sign};
  /// let sign = Sign::Positive;
  /// let numerator = 14;
  /// let denominator = 5;
  /// let number = Fraction::<u128>::new(sign, numerator, denominator);
  /// assert_eq!(number.trunc(), 2);
  /// ```
  pub fn trunc(&self) -> N {
    self.numerator() / self.denominator()
  }
}
