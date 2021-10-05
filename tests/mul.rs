use rfraction::Fraction;
use rfraction::OperationErrorType;
use rfraction::Sign;

#[test]
fn with_usual_u128_nums_mul_works() {
  let first = Fraction::<u128>::new(Sign::Negative, 13, 17).unwrap();
  let second = Fraction::<u128>::new(Sign::Positive, 18, 19).unwrap();

  let result = first * second;

  assert!(result.is_negative());
  assert_eq!(result.numerator(), 234);
  assert_eq!(result.denominator(), 323);
}

#[test]
fn with_usual_u64_nums_mul_works() {
  let first = Fraction::<u64>::new(Sign::Negative, 13, 17).unwrap();
  let second = Fraction::<u64>::new(Sign::Positive, 18, 19).unwrap();

  let result = first * second;

  assert!(result.is_negative());
  assert_eq!(result.numerator(), 234);
  assert_eq!(result.denominator(), 323);
}

#[test]
fn with_usual_u32_nums_mul_works() {
  let first = Fraction::<u32>::new(Sign::Negative, 13, 17).unwrap();
  let second = Fraction::<u32>::new(Sign::Positive, 18, 19).unwrap();

  let result = first * second;

  assert!(result.is_negative());
  assert_eq!(result.numerator(), 234);
  assert_eq!(result.denominator(), 323);
}

#[test]
fn with_usual_u16_nums_mul_works() {
  let first = Fraction::<u16>::new(Sign::Negative, 13, 17).unwrap();
  let second = Fraction::<u16>::new(Sign::Positive, 18, 19).unwrap();

  let result = first * second;

  assert!(result.is_negative());
  assert_eq!(result.numerator(), 234);
  assert_eq!(result.denominator(), 323);
}

#[test]
fn with_usual_u8_nums_mul_works() {
  let first = Fraction::<u8>::new(Sign::Negative, 13, 11).unwrap();
  let second = Fraction::<u8>::new(Sign::Positive, 18, 19).unwrap();

  let result = first * second;

  assert!(result.is_negative());
  assert_eq!(result.numerator(), 234);
  assert_eq!(result.denominator(), 209);
}

#[test]
fn with_zeros_nums_mul_works() {
  let default_number = Fraction::<u128>::new_natural(123);
  let zero = Fraction::<u128>::new_zero();

  let result = (&default_number) * (&zero);
  assert!(result.is_zero());

  let result = (&zero) * (&default_number);
  assert!(result.is_zero());

  let result = (&zero) * (&zero);
  assert!(result.is_zero());
}

#[test]
#[should_panic]
fn with_overflowing_nums_mul_works() {
  let first = Fraction::<u8>::new_natural(150);
  let second = Fraction::<u8>::new_natural(150);

  first.mul(&second);
}

#[test]
fn with_operator_overloads_it_works() {
  let first = Fraction::<u128>::new_natural(1);
  let second = Fraction::<u128>::new_natural(18);

  let mut result = &first * &second;
  assert_eq!(result, Fraction::new_natural(18));

  result *= &second;
  assert_eq!(result, Fraction::new_natural(324));

  result *= second;
  assert_eq!(result, Fraction::new_natural(5832));
}

#[test]
fn its_possible_to_handle_an_overflow_with_u8() {
  let first = Fraction::<u8>::new_natural(150);
  let second = Fraction::<u8>::new_natural(150);

  match first.try_mul(&second) {
    Err(error) => {
      assert_eq!(error.error_type(), OperationErrorType::Overflow);
    }
    _ => panic!("It's impossible to handle an overflow"),
  }
}

#[test]
fn its_possible_to_handle_an_overflow_with_u16() {
  let first = Fraction::<u16>::new_natural(u16::MAX - 1);
  let second = Fraction::<u16>::new_natural(u16::MAX - 1);

  match first.try_mul(&second) {
    Err(error) => {
      assert_eq!(error.error_type(), OperationErrorType::Overflow);
    }
    _ => panic!("It's impossible to handle an overflow"),
  }
}

#[test]
fn its_possible_to_handle_an_overflow_with_u32() {
  let first = Fraction::<u32>::new_natural(u32::MAX - 1);
  let second = Fraction::<u32>::new_natural(u32::MAX - 1);

  match first.try_mul(&second) {
    Err(error) => {
      assert_eq!(error.error_type(), OperationErrorType::Overflow);
    }
    _ => panic!("It's impossible to handle an overflow"),
  }
}

#[test]
fn its_possible_to_handle_an_overflow_with_u64() {
  let first = Fraction::<u64>::new_natural(u64::MAX - 1);
  let second = Fraction::<u64>::new_natural(u64::MAX - 1);

  match first.try_mul(&second) {
    Err(error) => {
      assert_eq!(error.error_type(), OperationErrorType::Overflow);
    }
    _ => panic!("It's impossible to handle an overflow"),
  }
}

#[test]
fn its_possible_to_handle_an_overflow_with_u128() {
  let first = Fraction::<u128>::new_natural(u128::MAX - 1);
  let second = Fraction::<u128>::new_natural(u128::MAX - 1);

  match first.try_mul(&second) {
    Err(error) => {
      assert_eq!(error.error_type(), OperationErrorType::Overflow);
    }
    _ => panic!("It's impossible to handle an overflow"),
  }
}
