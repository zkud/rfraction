use super::super::unsigned_number::UnsignedNumber;
use super::super::OperationError;
use super::Fraction;
use std::ops;

impl<N: UnsignedNumber> Fraction<N> {
  pub fn mul(&self, other: &Fraction<N>) -> Fraction<N> {
    self.try_mul(other).unwrap()
  }

  pub fn try_mul(&self, other: &Fraction<N>) -> Result<Fraction<N>, OperationError> {
    let sign = self.sign.mul(&other.sign);
    let numerator = self.numerator().try_mul(other.numerator())?;
    let denominator = self.denominator().try_mul(other.denominator())?;
    Fraction::try_new(sign, numerator, denominator)
  }
}

impl<N: UnsignedNumber> ops::Mul for &Fraction<N> {
  type Output = Fraction<N>;

  fn mul(self, other: Self) -> Fraction<N> {
    self.mul(other)
  }
}

impl<N: UnsignedNumber> ops::Mul for Fraction<N> {
  type Output = Self;

  fn mul(self, other: Self) -> Self {
    (&self).mul(&other)
  }
}

impl<N: UnsignedNumber> ops::MulAssign<&Self> for Fraction<N> {
  fn mul_assign(&mut self, other: &Self) {
    *self = self.mul(other)
  }
}

impl<N: UnsignedNumber> ops::MulAssign<Self> for Fraction<N> {
  fn mul_assign(&mut self, other: Self) {
    *self = self.mul(&other)
  }
}
