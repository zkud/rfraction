use super::super::sign::Sign;
use super::super::unsigned_number::UnsignedNumber;
use super::super::OperationError;
use super::Fraction;
use std::ops;

impl<N: UnsignedNumber> Fraction<N> {
  /// Adds two numbers and returns a sum
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
  /// let result = a.add(&b);
  ///
  /// assert!(result.is_positive());
  /// assert_eq!(result.numerator(), 1);
  /// assert_eq!(result.denominator(), 8);
  /// ```
  ///
  /// ### Panics
  ///
  /// If some errors occur during addition, for example overflow is encountered,
  /// it will panic
  pub fn add(&self, other: &Self) -> Self {
    self.try_add(other).unwrap()
  }

  /// Adds two numbers and returns a sum or could fail with an [`OperationError`]
  ///
  /// ### Examples
  /// ```rust
  /// use rfraction::{Fraction, Sign, OperationError};
  ///
  /// let sign = Sign::Positive;
  /// let numerator = 10;
  /// let denominator = 140;
  /// let a = Fraction::<u8>::new(sign, numerator, denominator);
  /// let numerator = 3;
  /// let denominator = 56;
  /// let b = Fraction::<u8>::new(sign, numerator, denominator);
  ///
  /// let result = a.add(&b);
  ///
  /// assert!(matches!(result, OperationError));
  /// ```
  ///
  /// ### Errors
  ///
  /// If some errors occur during addition, for example overflow is encountered,
  /// it will return a [`OperationError`]
  pub fn try_add(&self, other: &Self) -> Result<Self, OperationError> {
    let (this, other) = self.unify(other)?;
    let sign = this.get_result_sign(&other);
    let numerator = this.get_result_numerator(&other)?;
    Fraction::try_new(sign, numerator, this.denominator)
  }

  #[inline]
  fn get_result_sign(&self, other: &Fraction<N>) -> Sign {
    if self.numerator() > other.numerator() {
      self.sign()
    } else {
      other.sign()
    }
  }

  #[inline]
  fn get_result_numerator(&self, other: &Fraction<N>) -> Result<N, OperationError> {
    if self.sign() == other.sign() {
      self.numerator().try_add(other.numerator())
    } else {
      Ok(Fraction::get_abs_numerator_difference(&self, &other))
    }
  }

  #[inline]
  fn get_abs_numerator_difference(&self, other: &Fraction<N>) -> N {
    if self.numerator() > other.numerator() {
      self.numerator() - other.numerator()
    } else {
      other.numerator() - self.numerator()
    }
  }
}

impl<N: UnsignedNumber> ops::Add for Fraction<N> {
  type Output = Fraction<N>;

  fn add(self, other: Self) -> Fraction<N> {
    self.try_add(&other).unwrap()
  }
}

impl<N: UnsignedNumber> ops::Add for &Fraction<N> {
  type Output = Fraction<N>;

  fn add(self, other: Self) -> Fraction<N> {
    self.try_add(other).unwrap()
  }
}

impl<N: UnsignedNumber> ops::AddAssign<&Self> for Fraction<N> {
  fn add_assign(&mut self, other: &Self) {
    *self = self.add(other)
  }
}

impl<N: UnsignedNumber> ops::AddAssign<Self> for Fraction<N> {
  fn add_assign(&mut self, other: Self) {
    *self = self.add(&other)
  }
}
