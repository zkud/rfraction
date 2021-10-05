use rfraction::Fraction;
use rfraction::Sign;

#[test]
fn with_usual_nums_sub_works() {
  let first = Fraction::<u128>::new(Sign::Positive, 10, 140).unwrap();
  let second = Fraction::<u128>::new(Sign::Positive, 15, 280).unwrap();

  let result = first - second;

  assert!(result.is_positive());
  assert_eq!(result.numerator(), 1);
  assert_eq!(result.denominator(), 56);
}

#[test]
fn with_operator_overloads_it_works() {
  let first = Fraction::<u128>::new_natural(150);
  let second = Fraction::<u128>::new_natural(150);

  let mut result = &first - &second;
  assert_eq!(result, Fraction::new_zero());

  result -= &second;
  assert_eq!(result, Fraction::new(Sign::Negative, 150, 1).unwrap());

  result -= second;
  assert_eq!(result, Fraction::new(Sign::Negative, 300, 1).unwrap());
}
