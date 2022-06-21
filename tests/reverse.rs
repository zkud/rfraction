use rfraction::Fraction;
use rfraction::Sign;

#[test]
fn with_usual_numbers_it_works() {
  let number = Fraction::<u128>::new(Sign::Negative, 1, 20);

  let reversed = number.reverse();

  assert_eq!(reversed.numerator(), 20);
  assert_eq!(reversed.denominator(), 1);
  assert!(reversed.is_negative());
}
