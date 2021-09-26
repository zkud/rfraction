use std::fmt;
use std::ops;

pub trait UnsignedNumber:
  fmt::Display
  + Copy
  + Clone
  + PartialEq
  + From<u8>
  + PartialOrd
  + ops::Add<Output = Self>
  + ops::Sub<Output = Self>
  + ops::Mul<Output = Self>
  + ops::Div<Output = Self>
  + ops::Rem<Output = Self>
{
  fn checked_add(self, other: Self) -> Option<Self>;
  fn checked_sub(self, other: Self) -> Option<Self>;
  fn checked_mul(self, other: Self) -> Option<Self>;
  fn checked_div(self, other: Self) -> Option<Self>;
  fn checked_rem(self, other: Self) -> Option<Self>;
}

impl UnsignedNumber for u128 {
  fn checked_add(self, other: Self) -> Option<Self> {
    self.checked_add(other)
  }

  fn checked_mul(self, other: Self) -> Option<Self> {
    self.checked_mul(other)
  }

  fn checked_div(self, other: Self) -> Option<Self> {
    self.checked_div(other)
  }

  fn checked_sub(self, other: Self) -> Option<Self> {
    self.checked_sub(other)
  }

  fn checked_rem(self, other: Self) -> Option<Self> {
    self.checked_rem(other)
  }
}

impl UnsignedNumber for u64 {
  fn checked_add(self, other: Self) -> Option<Self> {
    self.checked_add(other)
  }

  fn checked_mul(self, other: Self) -> Option<Self> {
    self.checked_mul(other)
  }

  fn checked_div(self, other: Self) -> Option<Self> {
    self.checked_div(other)
  }

  fn checked_sub(self, other: Self) -> Option<Self> {
    self.checked_sub(other)
  }

  fn checked_rem(self, other: Self) -> Option<Self> {
    self.checked_rem(other)
  }
}

impl UnsignedNumber for u32 {
  fn checked_add(self, other: Self) -> Option<Self> {
    self.checked_add(other)
  }

  fn checked_mul(self, other: Self) -> Option<Self> {
    self.checked_mul(other)
  }

  fn checked_div(self, other: Self) -> Option<Self> {
    self.checked_div(other)
  }

  fn checked_sub(self, other: Self) -> Option<Self> {
    self.checked_sub(other)
  }

  fn checked_rem(self, other: Self) -> Option<Self> {
    self.checked_rem(other)
  }
}

impl UnsignedNumber for u16 {
  fn checked_add(self, other: Self) -> Option<Self> {
    self.checked_add(other)
  }

  fn checked_mul(self, other: Self) -> Option<Self> {
    self.checked_mul(other)
  }

  fn checked_div(self, other: Self) -> Option<Self> {
    self.checked_div(other)
  }

  fn checked_sub(self, other: Self) -> Option<Self> {
    self.checked_sub(other)
  }

  fn checked_rem(self, other: Self) -> Option<Self> {
    self.checked_rem(other)
  }
}

impl UnsignedNumber for u8 {
  fn checked_add(self, other: Self) -> Option<Self> {
    self.checked_add(other)
  }

  fn checked_mul(self, other: Self) -> Option<Self> {
    self.checked_mul(other)
  }

  fn checked_div(self, other: Self) -> Option<Self> {
    self.checked_div(other)
  }

  fn checked_sub(self, other: Self) -> Option<Self> {
    self.checked_sub(other)
  }

  fn checked_rem(self, other: Self) -> Option<Self> {
    self.checked_rem(other)
  }
}
