use rfraction::Fraction;

#[test]
fn with_usual_nums_sub_works() {
  let first = Fraction::<u128>::new(10, 140, false);
  let second = Fraction::<u128>::new(15, 280, false);

  let result = first - second;

  assert!(result.is_positive());
  assert_eq!(result.numerator(), 1);
  assert_eq!(result.denominator(), 56);
}

#[test]
fn with_operator_overloads_it_works() {
  let first = Fraction::<u128>::new_natural(150);
  let second = Fraction::<u128>::new_natural(150);

  let mut result = &first - &second;
  assert_eq!(result, Fraction::new_zero());

  result -= &second;
  assert_eq!(result, Fraction::new(150, 1, true));

  result -= second;
  assert_eq!(result, Fraction::new(300, 1, true));
}