use rfraction::Fraction;
use rfraction::Sign;

#[test]
fn with_different_signes_it_compares() {
  let negative = Fraction::<u32>::new(Sign::Negative, 1, 3);
  let positive = Fraction::<u32>::new(Sign::Positive, 1, 17);

  assert!(positive > negative);
  assert!(positive != negative);
  assert!(positive >= negative);
  assert!(positive == positive);
  assert!(negative == negative);
}

#[test]
fn with_negative_numbers_it_compares() {
  let negative1 = Fraction::<u32>::new(Sign::Negative, 1, 3);
  let negative2 = Fraction::<u32>::new(Sign::Negative, 1, 17);

  assert!(negative2 > negative1);
  assert!(negative2 != negative1);
  assert!(negative2 >= negative1);
  assert!(negative2 == negative2);
  assert!(negative2 == negative2);
}

#[test]
fn with_positive_numbers_it_compares() {
  let positive1 = Fraction::<u32>::new(Sign::Positive, 1, 3);
  let positive2 = Fraction::<u32>::new(Sign::Positive, 1, 17);

  assert!(positive2 < positive1);
  assert!(positive2 != positive1);
  assert!(positive2 <= positive1);
  assert!(positive2 == positive2);
  assert!(positive1 == positive1);
}

#[test]
fn it_is_displayable() {
  let positive = Fraction::<u32>::new(Sign::Positive, 10, 3);
  let negative = Fraction::<u32>::new(Sign::Negative, 3, 10);
  let zero = Fraction::<u32>::new_zero();

  assert_eq!(String::from("+10/3"), format!("{}", positive));
  assert_eq!(String::from("-3/10"), format!("{}", negative));
  assert_eq!(String::from("0"), format!("{}", zero));

  let simple_ratio = Fraction::<u32>::new(Sign::Positive, 1, 3);
  assert_eq!(String::from("0.33333"), format!("{:.5}", simple_ratio));
  assert_eq!(
    String::from("-0.33333"),
    format!("{:.5}", simple_ratio.neg())
  );
}

#[test]
fn it_is_debuggable() {
  let positive = Fraction::<u32>::new(Sign::Positive, 10, 3);
  let negative = Fraction::<u32>::new(Sign::Negative, 3, 10);
  let zero = Fraction::<u32>::new_zero();

  assert_eq!(
    String::from("Fraction { sign: Positive, numerator: 10, denominator: 3 }"),
    format!("{:?}", positive)
  );
  assert_eq!(
    String::from("Fraction { sign: Negative, numerator: 3, denominator: 10 }"),
    format!("{:?}", negative)
  );
  assert_eq!(
    String::from("Fraction { sign: Positive, numerator: 0, denominator: 1 }"),
    format!("{:?}", zero)
  );
}

#[test]
fn it_could_give_a_default_value() {
  let default_value: Fraction<u128> = Default::default();
  assert!(default_value.is_zero());
}
