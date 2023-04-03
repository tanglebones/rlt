use num::cast::FromPrimitive;
use num::Num;

pub struct Rect<T> {
  x1: T,
  y1: T,
  x2: T,
  y2: T,
}

impl<T> Rect<T>
where
  T: Num + Copy + PartialOrd + FromPrimitive,
{
  pub fn new(x: T, y: T, w: T, h: T) -> Rect<T> {
    Rect {
      x1: x,
      y1: y,
      x2: x + w,
      y2: y + h,
    }
  }

  // Returns true if self overlaps with other
  pub fn intersect(&self, other: &Rect<T>) -> bool {
    self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
  }

  pub fn center(&self) -> (T, T) {
    let i2 = T::from_i32(2).unwrap();
    ((self.x1 + self.x2) / i2, (self.y1 + self.y2) / i2)
  }

  pub fn iter(&self) -> RectIter<'_, T> {
    RectIter {
      rect: self,
      x: self.x1,
      y: self.y1,
    }
  }
}

pub struct RectIter<'a, T> {
  rect: &'a Rect<T>,
  x: T,
  y: T,
}

impl<'a, T> Iterator for RectIter<'a, T>
where
  T: Num + Copy + PartialOrd + FromPrimitive,
{
  type Item = (T, T);

  fn next(&mut self) -> Option<Self::Item> {
    let i1 = T::from_i32(1).unwrap();

    let r = (self.x, self.y);
    if self.x == self.rect.x2 {
      self.y = self.y + i1;
      self.x = self.rect.x1;
    } else {
      self.x = self.x + i1;
    }
    if r.1 > self.rect.y2 {
      return None;
    }
    Some(r)
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn intersect() {
    let r1 = Rect::new(2, 2, 2, 2);
    for x in 0..=5 {
      for y in 0..=5 {
        let r2 = Rect::new(x, y, 1, 1);
        let actual = r1.intersect(&r2);
        if (x >= 1 && x <= 4) && (y >= 1 && y <= 4) {
          assert!(actual, "{x},{y} expected true");
        } else {
          assert!(!actual, "{x},{y} expected false");
        }
      }
    }
    for x in 0..=5 {
      for y in 0..=5 {
        let r2 = Rect::new(x, y, 2, 2);
        let actual = r1.intersect(&r2);
        if (x >= 0 && x <= 4) && (y >= 0 && y <= 4) {
          assert!(actual, "{x},{y} expected true");
        } else {
          assert!(!actual, "{x},{y} expected false");
        }
      }
    }
  }

  #[test]
  fn center() {
    assert_eq!(Rect::new(0, 0, 1, 1).center(), (0, 0));
    assert_eq!(Rect::new(0, 0, 2, 2).center(), (1, 1));
    assert_eq!(Rect::new(0, 0, 2, 4).center(), (1, 2));
    assert_eq!(
      Rect::new(0i128, 0i128, 1i128, 1i128).center(),
      (0i128, 0i128)
    );
    assert_eq!(Rect::new(0i16, 0i16, 2i16, 2i16).center(), (1i16, 1i16));
    assert_eq!(Rect::new(0u8, 0u8, 2u8, 4u8).center(), (1u8, 2u8));
    assert_eq!(Rect::new(0f32, 0f32, 1f32, 1f32).center(), (0.5, 0.5));
    assert_eq!(Rect::new(0f32, 0f32, 2f32, 2f32).center(), (1.0, 1.0));
    assert_eq!(Rect::new(0f32, 0f32, 2f32, 4f32).center(), (1.0, 2.0));
    assert_eq!(Rect::new(0f64, 0f64, 1f64, 1f64).center(), (0.5f64, 0.5f64));
    assert_eq!(Rect::new(0f64, 0f64, 2f64, 2f64).center(), (1.0f64, 1.0f64));
    assert_eq!(Rect::new(0f64, 0f64, 2f64, 4f64).center(), (1.0f64, 2.0f64));
  }

  #[test]
  fn iter() {
    let r = Rect::new(0, 0, 1, 1);
    let v = r.iter().collect::<Vec<_>>();
    assert_eq!(v, vec![(0, 0), (1, 0), (0, 1), (1, 1)]);
  }
}
