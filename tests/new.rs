use rfraction::Fraction;
use rfraction::Sign;

#[test]
fn new_works() {
  let number = Fraction::<u128>::new(Sign::Positive, 123, 321).unwrap();
  assert_eq!(number.numerator(), 41);
  assert_eq!(number.denominator(), 107);
  assert!(number.is_positive());

  let number = Fraction::<u128>::new(Sign::Negative, 777, 888).unwrap();
  assert_eq!(number.numerator(), 7);
  assert_eq!(number.denominator(), 8);
  assert!(number.is_negative());
}
