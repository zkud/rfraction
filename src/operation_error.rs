use std::error;
use std::fmt;

#[derive(fmt::Debug, Clone, Hash)]
pub struct OperationError {
  error_type: OperationErrorType,
  message: String,
}

#[derive(fmt::Debug, Clone, PartialEq, Hash)]
pub enum OperationErrorType {
  Overflow,
  DivisionByZero,
}

impl fmt::Display for OperationErrorType {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      OperationErrorType::Overflow => write!(formatter, "overflow error"),
      OperationErrorType::DivisionByZero => write!(formatter, "division by zero"),
    }
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

impl error::Error for OperationError {}

unsafe impl Send for OperationError {}

unsafe impl Sync for OperationError {}

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

#[cfg(test)]
mod tests {
  use super::OperationError;
  use super::OperationErrorType;

  #[test]
  fn it_inits() {
    let division_error = OperationError::new("division error", OperationErrorType::DivisionByZero);

    assert_eq!(division_error.message(), "division error");
    assert_eq!(
      division_error.error_type(),
      OperationErrorType::DivisionByZero
    );

    let overflow_error = OperationError::new("operation error", OperationErrorType::Overflow);

    assert_eq!(overflow_error.message(), "operation error");
    assert_eq!(overflow_error.error_type(), OperationErrorType::Overflow);
  }
}
