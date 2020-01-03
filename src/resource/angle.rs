use std::cmp;
use std::cmp::Ordering;
use std::ops;

#[derive(Debug)]
pub struct Angle(f32);

impl Angle {
  pub fn new(angle: f32) -> Self {
    let mut angle = Angle(angle);
    angle.normalize();
    angle
  }
  pub fn normalize(&mut self) {
    self.0 %= 360.0;
    if self.0 < 0.0 {
      self.0 += 360.0
    }
  }

  pub fn update(&mut self, float: f32) {
    self.0 = float;
    self.normalize();
  }
}

impl ops::Add for Angle {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    Self::new(self.0 + rhs.0)
  }
}

impl ops::AddAssign for Angle {
  fn add_assign(&mut self, rhs: Self) {
    *self = Self::new(self.0 + rhs.0)
  }
}

impl ops::Sub for Angle {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self::Output {
    Self::new(self.0 - rhs.0)
  }
}

impl ops::SubAssign for Angle {
  fn sub_assign(&mut self, rhs: Self) {
    *self = Self::new(self.0 - rhs.0)
  }
}

impl ops::Neg for Angle {
  type Output = Self;

  fn neg(self) -> Self::Output {
    Self::new(360.0 - self.0)
  }
}

impl cmp::PartialEq for Angle {
  fn eq(&self, other: &Self) -> bool {
    (self.0 - other.0).abs() < std::f32::EPSILON
  }
}

impl cmp::PartialOrd for Angle {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.0.partial_cmp(&other.0)
  }

  fn lt(&self, other: &Self) -> bool {
    self.0 < other.0
  }

  fn le(&self, other: &Self) -> bool {
    self.0 <= other.0
  }

  fn gt(&self, other: &Self) -> bool {
    self.0 > other.0
  }

  fn ge(&self, other: &Self) -> bool {
    self.0 >= other.0
  }
}

impl cmp::PartialEq<f32> for Angle {
  fn eq(&self, other: &f32) -> bool {
    (self.0 - *other).abs() < std::f32::EPSILON
  }
}

impl cmp::PartialOrd<f32> for Angle {
  fn partial_cmp(&self, other: &f32) -> Option<Ordering> {
    self.0.partial_cmp(&other)
  }

  fn lt(&self, other: &f32) -> bool {
    self.0 < *other
  }

  fn le(&self, other: &f32) -> bool {
    self.0 <= *other
  }

  fn gt(&self, other: &f32) -> bool {
    self.0 > *other
  }

  fn ge(&self, other: &f32) -> bool {
    self.0 >= *other
  }
}

#[cfg(test)]
mod tests {
  use crate::resource::angle::Angle;

  #[test]
  fn overload_add() {
    assert_eq!(Angle::new(11.2) + Angle::new(0.8), 12.0);
  }

  #[test]
  fn overload_add_assign() {
    let mut angle = Angle::new(350.0);
    angle += Angle::new(11.2) + Angle::new(0.8);
    assert_eq!(angle, 2.0);
  }

  #[test]
  fn overload_sub() {
    assert_eq!(Angle::new(5.1) - Angle::new(0.1), 5.0);
  }

  #[test]
  fn overload_sub_assign() {
    let mut angle = Angle::new(-2.0);
    angle -= Angle::new(11.2) - Angle::new(0.8);
    assert_eq!(angle, 347.6);
  }

  #[test]
  fn overload_neg() {
    let mut angle = Angle::new(45.0);
    assert_eq!(-angle, 315.0);
  }

  #[test]
  fn overload_partial_eq() {
    let angle_a = Angle::new(10.1);
    let angle_b = Angle::new(10.1);
    assert_eq!(angle_a == angle_b, true);
  }

  #[test]
  fn overload_partial_ord() {
    let angle_a = Angle::new(10.1);
    let angle_b = Angle::new(10.2);
    assert!(angle_a < angle_b, true);
    assert!(angle_a <= angle_b, true);
    assert!(angle_b > angle_a, true);
    assert!(angle_b >= angle_a, true);
  }
}
