use rfraction::Fraction;
use rfraction::Sign;

#[test]
fn with_usual_numbers_it_works() {
  let number = Fraction::<u128>::new(Sign::Negative, 1, 20);
  assert_eq!(number.is_positive(), false);
  assert_eq!(number.is_negative(), true);
  assert_eq!(number.is_natural(), false);
  assert_eq!(number.is_zero(), false);

  let natural = Fraction::<u128>::new_natural(20);
  assert_eq!(natural.is_positive(), true);
  assert_eq!(natural.is_negative(), false);
  assert_eq!(natural.is_natural(), true);
  assert_eq!(natural.is_zero(), false);

  let zero = Fraction::<u128>::new_zero();
  assert_eq!(zero.is_positive(), false);
  assert_eq!(zero.is_negative(), false);
  assert_eq!(zero.is_natural(), false);
  assert_eq!(zero.is_zero(), true);
}
