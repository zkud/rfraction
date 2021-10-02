use super::operation_error::OperationError;
use super::operation_error::OperationErrorType;
use super::unsigned_number::UnsignedNumber;
use std::cmp;
use std::convert;
use std::fmt;
use std::hash;
use std::ops;

#[derive(Clone, Eq)]
pub struct Fraction<N: UnsignedNumber> {
  numerator: N,
  denominator: N,
  is_negative: bool,
}

impl<N: UnsignedNumber> Fraction<N> {
  pub fn abs(&self) -> Fraction<N> {
    self.try_abs().unwrap()
  }

  pub fn try_abs(&self) -> Result<Fraction<N>, OperationError> {
    Fraction::new(false, self.numerator(), self.denominator())
  }

  pub fn neg(&self) -> Fraction<N> {
    self.try_neg().unwrap()
  }

  pub fn try_neg(&self) -> Result<Fraction<N>, OperationError> {
    Fraction::new(!self.is_negative(), self.numerator(), self.denominator())
  }

  pub fn add(&self, other: &Fraction<N>) -> Fraction<N> {
    self.try_add(other).unwrap()
  }

  pub fn try_add(&self, other: &Fraction<N>) -> Result<Fraction<N>, OperationError> {
    let (unified_self, unified_other) = self.unify(other)?;

    if unified_self.is_negative == unified_other.is_negative {
      return Fraction::new(
        unified_self.is_negative,
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
      Ok(Fraction::new(
        unified_self.is_negative,
        unified_self.numerator - unified_other.numerator,
        unified_self.denominator,
      )?)
    } else {
      Ok(Fraction::new(
        unified_other.is_negative,
        unified_other.numerator - unified_self.numerator,
        unified_self.denominator,
      )?)
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

    Fraction::new(self.is_negative ^ other.is_negative, numerator, denominator)
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
      Ok(Fraction::new(
        self.is_negative,
        self.denominator,
        self.numerator,
      )?)
    }
  }

  pub fn numerator(&self) -> N {
    self.numerator
  }

  pub fn denominator(&self) -> N {
    self.denominator
  }

  pub fn is_negative(&self) -> bool {
    if self.is_zero() {
      false
    } else {
      self.is_negative
    }
  }

  pub fn is_positive(&self) -> bool {
    if self.is_zero() {
      false
    } else {
      !self.is_negative
    }
  }

  pub fn is_natural(&self) -> bool {
    self.is_positive() && self.numerator() >= N::from(1) && self.denominator() == N::from(1)
  }

  pub fn is_zero(&self) -> bool {
    self.numerator() == N::from(0) && self.denominator() != N::from(0)
  }

  pub fn new(
    is_negative: bool,
    numerator: N,
    denominator: N,
  ) -> Result<Fraction<N>, OperationError> {
    if denominator == N::from(0) {
      Err(OperationError::new(
        "Denominator can not be zero",
        OperationErrorType::DivisionByZero,
      ))
    } else {
      Ok(
        Fraction {
          numerator,
          denominator,
          is_negative,
        }
        .simplify(),
      )
    }
  }

  pub fn new_zero() -> Fraction<N> {
    Fraction {
      numerator: N::from(0),
      denominator: N::from(1),
      is_negative: false,
    }
  }

  pub fn new_natural(value: N) -> Fraction<N> {
    Fraction {
      numerator: value,
      denominator: N::from(1),
      is_negative: false,
    }
  }

  #[inline]
  fn simplify(mut self) -> Fraction<N> {
    if !self.is_zero() {
      let gcd = self.find_gcd(self.numerator, self.denominator);

      self.numerator = self.numerator / gcd;
      self.denominator = self.denominator / gcd;
    }

    self
  }

  #[inline]
  fn find_gcd(&self, mut a: N, mut b: N) -> N {
    while b != N::from(0) {
      let c = b;
      b = a % b;
      a = c;
    }

    a
  }

  #[inline]
  fn unify(&self, other: &Fraction<N>) -> Result<(Fraction<N>, Fraction<N>), OperationError> {
    match self.denominator {
      x if x == other.denominator => Ok((self.clone(), other.clone())),
      x if other.denominator % x == N::from(0) => {
        let scale = other.denominator / x;
        Ok((self.mul_with_number(scale)?, other.clone()))
      }
      x if x % other.denominator == N::from(0) => {
        let scale = x / other.denominator;
        Ok((self.clone(), other.mul_with_number(scale)?))
      }
      _ => Ok((
        self.mul_with_number(other.denominator)?,
        other.mul_with_number(self.denominator)?,
      )),
    }
  }

  #[inline]
  fn mul_with_number(&self, number: N) -> Result<Fraction<N>, OperationError> {
    let numerator = self.numerator.try_mul(number)?;
    let denominator = self.denominator.try_mul(number)?;

    Ok(Fraction {
      numerator,
      denominator,
      is_negative: self.is_negative,
    })
  }
}

impl<N: UnsignedNumber> PartialEq for Fraction<N> {
  fn eq(&self, other: &Self) -> bool {
    let (unified_self, unified_other) = self.unify(other).unwrap();

    unified_self.numerator() == unified_other.numerator()
      && unified_self.denominator() == unified_other.denominator()
      && unified_self.is_negative() == unified_other.is_negative()
  }
}

impl<N: UnsignedNumber> hash::Hash for Fraction<N> {
  fn hash<H: hash::Hasher>(&self, state: &mut H) {
    let simplified_self = self.clone().simplify();

    simplified_self.numerator().hash(state);
    simplified_self.denominator().hash(state);
    simplified_self.is_negative().hash(state);
  }
}

impl<N: UnsignedNumber> PartialOrd for Fraction<N> {
  fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
    let (unified_self, unified_other) = self.unify(other).unwrap();

    match (unified_self.is_negative(), unified_other.is_negative()) {
      (true, false) => Some(cmp::Ordering::Less),
      (false, true) => Some(cmp::Ordering::Greater),
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
      .field("is_negative", &self.is_negative())
      .field("numerator", &self.numerator())
      .field("denominator", &self.denominator())
      .finish()
  }
}

impl<N: UnsignedNumber> fmt::Display for Fraction<N> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if self.is_zero() {
      write!(f, "0")
    } else {
      write!(
        f,
        "{}{}/{}",
        if self.is_positive() { '+' } else { '-' },
        self.numerator(),
        self.denominator(),
      )
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
    Fraction::new_natural(*number)
  }
}

impl<N: UnsignedNumber> convert::TryFrom<f32> for Fraction<N> {
  type Error = OperationError;

  fn try_from(mut number: f32) -> Result<Self, Self::Error> {
    let mut denominator: N = N::from(1);

    for _ in 0..f32::MAX_10_EXP as usize {
      denominator = denominator.try_mul(N::from(10))?;
      number *= 10.0;

      if number.abs().fract() < f32::EPSILON {
        break;
      }
    }

    Fraction::new(number < 0.0, N::try_from_f32(number.abs())?, denominator)
  }
}

impl<N: UnsignedNumber> convert::TryFrom<f64> for Fraction<N> {
  type Error = OperationError;

  fn try_from(mut number: f64) -> Result<Self, Self::Error> {
    let mut denominator: N = N::from(1);

    for _ in 0..f64::MAX_10_EXP as usize {
      denominator = denominator.try_mul(N::from(10))?;
      number *= 10.0;

      if number.abs().fract() < f64::EPSILON {
        break;
      }
    }

    Fraction::new(number < 0.0, N::try_from_f64(number.abs())?, denominator)
  }
}

impl<N: UnsignedNumber> convert::TryFrom<&str> for Fraction<N> {
  type Error = OperationError;

  fn try_from(number: &str) -> Result<Self, Self::Error> {
    if let Ok(natural_number) = number.parse::<N>() {
      return Ok(Fraction::new_natural(natural_number));
    }

    if let Ok(natural_number) = number.replace("-", "").parse::<N>() {
      return Fraction::new(true, natural_number, N::from(1));
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
