use rfraction::Fraction;

#[test]
fn new_works() {
  let number = Fraction::<u128>::new(123, 321, false);
  assert_eq!(number, Fraction::new(41, 107, false));

  let number = Fraction::<u128>::new(777, 888, true);
  assert_eq!(number, Fraction::new(777, 888, true));
}
