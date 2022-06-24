use super::super::unsigned_number::UnsignedNumber;
use super::super::OperationError;
use super::Fraction;

impl<N: UnsignedNumber> Fraction<N> {
  pub fn neg(&self) -> Fraction<N> {
    self.try_neg().unwrap()
  }

  pub fn try_neg(&self) -> Result<Fraction<N>, OperationError> {
    Fraction::try_new(self.sign.inverse(), self.numerator(), self.denominator())
  }
}
