pub struct Clamper<T>(T, T);

impl<T> Clamper<T>
where
  T: PartialOrd + Copy,
{
  pub fn clamp(&self, v: T) -> T {
    if v < self.0 {
      self.0
    } else if v > self.1 {
      self.1
    } else {
      v
    }
  }

  pub const fn new(min: T, max: T) -> Clamper<T> {
    Self(min, max)
  }
}
