use rfraction::Fraction;

#[test]
fn new_works() {
  let number = Fraction::new(123, 321, false);
  assert_eq!(number.numerator(), 123);
  assert_eq!(number.denominator(), 321);
  assert!(number.is_positive());

  let number = Fraction::new(777, 888, true);
  assert_eq!(number.numerator(), 777);
  assert_eq!(number.denominator(), 888);
  assert!(number.is_negative());
}
