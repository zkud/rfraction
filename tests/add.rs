use rfraction::Fraction;

#[test]
fn with_usual_nums_add_works() {
  let first = Fraction::<u128>::new(10, 140, false);
  let second = Fraction::<u128>::new(15, 280, false);

  let result = first.add(&second);

  assert!(result.is_positive());
  assert_eq!(result.numerator(), 1);
  assert_eq!(result.denominator(), 8);
}

#[test]
fn with_nans_nums_add_works() {
  let nan_number = Fraction::<u128>::new_nan();
  let default_number = Fraction::<u128>::new_natural(123);
  let inf = Fraction::<u128>::new_positive_infinity();

  let result = nan_number.add(&nan_number);
  assert!(result.is_nan());

  let result = nan_number.add(&default_number);
  assert!(result.is_nan());

  let result = default_number.add(&nan_number);
  assert!(result.is_nan());

  let result = nan_number.add(&nan_number);
  assert!(result.is_nan());

  let result = inf.add(&nan_number);
  assert!(result.is_nan());
}

#[test]
fn with_infinities_nums_add_works() {
  let default_number = Fraction::<u128>::new_natural(123);
  let positive_inf = Fraction::<u128>::new_positive_infinity();
  let negative_inf = Fraction::<u128>::new_negative_infinity();

  let result = default_number.add(&positive_inf);
  assert!(result.is_positive_infinity());

  let result = default_number.add(&negative_inf);
  assert!(result.is_negative_infinity());

  let result = positive_inf.add(&negative_inf);
  assert!(result.is_nan());

  let result = positive_inf.add(&positive_inf);
  assert!(result.is_positive_infinity());

  let result = negative_inf.add(&negative_inf);
  assert!(result.is_negative_infinity());
}

#[test]
fn with_zeros_nums_add_works() {
  let default_number = Fraction::<u128>::new_natural(123);
  let zero = Fraction::<u128>::new_zero();

  let result = default_number.add(&zero);
  assert_eq!(result.numerator(), 123);
  assert_eq!(result.denominator(), 1);
  assert!(result.is_positive());

  let result = zero.add(&default_number);
  assert_eq!(result.numerator(), 123);
  assert_eq!(result.denominator(), 1);
  assert!(result.is_positive());

  let result = zero.add(&zero);
  assert!(result.is_zero());
}
