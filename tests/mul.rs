use rfraction::Fraction;

#[test]
fn with_usual_u128_nums_mul_works() {
  let first = Fraction::<u128>::new(13, 17, true);
  let second = Fraction::<u128>::new(18, 19, false);

  let result = first.mul(&second);

  assert!(result.is_negative());
  assert_eq!(result.numerator(), 234);
  assert_eq!(result.denominator(), 323);
}

#[test]
fn with_usual_u64_nums_mul_works() {
  let first = Fraction::<u64>::new(13, 17, true);
  let second = Fraction::<u64>::new(18, 19, false);

  let result = first.mul(&second);

  assert!(result.is_negative());
  assert_eq!(result.numerator(), 234);
  assert_eq!(result.denominator(), 323);
}

#[test]
fn with_usual_u32_nums_mul_works() {
  let first = Fraction::<u32>::new(13, 17, true);
  let second = Fraction::<u32>::new(18, 19, false);

  let result = first.mul(&second);

  assert!(result.is_negative());
  assert_eq!(result.numerator(), 234);
  assert_eq!(result.denominator(), 323);
}

#[test]
fn with_usual_u16_nums_mul_works() {
  let first = Fraction::<u16>::new(13, 17, true);
  let second = Fraction::<u16>::new(18, 19, false);

  let result = first.mul(&second);

  assert!(result.is_negative());
  assert_eq!(result.numerator(), 234);
  assert_eq!(result.denominator(), 323);
}

#[test]
fn with_usual_u8_nums_mul_works() {
  let first = Fraction::<u16>::new(13, 11, true);
  let second = Fraction::<u16>::new(18, 19, false);

  let result = first.mul(&second);

  assert!(result.is_negative());
  assert_eq!(result.numerator(), 234);
  assert_eq!(result.denominator(), 209);
}

#[test]
fn with_nans_nums_mul_works() {
  let nan_number = Fraction::<u128>::new_nan();
  let default_number = Fraction::<u128>::new_natural(123);
  let inf = Fraction::<u128>::new_positive_infinity();

  let result = nan_number.mul(&nan_number);
  assert!(result.is_nan());

  let result = nan_number.mul(&default_number);
  assert!(result.is_nan());

  let result = default_number.mul(&nan_number);
  assert!(result.is_nan());

  let result = nan_number.mul(&nan_number);
  assert!(result.is_nan());

  let result = inf.mul(&nan_number);
  assert!(result.is_nan());
}

#[test]
fn with_infinities_nums_mul_works() {
  let default_number = Fraction::<u128>::new_natural(123);
  let positive_inf = Fraction::<u128>::new_positive_infinity();
  let negative_inf = Fraction::<u128>::new_negative_infinity();

  let result = default_number.mul(&positive_inf);
  assert!(result.is_positive_infinity());

  let result = default_number.mul(&negative_inf);
  assert!(result.is_negative_infinity());

  let result = positive_inf.mul(&negative_inf);
  assert!(result.is_negative_infinity());

  let result = positive_inf.mul(&positive_inf);
  assert!(result.is_positive_infinity());

  let result = negative_inf.mul(&negative_inf);
  assert!(result.is_positive_infinity());
}

#[test]
fn with_zeros_nums_mul_works() {
  let default_number = Fraction::<u128>::new_natural(123);
  let zero = Fraction::<u128>::new_zero();
  let infinity = Fraction::<u128>::new_positive_infinity();

  let result = default_number.mul(&zero);
  assert!(result.is_zero());

  let result = zero.mul(&default_number);
  assert!(result.is_zero());

  let result = zero.mul(&zero);
  assert!(result.is_zero());

  let result = zero.mul(&infinity);
  assert!(result.is_nan());
}

#[test]
fn with_overflowing_nums_mul_works() {
  let first = Fraction::<u8>::new(150, 1, false);
  let second = Fraction::<u8>::new(150, 1, false);

  let result = first.mul(&second);

  assert!(result.is_nan());

  let first = Fraction::<u8>::new(1, 150, false);
  let second = Fraction::<u8>::new(1, 13, false);

  let result = first.mul(&second);

  assert!(result.is_nan());
}
