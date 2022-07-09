use rfraction::Fraction;
use rfraction::OperationErrorType;
use rfraction::Sign;

#[test]
fn with_usual_u128_nums_add_works() {
  let first = Fraction::<u128>::new(Sign::Positive, 10, 140);
  let second = Fraction::<u128>::new(Sign::Positive, 15, 280);

  let result = first + second;

  assert!(result.is_positive());
  assert_eq!(result.numerator(), 1);
  assert_eq!(result.denominator(), 8);
}

#[test]
fn with_usual_u64_nums_add_works() {
  let first = Fraction::<u64>::new(Sign::Positive, 10, 140);
  let second = Fraction::<u64>::new(Sign::Positive, 15, 280);

  let result = first + second;

  assert!(result.is_positive());
  assert_eq!(result.numerator(), 1);
  assert_eq!(result.denominator(), 8);
}

#[test]
fn with_usual_u32_nums_add_works() {
  let first = Fraction::<u32>::new(Sign::Positive, 10, 140);
  let second = Fraction::<u32>::new(Sign::Positive, 15, 280);

  let result = first + second;

  assert!(result.is_positive());
  assert_eq!(result.numerator(), 1);
  assert_eq!(result.denominator(), 8);
}

#[test]
fn with_usual_u16_nums_add_works() {
  let first = Fraction::<u16>::new(Sign::Positive, 10, 140);
  let second = Fraction::<u16>::new(Sign::Positive, 15, 280);

  let result = first + second;

  assert!(result.is_positive());
  assert_eq!(result.numerator(), 1);
  assert_eq!(result.denominator(), 8);
}

#[test]
fn with_usual_u8_nums_add_works() {
  let first = Fraction::<u8>::new(Sign::Positive, 20, 20);
  let second = Fraction::<u8>::new(Sign::Positive, 15, 10);

  let result = first + second;

  assert!(result.is_positive());
  assert_eq!(result.numerator(), 5);
  assert_eq!(result.denominator(), 2);
}

#[test]
fn with_zeros_nums_add_works() {
  let default_number = Fraction::<u128>::new_natural(123);
  let zero = Fraction::<u128>::new_zero();

  let result = (&default_number) + (&zero);
  assert_eq!(result.numerator(), 123);
  assert_eq!(result.denominator(), 1);
  assert!(result.is_positive());

  let result = (&zero) + (&default_number);
  assert_eq!(result.numerator(), 123);
  assert_eq!(result.denominator(), 1);
  assert!(result.is_positive());

  let result = (&zero) + (&zero);
  assert!(result.is_zero());
}

#[test]
#[should_panic]
fn with_overflowing_nums_add_works() {
  let first = Fraction::<u8>::new_natural(150);
  let second = Fraction::<u8>::new_natural(150);

  first.add(&second);
}

#[test]
fn with_operator_overloads_it_works() {
  let first = Fraction::<u128>::new_natural(150);
  let second = Fraction::<u128>::new_natural(150);

  let mut result = &first + &second;
  assert_eq!(result, Fraction::new_natural(300));

  result += &second;
  assert_eq!(result, Fraction::new_natural(450));

  result += second;
  assert_eq!(result, Fraction::new_natural(600));
}

#[test]
fn its_possible_to_handle_overflows_with_u8() {
  let first = Fraction::<u8>::new_natural(150);
  let second = Fraction::<u8>::new_natural(150);

  match first.try_add(&second) {
    Err(error) => {
      assert_eq!(error.error_type(), OperationErrorType::Overflow);
    }
    _ => panic!("It's impossible to handle an overflow"),
  }
}

#[test]
fn its_possible_to_handle_overflows_with_u16() {
  let first = Fraction::<u16>::new_natural(u16::MAX - 1);
  let second = Fraction::<u16>::new_natural(u16::MAX - 1);

  match first.try_add(&second) {
    Err(error) => {
      assert_eq!(error.error_type(), OperationErrorType::Overflow);
    }
    _ => panic!("It's impossible to handle an overflow"),
  }
}

#[test]
fn its_possible_to_handle_overflows_with_u32() {
  let first = Fraction::<u32>::new_natural(u32::MAX - 1);
  let second = Fraction::<u32>::new_natural(u32::MAX - 1);

  match first.try_add(&second) {
    Err(error) => {
      assert_eq!(error.error_type(), OperationErrorType::Overflow);
    }
    _ => panic!("It's impossible to handle an overflow"),
  }
}

#[test]
fn its_possible_to_handle_overflows_with_u64() {
  let first = Fraction::<u64>::new_natural(u64::MAX - 1);
  let second = Fraction::<u64>::new_natural(u64::MAX - 1);

  match first.try_add(&second) {
    Err(error) => {
      assert_eq!(error.error_type(), OperationErrorType::Overflow);
    }
    _ => panic!("It's impossible to handle an overflow"),
  }
}

#[test]
fn its_possible_to_handle_overflows_with_u128() {
  let first = Fraction::<u128>::new_natural(u128::MAX - 1);
  let second = Fraction::<u128>::new_natural(u128::MAX - 1);

  match first.try_add(&second) {
    Err(error) => {
      assert_eq!(error.error_type(), OperationErrorType::Overflow);
    }
    _ => panic!("It's impossible to handle an overflow"),
  }
}
