use super::super::unsigned_number::UnsignedNumber;
use super::super::OperationError;
use super::super::Sign;
use super::Fraction;

impl<N: UnsignedNumber> Fraction<N> {
  pub fn abs(&self) -> Fraction<N> {
    self.try_abs().unwrap()
  }

  pub fn try_abs(&self) -> Result<Fraction<N>, OperationError> {
    Fraction::try_new(Sign::Positive, self.numerator(), self.denominator())
  }
}
