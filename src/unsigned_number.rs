use super::OperationError;
use super::OperationErrorType;
use std::fmt;
use std::hash;
use std::ops;
use std::str;

pub trait UnsignedNumber:
  fmt::Display
  + fmt::Debug
  + Copy
  + Clone
  + Eq
  + From<u8>
  + Ord
  + hash::Hash
  + str::FromStr
  + ops::Add<Output = Self>
  + ops::Sub<Output = Self>
  + ops::Mul<Output = Self>
  + ops::Div<Output = Self>
  + ops::Rem<Output = Self>
{
  fn try_add(self, other: Self) -> Result<Self, OperationError>;
  fn try_mul(self, other: Self) -> Result<Self, OperationError>;
  fn try_from_f32(value: f32) -> Result<Self, OperationError>;
  fn try_from_f64(value: f64) -> Result<Self, OperationError>;
}

impl UnsignedNumber for u128 {
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

  fn try_from_f32(value: f32) -> Result<Self, OperationError> {
    if value < 0.0 {
      Err(OperationError::new(
        "Unable to convert from negative values to unsigned ones",
        OperationErrorType::ConvertionError,
      ))
    } else {
      Ok(value as u128)
    }
  }

  fn try_from_f64(value: f64) -> Result<Self, OperationError> {
    if value < 0.0 {
      Err(OperationError::new(
        "Unable to convert from negative values to unsigned ones",
        OperationErrorType::ConvertionError,
      ))
    } else if value > u128::MAX as f64 {
      Err(OperationError::new(
        "F64 value is bigger than max for u128",
        OperationErrorType::ConvertionError,
      ))
    } else {
      Ok(value as u128)
    }
  }
}

impl UnsignedNumber for u64 {
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

  fn try_from_f32(value: f32) -> Result<Self, OperationError> {
    if value < 0.0 {
      Err(OperationError::new(
        "Unable to convert from negative values to unsigned ones",
        OperationErrorType::ConvertionError,
      ))
    } else if value > u64::MAX as f32 {
      Err(OperationError::new(
        "F32 value is bigger than max for u64",
        OperationErrorType::ConvertionError,
      ))
    } else {
      Ok(value as u64)
    }
  }

  fn try_from_f64(value: f64) -> Result<Self, OperationError> {
    if value < 0.0 {
      Err(OperationError::new(
        "Unable to convert from negative values to unsigned ones",
        OperationErrorType::ConvertionError,
      ))
    } else if value > u64::MAX as f64 {
      Err(OperationError::new(
        "F64 value is bigger than max for u64",
        OperationErrorType::ConvertionError,
      ))
    } else {
      Ok(value as u64)
    }
  }
}

impl UnsignedNumber for u32 {
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

  fn try_from_f32(value: f32) -> Result<Self, OperationError> {
    if value < 0.0 {
      Err(OperationError::new(
        "Unable to convert from negative values to unsigned ones",
        OperationErrorType::ConvertionError,
      ))
    } else if value > u32::MAX as f32 {
      Err(OperationError::new(
        "F32 value is bigger than max for u32",
        OperationErrorType::ConvertionError,
      ))
    } else {
      Ok(value as u32)
    }
  }

  fn try_from_f64(value: f64) -> Result<Self, OperationError> {
    if value < 0.0 {
      Err(OperationError::new(
        "Unable to convert from negative values to unsigned ones",
        OperationErrorType::ConvertionError,
      ))
    } else if value > u32::MAX as f64 {
      Err(OperationError::new(
        "F64 value is bigger than max for u32",
        OperationErrorType::ConvertionError,
      ))
    } else {
      Ok(value as u32)
    }
  }
}

impl UnsignedNumber for u16 {
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

  fn try_from_f32(value: f32) -> Result<Self, OperationError> {
    if value < 0.0 {
      Err(OperationError::new(
        "Unable to convert from negative values to unsigned ones",
        OperationErrorType::ConvertionError,
      ))
    } else if value > u16::MAX as f32 {
      Err(OperationError::new(
        "F32 value is bigger than max for u16",
        OperationErrorType::ConvertionError,
      ))
    } else {
      Ok(value as u16)
    }
  }

  fn try_from_f64(value: f64) -> Result<Self, OperationError> {
    if value < 0.0 {
      Err(OperationError::new(
        "Unable to convert from negative values to unsigned ones",
        OperationErrorType::ConvertionError,
      ))
    } else {
      if value > u16::MAX as f64 {
        Err(OperationError::new(
          "F64 value is bigger than max for u16",
          OperationErrorType::ConvertionError,
        ))
      } else {
        Ok(value as u16)
      }
    }
  }
}

impl UnsignedNumber for u8 {
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

  fn try_from_f32(value: f32) -> Result<Self, OperationError> {
    if value < 0.0 {
      Err(OperationError::new(
        "Unable to convert from negative values to unsigned ones",
        OperationErrorType::ConvertionError,
      ))
    } else {
      if value > u8::MAX as f32 {
        Err(OperationError::new(
          "F32 value is bigger than max for u8",
          OperationErrorType::ConvertionError,
        ))
      } else {
        Ok(value as u8)
      }
    }
  }

  fn try_from_f64(value: f64) -> Result<Self, OperationError> {
    if value < 0.0 {
      Err(OperationError::new(
        "Unable to convert from negative values to unsigned ones",
        OperationErrorType::ConvertionError,
      ))
    } else {
      if value > u8::MAX as f64 {
        Err(OperationError::new(
          "F64 value is bigger than max for u8",
          OperationErrorType::ConvertionError,
        ))
      } else {
        Ok(value as u8)
      }
    }
  }
}
