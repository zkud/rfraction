pub trait ConvertableTo<T> {
  fn convert_to(&self) -> T;
}

impl ConvertableTo<f32> for u8 {
  fn convert_to(&self) -> f32 {
    *self as f32
  }
}

impl ConvertableTo<f64> for u8 {
  fn convert_to(&self) -> f64 {
    *self as f64
  }
}

impl ConvertableTo<f32> for u16 {
  fn convert_to(&self) -> f32 {
    *self as f32
  }
}

impl ConvertableTo<f64> for u16 {
  fn convert_to(&self) -> f64 {
    *self as f64
  }
}

impl ConvertableTo<f32> for u32 {
  fn convert_to(&self) -> f32 {
    *self as f32
  }
}

impl ConvertableTo<f64> for u32 {
  fn convert_to(&self) -> f64 {
    *self as f64
  }
}

impl ConvertableTo<f32> for u64 {
  fn convert_to(&self) -> f32 {
    *self as f32
  }
}

impl ConvertableTo<f64> for u64 {
  fn convert_to(&self) -> f64 {
    *self as f64
  }
}

impl ConvertableTo<f32> for u128 {
  fn convert_to(&self) -> f32 {
    *self as f32
  }
}

impl ConvertableTo<f64> for u128 {
  fn convert_to(&self) -> f64 {
    *self as f64
  }
}
