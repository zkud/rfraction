use super::super::unsigned_number::UnsignedNumber;
use super::super::OperationError;
use super::super::OperationErrorType;
use super::Fraction;
use std::ops;

impl<N: UnsignedNumber> Fraction<N> {
  pub fn div(&self, other: &Fraction<N>) -> Fraction<N> {
    self.try_div(other).unwrap()
  }

  pub fn try_div(&self, other: &Fraction<N>) -> Result<Fraction<N>, OperationError> {
    self.try_mul(&other.try_reverse()?)
  }

  pub fn reverse(&self) -> Fraction<N> {
    self.try_reverse().unwrap()
  }

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
