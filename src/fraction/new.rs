use super::super::unsigned_number::UnsignedNumber;
use super::super::OperationError;
use super::super::OperationErrorType;
use super::super::Sign;
use super::Fraction;

impl<N: UnsignedNumber> Fraction<N> {
  /// Creates a new [`Fraction<N: UnsignedNumber>`]
  ///
  /// ### Examples
  ///
  /// ```rust
  /// use rfraction::{Fraction, Sign};
  /// let sign = Sign::Positive;
  /// let numerator = 42;
  /// let denominator = 15;
  /// Fraction::<u128>::new(sign, numerator, denominator);
  /// ```
  ///
  /// ### Panics
  ///
  /// If one of inputs is incorrect, for example a zero denominator, it will panic
  pub fn new(sign: Sign, numerator: N, denominator: N) -> Fraction<N> {
    Fraction::try_new(sign, numerator, denominator).unwrap()
  }

  /// Creates a new [`Fraction<N: UnsignedNumber>`] or could fail with an [`OperationError`]
  ///
  /// ### Examples
  ///
  /// ```rust
  /// use rfraction::{Fraction, Sign};
  /// let sign = Sign::Positive;
  /// let numerator = 42;
  /// let denominator = 0;
  /// match Fraction::<u128>::try_new(sign, numerator, denominator) {
  ///   Ok(number) => println!("{}", number),
  ///   Err(error) => println!("{}", error)
  /// };
  /// ```
  ///
  /// ### Errors
  ///
  /// If one of inputs is incorrect, for example a zero denominator,
  /// it will return a [`OperationError`]
  pub fn try_new(sign: Sign, numerator: N, denominator: N) -> Result<Fraction<N>, OperationError> {
    if denominator == N::ZERO {
      return Err(OperationError::new(
        "Denominator can not be zero",
        OperationErrorType::DivisionByZero,
      ));
    }

    if numerator == N::ZERO {
      return Ok(Fraction::new_zero());
    }

    Ok(
      Fraction {
        numerator,
        denominator,
        sign,
      }
      .simplify(),
    )
  }

  /// Creates a new [`Fraction<N: UnsignedNumber>`], which equals to zero
  ///
  /// ### Examples
  ///
  /// ```rust
  /// use rfraction::Fraction;
  /// Fraction::<u128>::new_zero();
  /// ```
  pub fn new_zero() -> Fraction<N> {
    Fraction {
      numerator: N::ZERO,
      denominator: N::ONE,
      sign: Sign::Positive,
    }
  }

  /// Creates a new [`Fraction<N: UnsignedNumber>`], which is a natural number
  ///
  /// ### Examples
  ///
  /// ```rust
  /// use rfraction::Fraction;
  /// let value = 42;
  /// Fraction::<u128>::new_natural(42);
  /// ```
  pub fn new_natural(value: N) -> Fraction<N> {
    Fraction {
      numerator: value,
      denominator: N::ONE,
      sign: Sign::Positive,
    }
  }
}

impl<N: UnsignedNumber> Default for Fraction<N> {
  fn default() -> Fraction<N> {
    Fraction::new_zero()
  }
}
