use rfraction::Fraction;

#[test]
fn with_usual_u128_nums_mul_works() {
  let first = Fraction::<u128>::new(true, 13, 17).unwrap();
  let second = Fraction::<u128>::new(false, 18, 19).unwrap();

  let result = first * second;

  assert!(result.is_negative());
  assert_eq!(result.numerator(), 234);
  assert_eq!(result.denominator(), 323);
}

#[test]
fn with_usual_u64_nums_mul_works() {
  let first = Fraction::<u64>::new(true, 13, 17).unwrap();
  let second = Fraction::<u64>::new(false, 18, 19).unwrap();

  let result = first * second;

  assert!(result.is_negative());
  assert_eq!(result.numerator(), 234);
  assert_eq!(result.denominator(), 323);
}

#[test]
fn with_usual_u32_nums_mul_works() {
  let first = Fraction::<u32>::new(true, 13, 17).unwrap();
  let second = Fraction::<u32>::new(false, 18, 19).unwrap();

  let result = first * second;

  assert!(result.is_negative());
  assert_eq!(result.numerator(), 234);
  assert_eq!(result.denominator(), 323);
}

#[test]
fn with_usual_u16_nums_mul_works() {
  let first = Fraction::<u16>::new(true, 13, 17).unwrap();
  let second = Fraction::<u16>::new(false, 18, 19).unwrap();

  let result = first * second;

  assert!(result.is_negative());
  assert_eq!(result.numerator(), 234);
  assert_eq!(result.denominator(), 323);
}

#[test]
fn with_usual_u8_nums_mul_works() {
  let first = Fraction::<u8>::new(true, 13, 11).unwrap();
  let second = Fraction::<u8>::new(false, 18, 19).unwrap();

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
