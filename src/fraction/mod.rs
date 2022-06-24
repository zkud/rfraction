mod abs;
mod add;
mod div;
mod mul;
mod neg;
mod new;
mod predicates;
mod sub;

use super::convertable_to::ConvertableTo;
use super::float_number::FloatNumber;
use super::operation_error::OperationError;
use super::operation_error::OperationErrorType;
use super::sign::Sign;
use super::unsigned_number::UnsignedNumber;
use std::cmp;
use std::convert;
use std::fmt;
use std::hash;
use std::ops;

#[derive(Clone, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Fraction<N: UnsignedNumber> {
  numerator: N,
  denominator: N,
  sign: Sign,
}

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

  pub fn to_decimal(&self, precision: usize) -> Fraction<N> {
    self.try_to_decimal(precision).unwrap()
  }

  pub fn try_to_decimal(&self, precision: usize) -> Result<Fraction<N>, OperationError> {
    if self.is_zero() {
      Ok(Fraction::new_zero())
    } else {
      let mut new_numerator = N::ZERO;
      let mut new_denominator = N::ONE;

      let mut remainder = self.abs().remainder();
      for _ in 0..precision {
        if !remainder.is_zero() {
          let bigger_numerator = remainder.numerator().try_mul(N::TEN)?;
          let digit = bigger_numerator.clone() / remainder.denominator();

          new_numerator = new_numerator.try_mul(N::TEN)?.try_add(digit.clone())?;
          new_denominator = new_denominator.try_mul(N::TEN)?;

          remainder = Fraction::new(
            Sign::Positive,
            bigger_numerator.clone() - remainder.denominator() * digit,
            remainder.denominator(),
          );
        } else {
          new_numerator = new_numerator.try_mul(N::TEN)?;
          new_denominator = new_denominator.try_mul(N::TEN)?;
        }
      }

      Fraction::try_new(self.sign(), new_numerator, new_denominator)?.try_add(&Fraction::try_new(
        self.sign(),
        self.trunc(),
        N::ONE,
      )?)
    }
  }

  pub fn to_number<F>(&self) -> F
  where
    F: ops::Div<F, Output = F> + ops::Neg<Output = F>,
    N: ConvertableTo<F>,
  {
    let numerator: F = self.numerator().convert_to();
    let denominator: F = self.denominator().convert_to();

    if self.is_positive() {
      numerator / denominator
    } else {
      -numerator / denominator
    }
  }

  pub fn from_float_number<F: FloatNumber + Into<u8>>(number: F) -> Fraction<N> {
    Fraction::try_from_float_number(number).unwrap()
  }

  pub fn try_from_float_number<F: FloatNumber>(number: F) -> Result<Fraction<N>, OperationError> {
    if number.is_nan() || number.is_infinite() {
      return Err(OperationError::new(
        "Invalid input number",
        OperationErrorType::ConvertionError,
      ));
    }

    let integer_part = Fraction::try_get_integer_part(number.clone())?;
    let float_part = Fraction::try_get_float_part(number.clone())?;

    integer_part.try_add(&float_part)
  }

  #[inline]
  fn try_get_integer_part<F: FloatNumber>(number: F) -> Result<Fraction<N>, OperationError> {
    let sign = if number > F::EPSILON {
      Sign::Positive
    } else {
      Sign::Negative
    };

    let mut integer_part = number.abs().trunc();
    let mut numerator: N = N::ZERO;

    while integer_part > F::EPSILON {
      numerator = numerator.try_mul(N::TEN)?;
      numerator = numerator.try_add(N::from(integer_part.rem_euclid(F::TEN).to_u8()))?;
      integer_part = (integer_part / F::TEN).trunc();
    }

    Fraction::try_new(sign, numerator, N::ONE)
  }

  #[inline]
  fn try_get_float_part<F: FloatNumber>(number: F) -> Result<Fraction<N>, OperationError> {
    let sign = if number > F::EPSILON {
      Sign::Positive
    } else {
      Sign::Negative
    };

    let mut denominator = N::ONE;
    let mut numerator: N = N::ZERO;
    let mut float_part = number.abs().fract();
    let mut zero_value = F::EPSILON;

    while float_part.abs() > zero_value {
      numerator = numerator.try_mul(N::TEN)?;
      numerator = numerator.try_add(N::from((float_part.clone() * F::TEN).trunc().to_u8()))?;
      denominator = denominator.try_mul(N::TEN)?;
      float_part = (float_part.clone() * F::TEN).fract();
      zero_value = F::TEN * zero_value;
    }

    Fraction::try_new(sign, numerator, denominator)
  }

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

  #[inline]
  fn to_ratio_string(&self) -> String {
    if self.is_zero() {
      "0".to_string()
    } else {
      format!("{}{}/{}", self.sign(), self.numerator(), self.denominator(),)
    }
  }

  #[inline]
  fn get_remainder_decimal_string(&self, precision: usize) -> String {
    let mut remainder_decimal_string = String::default();

    let mut remainder = self.abs().remainder();
    for _ in 0..precision {
      if !remainder.is_zero() {
        let bigger_numerator = remainder.numerator() * N::TEN;

        let digit = bigger_numerator.clone() / remainder.denominator();

        remainder_decimal_string.push_str(&format!("{}", digit));

        remainder = Fraction::new(
          Sign::Positive,
          bigger_numerator.clone() - remainder.denominator() * digit,
          remainder.denominator(),
        );
      } else {
        remainder_decimal_string.push('0');
      }
    }

    remainder_decimal_string
  }
}

impl<N: UnsignedNumber> PartialEq for Fraction<N> {
  fn eq(&self, other: &Self) -> bool {
    let (unified_self, unified_other) = self.unify(other).unwrap();

    unified_self.numerator() == unified_other.numerator()
      && unified_self.denominator() == unified_other.denominator()
      && unified_self.sign() == unified_other.sign()
  }
}

impl<N: UnsignedNumber> hash::Hash for Fraction<N> {
  fn hash<H: hash::Hasher>(&self, state: &mut H) {
    let simplified_self = self.clone().simplify();

    simplified_self.numerator().hash(state);
    simplified_self.denominator().hash(state);
    simplified_self.sign().hash(state);
  }
}

impl<N: UnsignedNumber> PartialOrd for Fraction<N> {
  fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
    let (unified_self, unified_other) = self.unify(other).unwrap();

    match (unified_self.sign(), unified_other.sign()) {
      (Sign::Negative, Sign::Positive) => Some(cmp::Ordering::Less),
      (Sign::Positive, Sign::Negative) => Some(cmp::Ordering::Greater),
      _ => {
        if unified_self.is_negative() {
          if unified_self.numerator() > unified_other.numerator() {
            return Some(cmp::Ordering::Less);
          }

          if unified_self.numerator() < unified_other.numerator() {
            return Some(cmp::Ordering::Greater);
          }
        } else {
          if unified_self.numerator() > unified_other.numerator() {
            return Some(cmp::Ordering::Greater);
          }

          if unified_self.numerator() < unified_other.numerator() {
            return Some(cmp::Ordering::Less);
          }
        }

        Some(cmp::Ordering::Equal)
      }
    }
  }
}

impl<N: UnsignedNumber> fmt::Debug for Fraction<N> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Fraction")
      .field("sign", &self.sign())
      .field("numerator", &self.numerator())
      .field("denominator", &self.denominator())
      .finish()
  }
}

impl<N: UnsignedNumber> fmt::Display for Fraction<N> {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    if let Some(precision) = formatter.precision() {
      if self.is_negative() {
        write!(formatter, "-")?;
      }
      write!(formatter, "{}.", self.abs().trunc())?;
      write!(
        formatter,
        "{}",
        self.get_remainder_decimal_string(precision)
      )
    } else {
      write!(formatter, "{}", self.to_ratio_string())
    }
  }
}

impl<N: UnsignedNumber> Default for Fraction<N> {
  fn default() -> Fraction<N> {
    Fraction::new_zero()
  }
}

impl<N: UnsignedNumber> From<N> for Fraction<N> {
  fn from(number: N) -> Fraction<N> {
    Fraction::new_natural(number)
  }
}

impl<N: UnsignedNumber> From<&N> for Fraction<N> {
  fn from(number: &N) -> Fraction<N> {
    Fraction::new_natural(number.clone())
  }
}

impl<N: UnsignedNumber> convert::TryFrom<f32> for Fraction<N> {
  type Error = OperationError;

  fn try_from(number: f32) -> Result<Self, Self::Error> {
    Fraction::try_from_float_number(number)
  }
}

impl<N: UnsignedNumber> convert::TryFrom<f64> for Fraction<N> {
  type Error = OperationError;

  fn try_from(number: f64) -> Result<Self, Self::Error> {
    Fraction::try_from_float_number(number)
  }
}

impl<N: UnsignedNumber> convert::TryFrom<&str> for Fraction<N> {
  type Error = OperationError;

  fn try_from(number: &str) -> Result<Self, Self::Error> {
    if let Ok(natural_number) = number.parse::<N>() {
      return Ok(Fraction::new_natural(natural_number));
    }

    if let Ok(natural_number) = number.replace("-", "").parse::<N>() {
      return Fraction::try_new(Sign::Negative, natural_number, N::ONE);
    }

    match number.parse::<f64>() {
      Ok(number) => Fraction::try_from(number),
      Err(error) => Err(OperationError::new(
        format!("Failed to convert from string, parsing error ({})", error),
        OperationErrorType::ConvertionError,
      )),
    }
  }
}
