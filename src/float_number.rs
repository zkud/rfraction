use std::fmt;
use std::ops;

#[cfg(feature = "convertions")]
pub trait FloatNumber:
  fmt::Display
  + fmt::Debug
  + Clone
  + PartialOrd
  + ops::Add<Output = Self>
  + ops::Sub<Output = Self>
  + ops::Mul<Output = Self>
  + ops::Div<Output = Self>
  + ops::Rem<Output = Self>
  + ops::Mul<Output = Self>
{
  const EPSILON: Self;
  const TEN: Self;

  fn trunc(&self) -> Self;
  fn fract(&self) -> Self;
  fn floor(&self) -> Self;
  fn rem_euclid(&self, modulo: Self) -> Self;
  fn is_nan(&self) -> bool;
  fn is_infinite(&self) -> bool;
  fn abs(&self) -> Self;
  fn to_u8(&self) -> u8;
}

impl FloatNumber for f32 {
  const EPSILON: Self = f32::EPSILON;
  const TEN: Self = 10.0;

  fn trunc(&self) -> Self {
    (*self).trunc()
  }

  fn fract(&self) -> Self {
    (*self).fract()
  }

  fn floor(&self) -> Self {
    (*self).floor()
  }

  fn rem_euclid(&self, modulo: Self) -> Self {
    (*self).rem_euclid(modulo)
  }

  fn is_nan(&self) -> bool {
    (*self).is_nan()
  }

  fn is_infinite(&self) -> bool {
    (*self).is_infinite()
  }

  fn abs(&self) -> Self {
    (*self).abs()
  }

  fn to_u8(&self) -> u8 {
    *self as u8
  }
}

impl FloatNumber for f64 {
  const EPSILON: Self = f64::EPSILON;
  const TEN: Self = 10.0;

  fn trunc(&self) -> Self {
    (*self).trunc()
  }

  fn fract(&self) -> Self {
    (*self).fract()
  }

  fn floor(&self) -> Self {
    (*self).floor()
  }

  fn rem_euclid(&self, modulo: Self) -> Self {
    (*self).rem_euclid(modulo)
  }

  fn is_nan(&self) -> bool {
    (*self).is_nan()
  }

  fn is_infinite(&self) -> bool {
    (*self).is_infinite()
  }

  fn abs(&self) -> Self {
    (*self).abs()
  }

  fn to_u8(&self) -> u8 {
    *self as u8
  }
}
