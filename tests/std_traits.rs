use rfraction::Fraction;
use rfraction::Sign;
use std::convert::TryFrom;

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

#[test]
fn it_convertable_from_the_origin_type() {
  let number = Fraction::<u128>::from(123);
  assert_eq!(number, Fraction::<u128>::new_natural(123));

  let number = Fraction::<u128>::from(&123);
  assert_eq!(number, Fraction::<u128>::new_natural(123));
}

#[test]
fn it_convertable_from_the_f32_type() {
  let number = Fraction::<u128>::try_from(3.14f32).unwrap();
  assert_eq!(number, Fraction::<u128>::new(Sign::Positive, 314, 100));
}

#[test]
fn it_convertable_from_the_f64_type() {
  let number = Fraction::<u128>::try_from(3.1438f64).unwrap();
  assert_eq!(number, Fraction::<u128>::new(Sign::Positive, 31438, 10000));
}

#[test]
fn it_convertable_from_the_string_type() {
  let number = Fraction::<u128>::try_from("3.1438").unwrap();
  assert_eq!(number, Fraction::<u128>::new(Sign::Positive, 31438, 10000));

  let number = Fraction::<u128>::try_from("-3.1438").unwrap();
  assert_eq!(number, Fraction::<u128>::new(Sign::Negative, 31438, 10000));

  let number = Fraction::<u128>::try_from("-3").unwrap();
  assert_eq!(number, Fraction::<u128>::new(Sign::Negative, 3, 1));

  let number = Fraction::<u128>::try_from("+3").unwrap();
  assert_eq!(number, Fraction::<u128>::new_natural(3));

  let number = Fraction::<u128>::try_from("3").unwrap();
  assert_eq!(number, Fraction::<u128>::new_natural(3));

  let number = Fraction::<u128>::try_from("1e-5").unwrap();
  assert_eq!(number, Fraction::<u128>::new(Sign::Positive, 1, 100000));
}
