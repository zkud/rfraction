use rfraction::Fraction;

#[test]
fn with_usual_numbers_it_works() {
  let number = Fraction::<u128>::new(1, 20, true);

  let abs = number.reverse();

  assert_eq!(abs.numerator(), 20);
  assert_eq!(abs.denominator(), 1);
  assert!(abs.is_negative());
}