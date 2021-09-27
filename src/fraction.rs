use super::unsigned_number::UnsignedNumber;
use std::cmp;
use std::fmt;
use std::hash;
use std::ops;

#[derive(Clone, Eq, Ord, hash::Hash)]
pub struct Fraction<N: UnsignedNumber> {
  numerator: N,
  denominator: N,
  is_negative: bool,
}

impl<N: UnsignedNumber> Fraction<N> {
  pub fn abs(&self) -> Fraction<N> {
    Fraction::new(self.numerator, self.denominator, false).simplify()
  }

  pub fn neg(&self) -> Fraction<N> {
    Fraction::new(self.numerator, self.denominator, !self.is_negative).simplify()
  }

  pub fn add(&self, other: &Fraction<N>) -> Fraction<N> {
    if let Some(result) = self.process_rare_numbers_for_add(other) {
      return result;
    }

    let (unified_self, unified_other) = self.unify(other);

    if unified_self.is_negative == unified_other.is_negative {
      if let Some(num_sum) = unified_self
        .numerator()
        .checked_add(unified_other.numerator())
      {
        return Fraction::new(num_sum, unified_self.denominator, unified_self.is_negative)
          .simplify();
      } else {
        return Fraction::new_nan();
      }
    }

    if unified_self.is_abs_equal(&unified_other) {
      return Fraction::new_zero();
    }

    if unified_self.is_abs_bigger(&unified_other) {
      Fraction::new(
        unified_self.numerator - unified_other.numerator,
        unified_self.denominator,
        unified_self.is_negative,
      )
      .simplify()
    } else {
      Fraction::new(
        unified_other.numerator - unified_self.numerator,
        unified_self.denominator,
        unified_other.is_negative,
      )
      .simplify()
    }
  }

  #[inline]
  fn process_rare_numbers_for_add(&self, other: &Fraction<N>) -> Option<Fraction<N>> {
    if self.is_nan() || other.is_nan() {
      return Some(Fraction::new_nan());
    }

    if self.is_negative_infinity() && other.is_positive_infinity() {
      return Some(Fraction::new_nan());
    }
    if self.is_positive_infinity() && other.is_negative_infinity() {
      return Some(Fraction::new_nan());
    }

    if self.is_infinity() {
      return Some(self.clone().simplify());
    }
    if other.is_infinity() {
      return Some(other.clone().simplify());
    }

    if self.is_zero() {
      return Some(other.clone().simplify());
    }
    if other.is_zero() {
      return Some(self.clone().simplify());
    }

    None
  }

  pub fn sub(&self, other: &Fraction<N>) -> Fraction<N> {
    self.add(&other.neg())
  }

  pub fn mul(&self, other: &Fraction<N>) -> Fraction<N> {
    if let Some(result) = self.process_rare_numbers_for_mul(other) {
      return result;
    }

    let new_numerator = self.numerator().checked_mul(other.numerator());
    let new_denominator = self.denominator().checked_mul(other.denominator());

    if let Some(numerator) = new_numerator {
      if let Some(denominator) = new_denominator {
        return Fraction::new(numerator, denominator, self.is_negative ^ other.is_negative)
          .simplify();
      }
    }

    Fraction::new_nan()
  }

  #[inline]
  fn process_rare_numbers_for_mul(&self, other: &Fraction<N>) -> Option<Fraction<N>> {
    if self.is_nan() || other.is_nan() {
      return Some(Fraction::new_nan());
    }

    if self.is_zero() && other.is_infinity() {
      return Some(Fraction::new_nan());
    }
    if self.is_infinity() && other.is_zero() {
      return Some(Fraction::new_nan());
    }

    if self.is_infinity() || other.is_infinity() {
      if self.is_negative() ^ other.is_negative() {
        return Some(Fraction::new_negative_infinity());
      } else {
        return Some(Fraction::new_positive_infinity());
      }
    }

    if self.is_zero() || other.is_zero() {
      return Some(Fraction::new_zero());
    }

    None
  }

  pub fn div(&self, other: &Fraction<N>) -> Fraction<N> {
    self.mul(&other.reverse())
  }

  pub fn reverse(&self) -> Fraction<N> {
    Fraction::new(self.denominator, self.numerator, self.is_negative).simplify()
  }

  pub fn numerator(&self) -> N {
    self.numerator
  }

  pub fn denominator(&self) -> N {
    self.denominator
  }

  pub fn is_negative(&self) -> bool {
    self.is_negative
  }

  pub fn is_positive(&self) -> bool {
    !self.is_negative
  }

  pub fn is_natural(&self) -> bool {
    !self.is_negative && self.numerator != N::from(0) && self.denominator == N::from(1)
  }

  pub fn is_nan(&self) -> bool {
    self.numerator == N::from(0) && self.denominator == N::from(0)
  }

  pub fn is_negative_infinity(&self) -> bool {
    self.is_negative && self.is_infinity()
  }

  pub fn is_positive_infinity(&self) -> bool {
    !self.is_negative && self.is_infinity()
  }

  pub fn is_infinity(&self) -> bool {
    self.numerator != N::from(0) && self.denominator == N::from(0)
  }

  pub fn is_zero(&self) -> bool {
    self.numerator == N::from(0) && self.denominator != N::from(0)
  }

  pub fn new(numerator: N, denominator: N, is_negative: bool) -> Fraction<N> {
    Fraction {
      numerator,
      denominator,
      is_negative,
    }
    .simplify()
  }

  pub fn new_positive_infinity() -> Fraction<N> {
    Fraction {
      numerator: N::from(1),
      denominator: N::from(0),
      is_negative: false,
    }
  }

  pub fn new_negative_infinity() -> Fraction<N> {
    Fraction {
      numerator: N::from(1),
      denominator: N::from(0),
      is_negative: true,
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

  pub fn new_nan() -> Fraction<N> {
    Fraction {
      numerator: N::from(0),
      denominator: N::from(0),
      is_negative: false,
    }
  }

  #[inline]
  fn simplify(mut self) -> Fraction<N> {
    if self.is_infinity() {
      self.numerator = N::from(1);
      self.denominator = N::from(0);
      return self;
    }

    if self.is_zero() {
      self.numerator = N::from(0);
      self.denominator = N::from(1);
      return self;
    }

    if self.is_nan() {
      self.numerator = N::from(0);
      self.denominator = N::from(0);
      self.is_negative = false;
      return self;
    }

    if self.numerator == N::from(1) || self.denominator == N::from(1) {
      return self;
    }

    let gcd = self.find_gcd(self.numerator, self.denominator);
    self.numerator = self.numerator / gcd;
    self.denominator = self.denominator / gcd;

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
  fn is_abs_equal(&self, other: &Fraction<N>) -> bool {
    self.numerator == other.numerator && self.denominator == other.denominator
  }

  #[inline]
  fn is_abs_bigger(&self, other: &Fraction<N>) -> bool {
    if self.denominator == other.denominator {
      self.numerator > other.numerator
    } else {
      let (unified_self, unified_other) = self.unify(other);
      unified_self.numerator > unified_other.numerator
    }
  }

  #[inline]
  fn unify(&self, other: &Fraction<N>) -> (Fraction<N>, Fraction<N>) {
    match self.denominator {
      x if x == other.denominator => (self.clone(), other.clone()),
      x if other.denominator % x == N::from(0) => {
        let scale = other.denominator / x;
        (self.mul_with_number(scale), other.clone())
      }
      x if x % other.denominator == N::from(0) => {
        let scale = x / other.denominator;
        (self.clone(), other.mul_with_number(scale))
      }
      _ => (
        self.mul_with_number(other.denominator),
        other.mul_with_number(self.denominator),
      ),
    }
  }

  #[inline]
  fn mul_with_number(&self, number: N) -> Fraction<N> {
    let new_numerator = self.numerator().checked_mul(number);
    let new_denominator = self.denominator().checked_mul(number);

    if let Some(numerator) = new_numerator {
      if let Some(denominator) = new_denominator {
        return Fraction {
          numerator,
          denominator,
          is_negative: self.is_negative,
        };
      }
    }

    Fraction::new_nan()
  }
}

impl<N: UnsignedNumber> PartialEq for Fraction<N> {
  fn eq(&self, other: &Self) -> bool {
    let (unified_self, unified_other) = self.unify(other);

    unified_self.numerator() == unified_other.numerator()
      && unified_self.denominator() == unified_other.denominator()
      && unified_self.is_negative() == unified_other.is_negative()
  }
}

impl<N: UnsignedNumber> PartialOrd for Fraction<N> {
  fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
    if self.is_nan() || other.is_nan() {
      return None;
    }

    match (self.is_infinity(), other.is_infinity()) {
      (true, false) => {
        if self.is_positive() {
          return Some(cmp::Ordering::Greater);
        } else {
          return Some(cmp::Ordering::Less);
        }
      }
      (false, true) => {
        if other.is_negative() {
          return Some(cmp::Ordering::Greater);
        } else {
          return Some(cmp::Ordering::Less);
        }
      }
      (true, true) => {
        if self.is_positive() && other.is_negative() {
          return Some(cmp::Ordering::Greater);
        }
        if self.is_negative() && other.is_positive() {
          return Some(cmp::Ordering::Less);
        }
        return None;
      }
      _ => (),
    }

    let (unified_self, unified_other) = self.unify(other);

    if unified_self.is_nan() || unified_other.is_nan() {
      return None;
    }

    match (unified_self.is_negative(), unified_other.is_negative()) {
      (true, false) => return Some(cmp::Ordering::Less),
      (false, true) => return Some(cmp::Ordering::Greater),
      _ => {
        if unified_self.is_negative() {
          if unified_self.numerator() > unified_other.denominator() {
            return Some(cmp::Ordering::Less);
          }

          if unified_self.numerator() < unified_other.denominator() {
            return Some(cmp::Ordering::Greater);
          }
        } else {
          if unified_self.numerator() > unified_other.denominator() {
            return Some(cmp::Ordering::Greater);
          }

          if unified_self.numerator() < unified_other.denominator() {
            return Some(cmp::Ordering::Less);
          }
        }

        return Some(cmp::Ordering::Equal);
      }
    }
  }
}

impl<N: UnsignedNumber> fmt::Debug for Fraction<N> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Fraction")
      .field("numerator", &self.numerator())
      .field("denominator", &self.denominator())
      .field("is_negative", &self.is_negative())
      .finish()
  }
}

impl<N: UnsignedNumber> fmt::Display for Fraction<N> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if self.is_positive_infinity() {
      return write!(f, "+infinity");
    }

    if self.is_negative_infinity() {
      return write!(f, "-infinity");
    }

    if self.is_nan() {
      return write!(f, "nan");
    }

    if self.is_zero() {
      return write!(f, "0");
    }

    write!(
      f,
      "{} {}/{}",
      if self.is_positive() { '+' } else { '-' },
      self.numerator(),
      self.denominator()
    )
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