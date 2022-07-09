use super::OperationError;
use super::OperationErrorType;
use std::fmt;
use std::hash;
use std::ops;
use std::str;

pub trait UnsignedNumber:
  fmt::Display
  + fmt::Debug
  + Clone
  + Eq
  + Ord
  + hash::Hash
  + str::FromStr
  + From<u8>
  + ops::Add<Output = Self>
  + ops::Sub<Output = Self>
  + ops::Mul<Output = Self>
  + ops::Div<Output = Self>
  + ops::Rem<Output = Self>
{
  const ZERO: Self;
  const ONE: Self;
  const TEN: Self;

  fn try_add(self, other: Self) -> Result<Self, OperationError>;
  fn try_mul(self, other: Self) -> Result<Self, OperationError>;
}

impl UnsignedNumber for u128 {
  const ZERO: Self = 0;
  const ONE: Self = 1;
  const TEN: Self = 10;

  fn try_add(self, other: Self) -> Result<Self, OperationError> {
    match self.checked_add(other) {
      Some(number) => Ok(number),
      None => Err(OperationError::new(
        "overflow during adding of u128 numbers",
        OperationErrorType::Overflow,
      )),
    }
  }

  fn try_mul(self, other: Self) -> Result<Self, OperationError> {
    match self.checked_mul(other) {
      Some(number) => Ok(number),
      None => Err(OperationError::new(
        "overflow during multiplication of u128 numbers",
        OperationErrorType::Overflow,
      )),
    }
  }
}

impl UnsignedNumber for u64 {
  const ZERO: Self = 0;
  const ONE: Self = 1;
  const TEN: Self = 10;

  fn try_add(self, other: Self) -> Result<Self, OperationError> {
    match self.checked_add(other) {
      Some(number) => Ok(number),
      None => Err(OperationError::new(
        "overflow during adding of u64 numbers",
        OperationErrorType::Overflow,
      )),
    }
  }

  fn try_mul(self, other: Self) -> Result<Self, OperationError> {
    match self.checked_mul(other) {
      Some(number) => Ok(number),
      None => Err(OperationError::new(
        "overflow during multiplication of u64 numbers",
        OperationErrorType::Overflow,
      )),
    }
  }
}

impl UnsignedNumber for u32 {
  const ZERO: Self = 0;
  const ONE: Self = 1;
  const TEN: Self = 10;

  fn try_add(self, other: Self) -> Result<Self, OperationError> {
    match self.checked_add(other) {
      Some(number) => Ok(number),
      None => Err(OperationError::new(
        "overflow during adding of u32 numbers",
        OperationErrorType::Overflow,
      )),
    }
  }

  fn try_mul(self, other: Self) -> Result<Self, OperationError> {
    match self.checked_mul(other) {
      Some(number) => Ok(number),
      None => Err(OperationError::new(
        "overflow during multiplication of u32 numbers",
        OperationErrorType::Overflow,
      )),
    }
  }
}

impl UnsignedNumber for u16 {
  const ZERO: Self = 0;
  const ONE: Self = 1;
  const TEN: Self = 10;

  fn try_add(self, other: Self) -> Result<Self, OperationError> {
    match self.checked_add(other) {
      Some(number) => Ok(number),
      None => Err(OperationError::new(
        "overflow during adding of u16 numbers",
        OperationErrorType::Overflow,
      )),
    }
  }

  fn try_mul(self, other: Self) -> Result<Self, OperationError> {
    match self.checked_mul(other) {
      Some(number) => Ok(number),
      None => Err(OperationError::new(
        "overflow during multiplication of u16 numbers",
        OperationErrorType::Overflow,
      )),
    }
  }
}

impl UnsignedNumber for u8 {
  const ZERO: Self = 0;
  const ONE: Self = 1;
  const TEN: Self = 10;

  fn try_add(self, other: Self) -> Result<Self, OperationError> {
    match self.checked_add(other) {
      Some(number) => Ok(number),
      None => Err(OperationError::new(
        "overflow during adding of u8 numbers",
        OperationErrorType::Overflow,
      )),
    }
  }

  fn try_mul(self, other: Self) -> Result<Self, OperationError> {
    match self.checked_mul(other) {
      Some(number) => Ok(number),
      None => Err(OperationError::new(
        "overflow during multiplication of u8 numbers",
        OperationErrorType::Overflow,
      )),
    }
  }
}
