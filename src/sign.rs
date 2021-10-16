use std::fmt;

#[derive(fmt::Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Sign {
  #[cfg_attr(feature = "serde", serde(rename = "+"))]
  Positive,
  #[cfg_attr(feature = "serde", serde(rename = "-"))]
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
