use rfraction::Fraction;

#[test]
fn with_usual_u128_nums_it_works() {
  let first = Fraction::<u128>::new(13, 17, true);
  let second = Fraction::<u128>::new(18, 19, false);

  let result = first / second;

  assert_eq!(result, Fraction::new(247, 306, true));
}

#[test]
fn with_operator_overloads_it_works() {
  let first = Fraction::<u128>::new_natural(13);
  let second = Fraction::<u128>::new_natural(18);

  let mut result = &first / &second;
  assert_eq!(result, Fraction::new(13, 18, false));

  result /= &second;
  assert_eq!(result, Fraction::new(13, 324, false));

  result /= second;
  assert_eq!(result, Fraction::new(13, 5832, false));
}
