use super::super::unsigned_number::UnsignedNumber;
use super::super::Sign;
use super::Fraction;

impl<N: UnsignedNumber> Fraction<N> {
  pub fn numerator(&self) -> N {
    self.numerator.clone()
  }

  pub fn denominator(&self) -> N {
    self.denominator.clone()
  }

  pub fn sign(&self) -> Sign {
    self.sign
  }

  pub fn remainder(&self) -> Fraction<N> {
    Fraction {
      numerator: self.numerator() % self.denominator(),
      denominator: self.denominator(),
      sign: self.sign(),
    }
  }

  pub fn trunc(&self) -> N {
    self.numerator() / self.denominator()
  }
}
