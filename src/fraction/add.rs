use super::super::unsigned_number::UnsignedNumber;
use super::super::OperationError;
use super::Fraction;
use std::ops;

impl<N: UnsignedNumber> Fraction<N> {
  pub fn add(&self, other: &Self) -> Self {
    self.try_add(other).unwrap()
  }

  pub fn try_add(&self, other: &Self) -> Result<Self, OperationError> {
    let (unified_self, unified_other) = self.unify(other)?;

    if unified_self.sign == unified_other.sign {
      return Fraction::try_new(
        unified_self.sign,
        unified_self
          .numerator()
          .try_add(unified_other.numerator())?,
        unified_self.denominator,
      );
    }

    if unified_self.numerator() == unified_other.numerator() {
      return Ok(Fraction::new_zero());
    }

    if unified_self.numerator() > unified_other.numerator() {
      Fraction::try_new(
        unified_self.sign,
        unified_self.numerator - unified_other.numerator,
        unified_self.denominator,
      )
    } else {
      Fraction::try_new(
        unified_other.sign,
        unified_other.numerator - unified_self.numerator,
        unified_self.denominator,
      )
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
