use rfraction::Fraction;
use rfraction::Sign;
use serde_json::{from_str, to_string};
use std::convert::TryFrom;

#[test]
fn it_convertable_to_f32_type() {
  let num = Fraction::<u128>::new(Sign::Positive, 1, 17);

  assert!((num.to_number::<f32>() - 0.058823529) < f32::EPSILON);
}

#[test]
fn it_convertable_to_f64_type() {
  let num = Fraction::<u128>::new(Sign::Positive, 1, 17);

  assert!((num.to_number::<f64>() - 0.058823529411764705) < f64::EPSILON);
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

#[test]
fn it_convertable_to_decimal() {
  let number = Fraction::<u128>::new(Sign::Positive, 4, 3);
  assert_eq!(
    number.to_decimal(5),
    Fraction::<u128>::new(Sign::Positive, 133333, 100000)
  );

  let number = Fraction::<u128>::new(Sign::Negative, 1, 6);
  assert_eq!(
    number.to_decimal(5),
    Fraction::<u128>::new(Sign::Negative, 16666, 100000)
  );
}

#[test]
fn it_supports_serde() {
  let number = Fraction::<u128>::new(Sign::Negative, 4, 3);
  let serialized_number = to_string(&number).unwrap();
  let number: Fraction<u128> = from_str(&serialized_number).unwrap();
  assert_eq!(number, Fraction::<u128>::new(Sign::Negative, 4, 3));

  let number = Fraction::<u128>::new_natural(14);
  let serialized_number = to_string(&number).unwrap();
  let number: Fraction<u128> = from_str(&serialized_number).unwrap();
  assert_eq!(number, Fraction::<u128>::new_natural(14));
}
