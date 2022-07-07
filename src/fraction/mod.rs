mod abs;
mod add;
mod cmp;
mod convertions;
mod div;
mod getters;
mod mul;
mod neg;
mod new;
mod predicates;
mod reverse;
mod sub;

use super::operation_error::OperationError;
use super::sign::Sign;
use super::unsigned_number::UnsignedNumber;

#[derive(Clone, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Fraction<N: UnsignedNumber> {
  numerator: N,
  denominator: N,
  sign: Sign,
}

impl<N: UnsignedNumber> Fraction<N> {
  #[inline]
  fn simplify(mut self) -> Fraction<N> {
    if !self.is_zero() {
      let gcd = self.find_gcd(self.numerator.clone(), self.denominator.clone());

      self.numerator = self.numerator / gcd.clone();
      self.denominator = self.denominator / gcd;
    }

    self
  }

  #[inline]
  fn find_gcd(&self, mut a: N, mut b: N) -> N {
    while b != N::ZERO {
      let c = b.clone();
      b = a % b;
      a = c;
    }

    a
  }

  #[inline]
  fn unify(&self, other: &Fraction<N>) -> Result<(Fraction<N>, Fraction<N>), OperationError> {
    let simplified_self = self.clone().simplify();
    let simplified_other = other.clone().simplify();

    match simplified_self.denominator.clone() {
      x if x == simplified_other.denominator => Ok((simplified_self, simplified_other)),
      x if simplified_other.denominator.clone() % x.clone() == N::ZERO => {
        let scale = simplified_other.denominator.clone() / x;
        Ok((simplified_self.mul_with_number(scale)?, simplified_other))
      }
      x if x.clone() % simplified_other.denominator.clone() == N::ONE => {
        let scale = x / simplified_other.denominator.clone();
        Ok((simplified_self, simplified_other.mul_with_number(scale)?))
      }
      _ => Ok((
        simplified_self.mul_with_number(simplified_other.denominator.clone())?,
        simplified_other.mul_with_number(simplified_self.denominator)?,
      )),
    }
  }

  #[inline]
  fn mul_with_number(&self, number: N) -> Result<Fraction<N>, OperationError> {
    let numerator = self.numerator.clone().try_mul(number.clone())?;
    let denominator = self.denominator.clone().try_mul(number)?;

    Ok(Fraction {
      numerator,
      denominator,
      sign: self.sign,
    })
  }
}
