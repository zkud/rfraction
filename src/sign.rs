use std::fmt;

#[derive(fmt::Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Sign {
  Positive,
  Negative,
}

impl Sign {
  pub fn inverse(&self) -> Self {
    match self {
      Sign::Positive => Sign::Negative,
      _ => Sign::Positive,
    }
  }

  pub fn mul(&self, other: &Self) -> Self {
    match (self, other) {
      (Sign::Negative, Sign::Negative) => Sign::Positive,
      (Sign::Positive, Sign::Positive) => Sign::Positive,
      _ => Sign::Negative,
    }
  }
}

impl fmt::Display for Sign {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Sign::Positive => write!(formatter, "+"),
      Sign::Negative => write!(formatter, "-"),
    }
  }
}
