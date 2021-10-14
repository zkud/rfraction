use super::convertable_to::ConvertableTo;
use super::operation_error::OperationError;
use super::operation_error::OperationErrorType;
use super::sign::Sign;
use super::unsigned_number::UnsignedNumber;
use serde;
use std::cmp;
use std::convert;
use std::fmt;
use std::hash;
use std::ops;

#[derive(Clone, Eq, serde::Serialize, serde::Deserialize)]
pub struct Fraction<N: UnsignedNumber> {
  numerator: N,
  denominator: N,
  sign: Sign,
}

impl<N: UnsignedNumber> Fraction<N> {
  pub fn abs(&self) -> Fraction<N> {
    self.try_abs().unwrap()
  }

  pub fn try_abs(&self) -> Result<Fraction<N>, OperationError> {
    Fraction::try_new(Sign::Positive, self.numerator(), self.denominator())
  }

  pub fn neg(&self) -> Fraction<N> {
    self.try_neg().unwrap()
  }

  pub fn try_neg(&self) -> Result<Fraction<N>, OperationError> {
    Fraction::try_new(self.sign.inverse(), self.numerator(), self.denominator())
  }

  pub fn add(&self, other: &Fraction<N>) -> Fraction<N> {
    self.try_add(other).unwrap()
  }

  pub fn try_add(&self, other: &Fraction<N>) -> Result<Fraction<N>, OperationError> {
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

  pub fn sub(&self, other: &Fraction<N>) -> Fraction<N> {
    self.try_sub(other).unwrap()
  }

  pub fn try_sub(&self, other: &Fraction<N>) -> Result<Fraction<N>, OperationError> {
    self.try_add(&other.try_neg()?)
  }

  pub fn mul(&self, other: &Fraction<N>) -> Fraction<N> {
    self.try_mul(other).unwrap()
  }

  pub fn try_mul(&self, other: &Fraction<N>) -> Result<Fraction<N>, OperationError> {
    let numerator = self.numerator().try_mul(other.numerator())?;
    let denominator = self.denominator().try_mul(other.denominator())?;

    Fraction::try_new(self.sign.mul(&other.sign), numerator, denominator)
  }

  pub fn div(&self, other: &Fraction<N>) -> Fraction<N> {
    self.try_div(other).unwrap()
  }

  pub fn try_div(&self, other: &Fraction<N>) -> Result<Fraction<N>, OperationError> {
    self.try_mul(&other.try_reverse()?)
  }

  pub fn reverse(&self) -> Fraction<N> {
    self.try_reverse().unwrap()
  }

  pub fn try_reverse(&self) -> Result<Fraction<N>, OperationError> {
    if self.is_zero() {
      Err(OperationError::new(
        "Numerator is zero, can't divide by zero",
        OperationErrorType::DivisionByZero,
      ))
    } else {
      Fraction::try_new(self.sign, self.denominator.clone(), self.numerator.clone())
    }
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

  pub fn numerator(&self) -> N {
    self.numerator.clone()
  }

  pub fn denominator(&self) -> N {
    self.denominator.clone()
  }

  pub fn sign(&self) -> Sign {
    self.sign
  }

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

  pub fn to_ratio_string(&self) -> String {
    if self.is_zero() {
      "0".to_string()
    } else {
      format!("{}{}/{}", self.sign(), self.numerator(), self.denominator(),)
    }
  }

  pub fn new(sign: Sign, numerator: N, denominator: N) -> Fraction<N> {
    Fraction::try_new(sign, numerator, denominator).unwrap()
  }

  pub fn try_new(sign: Sign, numerator: N, denominator: N) -> Result<Fraction<N>, OperationError> {
    if denominator == N::ZERO {
      Err(OperationError::new(
        "Denominator can not be zero",
        OperationErrorType::DivisionByZero,
      ))
    } else {
      Ok(
        Fraction {
          numerator,
          denominator,
          sign,
        }
        .simplify(),
      )
    }
  }

  pub fn new_zero() -> Fraction<N> {
    Fraction {
      numerator: N::ZERO,
      denominator: N::ONE,
      sign: Sign::Positive,
    }
  }

  pub fn new_natural(value: N) -> Fraction<N> {
    Fraction {
      numerator: value,
      denominator: N::ONE,
      sign: Sign::Positive,
    }
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
    match self.denominator.clone() {
      x if x == other.denominator => Ok((self.clone(), other.clone())),
      x if other.denominator.clone() % x.clone() == N::ZERO => {
        let scale = other.denominator.clone() / x;
        Ok((self.mul_with_number(scale)?, other.clone()))
      }
      x if x.clone() % other.denominator.clone() == N::ONE => {
        let scale = x / other.denominator.clone();
        Ok((self.clone(), other.mul_with_number(scale)?))
      }
      _ => Ok((
        self.mul_with_number(other.denominator.clone())?,
        other.mul_with_number(self.denominator.clone())?,
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

impl<N: UnsignedNumber> ops::Add for &Fraction<N> {
  type Output = Fraction<N>;

  fn add(self, other: Self) -> Fraction<N> {
    self.add(other)
  }
}

impl<N: UnsignedNumber> ops::Add for Fraction<N> {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    (&self).add(&other)
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

impl<N: UnsignedNumber> ops::Div for &Fraction<N> {
  type Output = Fraction<N>;

  fn div(self, other: Self) -> Fraction<N> {
    self.div(other)
  }
}

impl<N: UnsignedNumber> ops::Div for Fraction<N> {
  type Output = Self;

  fn div(self, other: Self) -> Self {
    (&self).div(&other)
  }
}

impl<N: UnsignedNumber> ops::DivAssign<&Self> for Fraction<N> {
  fn div_assign(&mut self, other: &Self) {
    *self = self.div(other)
  }
}

impl<N: UnsignedNumber> ops::DivAssign<Self> for Fraction<N> {
  fn div_assign(&mut self, other: Self) {
    *self = self.div(&other)
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

  fn try_from(mut number: f32) -> Result<Self, Self::Error> {
    let mut denominator: N = N::ONE;

    for _ in 0..f32::MAX_10_EXP as usize {
      denominator = denominator.try_mul(N::TEN)?;
      number *= 10.0;

      if number.abs().fract() < f32::EPSILON {
        break;
      }
    }

    if number < 0.0 {
      Fraction::try_new(Sign::Negative, N::try_from_f32(number.abs())?, denominator)
    } else {
      Fraction::try_new(Sign::Positive, N::try_from_f32(number.abs())?, denominator)
    }
  }
}

impl<N: UnsignedNumber> convert::TryFrom<f64> for Fraction<N> {
  type Error = OperationError;

  fn try_from(mut number: f64) -> Result<Self, Self::Error> {
    let mut denominator: N = N::ONE;

    for _ in 0..f64::MAX_10_EXP as usize {
      denominator = denominator.try_mul(N::TEN)?;
      number *= 10.0;

      if number.abs().fract() < f64::EPSILON {
        break;
      }
    }

    if number < 0.0 {
      Fraction::try_new(Sign::Negative, N::try_from_f64(number.abs())?, denominator)
    } else {
      Fraction::try_new(Sign::Positive, N::try_from_f64(number.abs())?, denominator)
    }
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
