use rfraction::Fraction;

#[test]
fn with_usual_numbers_it_works() {
  let number = Fraction::<u128>::new(1, 20, true);

  let abs = number.abs();

  assert_eq!(abs.numerator(), 1);
  assert_eq!(abs.denominator(), 20);
  assert!(abs.is_positive());
}
