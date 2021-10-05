use rfraction::Fraction;
use rfraction::Sign;

#[test]
fn with_usual_u128_nums_it_works() {
  let first = Fraction::<u128>::new(Sign::Negative, 13, 17).unwrap();
  let second = Fraction::<u128>::new(Sign::Positive, 18, 19).unwrap();

  let result = first / second;

  assert_eq!(result, Fraction::new(Sign::Negative, 247, 306).unwrap());
}

#[test]
fn with_operator_overloads_it_works() {
  let first = Fraction::<u128>::new_natural(13);
  let second = Fraction::<u128>::new_natural(18);

  let mut result = &first / &second;
  assert_eq!(result, Fraction::new(Sign::Positive, 13, 18).unwrap());

  result /= &second;
  assert_eq!(result, Fraction::new(Sign::Positive, 13, 324).unwrap());

  result /= second;
  assert_eq!(result, Fraction::new(Sign::Positive, 13, 5832).unwrap());
}
