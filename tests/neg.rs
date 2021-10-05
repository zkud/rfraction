use rfraction::Fraction;
use rfraction::Sign;

#[test]
fn with_usual_numbers_it_works() {
  let number = Fraction::<u128>::new(Sign::Negative, 1, 20).unwrap();

  let abs = number.neg();

  assert_eq!(abs.numerator(), 1);
  assert_eq!(abs.denominator(), 20);
  assert!(abs.is_positive());
}
