use std::error;
use std::fmt;

#[derive(fmt::Debug, Clone, Hash, PartialEq, Eq)]
pub struct OperationError {
  error_type: OperationErrorType,
  message: String,
}

#[derive(fmt::Debug, Clone, PartialEq, Eq, Hash)]
pub enum OperationErrorType {
  Overflow,
  DivisionByZero,
  ConvertionError,
}

impl OperationError {
  pub fn new<M: AsRef<str>>(message: M, error_type: OperationErrorType) -> OperationError {
    OperationError {
      error_type,
      message: message.as_ref().to_string(),
    }
  }

  pub fn message(&self) -> String {
    self.message.clone()
  }

  pub fn error_type(&self) -> OperationErrorType {
    self.error_type.clone()
  }
}

impl fmt::Display for OperationError {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      formatter,
      "error caused by {}, reason: {}",
      self.error_type, self.message
    )
  }
}

impl fmt::Display for OperationErrorType {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      OperationErrorType::Overflow => write!(formatter, "overflow"),
      OperationErrorType::DivisionByZero => write!(formatter, "division by zero"),
      OperationErrorType::ConvertionError => write!(formatter, "convertion"),
    }
  }
}

impl error::Error for OperationError {}
