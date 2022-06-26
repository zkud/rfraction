use super::super::sign::Sign;
use super::super::unsigned_number::UnsignedNumber;
use super::super::OperationError;
use super::Fraction;
use std::ops;

impl<N: UnsignedNumber> Fraction<N> {
  pub fn add(&self, other: &Self) -> Self {
    self.try_add(other).unwrap()
  }

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
