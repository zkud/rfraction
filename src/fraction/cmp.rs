use super::super::unsigned_number::UnsignedNumber;
use super::super::Sign;
use super::Fraction;
use std::cmp;
use std::hash;

impl<N: UnsignedNumber> PartialEq for Fraction<N> {
  fn eq(&self, other: &Self) -> bool {
    let (unified_self, unified_other) = self.unify(other).unwrap();

    unified_self.numerator() == unified_other.numerator()
      && unified_self.denominator() == unified_other.denominator()
      && unified_self.sign() == unified_other.sign()
  }
}

impl<N: UnsignedNumber> hash::Hash for Fraction<N> {
  fn hash<H: hash::Hasher>(&self, state: &mut H) {
    let simplified_self = self.clone().simplify();

    simplified_self.numerator().hash(state);
    simplified_self.denominator().hash(state);
    simplified_self.sign().hash(state);
  }
}

impl<N: UnsignedNumber> PartialOrd for Fraction<N> {
  fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
    let (unified_self, unified_other) = self.unify(other).unwrap();

    match (unified_self.sign(), unified_other.sign()) {
      (Sign::Negative, Sign::Positive) => Some(cmp::Ordering::Less),
      (Sign::Positive, Sign::Negative) => Some(cmp::Ordering::Greater),
      _ => {
        if unified_self.is_negative() {
          if unified_self.numerator() > unified_other.numerator() {
            return Some(cmp::Ordering::Less);
          }

          if unified_self.numerator() < unified_other.numerator() {
            return Some(cmp::Ordering::Greater);
          }
        } else {
          if unified_self.numerator() > unified_other.numerator() {
            return Some(cmp::Ordering::Greater);
          }

          if unified_self.numerator() < unified_other.numerator() {
            return Some(cmp::Ordering::Less);
          }
        }

        Some(cmp::Ordering::Equal)
      }
    }
  }
}
