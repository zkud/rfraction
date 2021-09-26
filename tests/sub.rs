use rfraction::Fraction;

#[test]
fn with_usual_nums_sub_works() {
  let first = Fraction::<u128>::new(10, 140, false);
  let second = Fraction::<u128>::new(15, 280, false);

  let result = first.sub(&second);

  assert!(result.is_positive());
  assert_eq!(result.numerator(), 1);
  assert_eq!(result.denominator(), 56);
}
