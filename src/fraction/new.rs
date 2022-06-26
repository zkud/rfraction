use super::super::unsigned_number::UnsignedNumber;
use super::super::OperationError;
use super::super::OperationErrorType;
use super::super::Sign;
use super::Fraction;

impl<N: UnsignedNumber> Fraction<N> {
  pub fn new(sign: Sign, numerator: N, denominator: N) -> Fraction<N> {
    Fraction::try_new(sign, numerator, denominator).unwrap()
  }

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

  pub fn new_zero() -> Fraction<N> {
    Fraction {
      numerator: N::ZERO,
      denominator: N::ONE,
      sign: Sign::Positive,
    }
  }

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
