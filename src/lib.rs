use std::fmt;
use std::ops;

pub trait UnsignedNumber:
  fmt::Display
  + Copy
  + Clone
  + PartialEq
  + Sized
  + From<u8>
  + PartialOrd
  + ops::Add<Output = Self>
  + ops::Sub<Output = Self>
  + ops::Mul<Output = Self>
  + ops::Div<Output = Self>
  + ops::Rem<Output = Self>
{
}

impl UnsignedNumber for u128 {}
impl UnsignedNumber for u64 {}
impl UnsignedNumber for u32 {}
impl UnsignedNumber for u8 {}

#[derive(Clone)]
pub struct Fraction<N: UnsignedNumber> {
  numerator: N,
  denominator: N,
  is_negative: bool,
}

impl<N: UnsignedNumber> fmt::Display for Fraction<N> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if self.is_negative {
      write!(f, "-{}/{}", self.numerator, self.denominator)
    } else {
      write!(f, "{}/{}", self.numerator, self.denominator)
    }
  }
}

impl<N: UnsignedNumber> Fraction<N> {
  pub fn new(numerator: N, denominator: N, is_negative: bool) -> Fraction<N> {
    Fraction {
      numerator,
      denominator,
      is_negative,
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
    Fraction::new(N::from(0), N::from(0), false)
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

  pub fn abs(&self) -> Fraction<N> {
    Fraction::new(self.numerator, self.denominator, false)
  }

  pub fn neg(&self) -> Fraction<N> {
    Fraction::new(self.numerator, self.denominator, !self.is_negative)
  }

  pub fn add(&self, other: &Fraction<N>) -> Fraction<N> {
    if let Some(result) = self.process_rare_numbers_for_add(other) {
      return result;
    }

    let (unified_self, unified_other) = self.unify(other);

    if unified_self.is_negative == unified_other.is_negative {
      return Fraction::new(
        unified_self.numerator + unified_other.numerator,
        unified_self.denominator,
        unified_self.is_negative,
      )
      .simplify();
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
    if other.is_zero() {
      return Some(self.clone());
    }
    if self.is_zero() {
      return Some(other.clone());
    }

    if self.is_nan() || other.is_nan() {
      return Some(Fraction::new_nan());
    }

    if self.is_negative_infinity() && other.is_positive_infinity() {
      return Some(Fraction::new_nan());
    }
    if self.is_positive_infinity() && other.is_negative_infinity() {
      return Some(Fraction::new_nan());
    }

    if self.is_infinity() || other.is_infinity() {
      return Some(self.clone());
    }

    None
  }

  pub fn sub(&self, other: &Fraction<N>) -> Fraction<N> {
    self.add(&other.neg())
  }

  pub fn mul(&self, other: &Fraction<N>) -> Fraction<N> {
    Fraction::new(
      self.numerator * other.numerator,
      self.denominator * other.denominator,
      self.is_negative ^ other.is_negative,
    )
    .simplify()
  }

  pub fn div(&self, other: &Fraction<N>) -> Fraction<N> {
    self.mul(&other.reverse())
  }

  pub fn reverse(&self) -> Fraction<N> {
    Fraction::new(self.denominator, self.numerator, self.is_negative)
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

  #[inline]
  fn mul_with_number(&self, number: N) -> Fraction<N> {
    Fraction::new(
      self.numerator * number,
      self.denominator * number,
      self.is_negative,
    )
  }

  #[inline]
  fn simplify(&self) -> Fraction<N> {
    if self.is_infinity() || self.is_zero() || self.is_nan() {
      return self.clone();
    }

    if self.numerator == N::from(1) || self.denominator == N::from(1) {
      return self.clone();
    }

    let gcd = self.find_gcd(self.numerator, self.denominator);
    Fraction::new(
      self.numerator / gcd,
      self.denominator / gcd,
      self.is_negative,
    )
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
}
