use super::super::unsigned_number::UnsignedNumber;
use super::super::Sign;
use super::Fraction;

impl<N: UnsignedNumber> Fraction<N> {
  pub fn is_negative(&self) -> bool {
    if self.is_zero() {
      false
    } else {
      matches!(self.sign, Sign::Negative)
    }
  }

  pub fn is_positive(&self) -> bool {
    if self.is_zero() {
      false
    } else {
      matches!(self.sign, Sign::Positive)
    }
  }

  pub fn is_natural(&self) -> bool {
    self.is_positive() && self.numerator() >= N::ONE && self.denominator() == N::ONE
  }

  pub fn is_zero(&self) -> bool {
    self.numerator() == N::ZERO && self.denominator() != N::ZERO
  }
}
