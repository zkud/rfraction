use rfraction::Fraction;
use rfraction::OperationErrorType;

#[test]
fn with_usual_u128_nums_add_works() {
  let first = Fraction::<u128>::new(false, 10, 140).unwrap();
  let second = Fraction::<u128>::new(false, 15, 280).unwrap();

  let result = first + second;

  assert!(result.is_positive());
  assert_eq!(result.numerator(), 1);
  assert_eq!(result.denominator(), 8);
}

#[test]
fn with_usual_u64_nums_add_works() {
  let first = Fraction::<u64>::new(false, 10, 140).unwrap();
  let second = Fraction::<u64>::new(false, 15, 280).unwrap();

  let result = first + second;

  assert!(result.is_positive());
  assert_eq!(result.numerator(), 1);
  assert_eq!(result.denominator(), 8);
}

#[test]
fn with_usual_u32_nums_add_works() {
  let first = Fraction::<u32>::new(false, 10, 140).unwrap();
  let second = Fraction::<u32>::new(false, 15, 280).unwrap();

  let result = first + second;

  assert!(result.is_positive());
  assert_eq!(result.numerator(), 1);
  assert_eq!(result.denominator(), 8);
}

#[test]
fn with_usual_u16_nums_add_works() {
  let first = Fraction::<u16>::new(false, 10, 140).unwrap();
  let second = Fraction::<u16>::new(false, 15, 280).unwrap();

  let result = first + second;

  assert!(result.is_positive());
  assert_eq!(result.numerator(), 1);
  assert_eq!(result.denominator(), 8);
}

#[test]
fn with_usual_u8_nums_add_works() {
  let first = Fraction::<u8>::new(false, 20, 20).unwrap();
  let second = Fraction::<u8>::new(false, 15, 10).unwrap();

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
fn its_possible_to_handle_overflows() {
  let first = Fraction::<u8>::new_natural(150);
  let second = Fraction::<u8>::new_natural(150);

  match first.try_add(&second) {
    Err(error) if error.error_type() == OperationErrorType::Overflow => {
      assert_eq!(error.message(), "Failed to add 150 and 150");
    }
    _ => panic!("It's impossible to handle an overflow"),
  }
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
