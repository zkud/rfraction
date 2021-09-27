use rfraction::Fraction;

#[test]
fn with_usual_u128_nums_mul_works() {
  let first = Fraction::<u128>::new(13, 17, true);
  let second = Fraction::<u128>::new(18, 19, false);

  let result = first / second;

  assert_eq!(result, Fraction::new(247, 306, true));
}
