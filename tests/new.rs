use rfraction::Fraction;

#[test]
fn new_works() {
  let number = Fraction::<u128>::new(false, 123, 321).unwrap();
  assert_eq!(number.numerator(), 41);
  assert_eq!(number.denominator(), 107);
  assert!(number.is_positive());

  let number = Fraction::<u128>::new(true, 777, 888).unwrap();
  assert_eq!(number.numerator(), 7);
  assert_eq!(number.denominator(), 8);
  assert!(number.is_negative());
}
