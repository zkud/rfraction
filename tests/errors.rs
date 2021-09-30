use rfraction::OperationError;
use rfraction::OperationErrorType;

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

#[test]
fn it_displayble() {
  let division_error = OperationError::new("division error", OperationErrorType::DivisionByZero);
  assert_eq!(
    format!("{}", division_error),
    String::from("error caused by division by zero, reason: division error")
  );

  let overflow_error = OperationError::new("operation error", OperationErrorType::Overflow);
  assert_eq!(
    format!("{}", overflow_error),
    String::from("error caused by overflow error, reason: operation error")
  );
}
