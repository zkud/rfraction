use rfraction::Fraction;

#[test]
fn with_usual_nums_mul_works() {
  let first = Fraction::<u128>::new(13, 17, true);
  let second = Fraction::<u128>::new(18, 19, false);

  let result = first.mul(&second);

  assert!(result.is_negative());
  assert_eq!(result.numerator(), 234);
  assert_eq!(result.denominator(), 323);
}
