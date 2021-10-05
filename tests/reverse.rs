use rfraction::Fraction;
use rfraction::Sign;

#[test]
fn with_usual_numbers_it_works() {
  let number = Fraction::<u128>::new(Sign::Negative, 1, 20).unwrap();

  let abs = number.reverse();

  assert_eq!(abs.numerator(), 20);
  assert_eq!(abs.denominator(), 1);
  assert!(abs.is_negative());
}
