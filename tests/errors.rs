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
