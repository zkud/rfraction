use super::super::unsigned_number::UnsignedNumber;
use super::super::OperationError;
use super::Fraction;
use std::ops;

impl<N: UnsignedNumber> Fraction<N> {
  pub fn sub(&self, other: &Fraction<N>) -> Fraction<N> {
    self.try_sub(other).unwrap()
  }

  pub fn try_sub(&self, other: &Fraction<N>) -> Result<Fraction<N>, OperationError> {
    self.try_add(&other.try_neg()?)
  }
}

impl<N: UnsignedNumber> ops::Sub for &Fraction<N> {
  type Output = Fraction<N>;

  fn sub(self, other: Self) -> Fraction<N> {
    self.sub(other)
  }
}

impl<N: UnsignedNumber> ops::Sub for Fraction<N> {
  type Output = Self;

  fn sub(self, other: Self) -> Self {
    (&self).sub(&other)
  }
}

impl<N: UnsignedNumber> ops::SubAssign<&Self> for Fraction<N> {
  fn sub_assign(&mut self, other: &Self) {
    *self = self.sub(other)
  }
}

impl<N: UnsignedNumber> ops::SubAssign<Self> for Fraction<N> {
  fn sub_assign(&mut self, other: Self) {
    *self = self.sub(&other)
  }
}
