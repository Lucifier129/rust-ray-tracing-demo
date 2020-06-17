use crate::utils::clamp;
use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Vec3(pub f64, pub f64, pub f64);

// Vec3 + Vec3
impl ops::Add<Vec3> for Vec3 {
  type Output = Vec3;

  fn add(self, rhs: Vec3) -> Vec3 {
    Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
  }
}

// Vec3 - Vec3
impl ops::Sub<Vec3> for Vec3 {
  type Output = Vec3;

  fn sub(self, rhs: Vec3) -> Vec3 {
    Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
  }
}

// Vec3 * Vec3
impl ops::Mul<Vec3> for Vec3 {
  type Output = Vec3;

  fn mul(self, rhs: Vec3) -> Vec3 {
    Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
  }
}

// Vec3 * f64
impl ops::Mul<f64> for Vec3 {
  type Output = Vec3;

  fn mul(self, rhs: f64) -> Vec3 {
    Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
  }
}

// f64 * Vec3
impl ops::Mul<Vec3> for f64 {
  type Output = Vec3;

  fn mul(self, rhs: Vec3) -> Vec3 {
    rhs * self
  }
}

// Vec3 / f64
impl ops::Div<f64> for Vec3 {
  type Output = Vec3;

  fn div(self, rhs: f64) -> Vec3 {
    (1.0 / rhs) * self
  }
}

// -Vec3
impl ops::Neg for Vec3 {
  type Output = Vec3;
  fn neg(self) -> Vec3 {
    -1.0 * self
  }
}

impl Vec3 {
  pub fn x(&self) -> f64 {
    self.0
  }

  pub fn y(&self) -> f64 {
    self.1
  }

  pub fn z(&self) -> f64 {
    self.2
  }

  pub fn r(&self) -> f64 {
    self.0
  }

  pub fn g(&self) -> f64 {
    self.1
  }

  pub fn b(&self) -> f64 {
    self.2
  }

  pub fn len(&self) -> f64 {
    self.length_squared().sqrt()
  }

  pub fn length_squared(&self) -> f64 {
    self.0.powi(2) + self.1.powi(2) + self.2.powi(2)
  }

  pub fn to_rgb_string(&self) -> String {
    let Vec3(r, g, b) = *self;
    let ir = (256.0 * clamp(r, 0.0, 0.999)) as u32;
    let ig = (256.0 * clamp(g, 0.0, 0.999)) as u32;
    let ib = (256.0 * clamp(b, 0.0, 0.999)) as u32;

    format!("{} {} {}\n", ir, ig, ib)
  }

  pub fn dot(&self, v: Vec3) -> f64 {
    let u = self;
    u.0 * v.0 + u.1 * v.1 + u.2 * v.2
  }

  pub fn cross(&self, v: Vec3) -> Vec3 {
    let u = self;
    Vec3(
      u.1 * v.2 - u.2 * v.1,
      u.2 * v.0 - u.0 * v.2,
      u.0 * v.1 - u.1 * v.0,
    )
  }

  pub fn unit_vector(&self) -> Vec3 {
    *self / self.len()
  }

  pub fn eq(&self, b: Vec3) -> bool {
    let a = self;
    a.0 == b.0 && a.1 == b.1 && a.2 == b.2
  }

  pub fn lerp(t: f64, start: Vec3, end: Vec3) -> Vec3 {
    (1.0 - t) * start + (t * end)
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  fn assert_approx_eq(a: f64, b: f64) {
    let n = 1.0e-6;
    assert!(
      (a - b).abs() < n,
      "assertion failed: `(left !== right)` \
      (left: `{:?}`, right: `{:?}`, expect diff: `{:?}`, real diff: `{:?}`)",
      a,
      b,
      n,
      (a - b).abs()
    )
  }

  #[test]
  fn test_vec3_methods() {
    let a = Vec3(0.1, 0.2, 0.3);

    assert_eq!(a.x(), 0.1);
    assert_eq!(a.y(), 0.2);
    assert_eq!(a.z(), 0.3);

    assert_eq!(a.r(), 0.1);
    assert_eq!(a.g(), 0.2);
    assert_eq!(a.b(), 0.3);

    assert_eq!(&a.len().to_string()[0..4], "1.85");
  }

  #[test]
  fn test_vec3_operators() {
    let a = Vec3(0.1, 0.2, 0.3);
    let b = Vec3(0.3, 0.2, 0.1);
    let c = a + b;
    let d = a - b;
    let e = a * b;
    let f = a * 2.0;
    let g = 2.0 * a;
    let h = a / 10.0;
    let i = -a;

    assert_approx_eq(c.x(), 0.4);
    assert_approx_eq(c.y(), 0.4);
    assert_approx_eq(c.z(), 0.4);

    assert_approx_eq(d.x(), -0.2);
    assert_approx_eq(d.y(), -0.0);
    assert_approx_eq(d.z(), 0.2);

    assert_approx_eq(e.x(), 0.03);
    assert_approx_eq(e.y(), 0.04);
    assert_approx_eq(e.z(), 0.03);

    assert_approx_eq(f.x(), 0.2);
    assert_approx_eq(f.y(), 0.4);
    assert_approx_eq(f.z(), 0.6);

    assert_approx_eq(g.x(), 0.2);
    assert_approx_eq(g.y(), 0.4);
    assert_approx_eq(g.z(), 0.6);

    assert_approx_eq(h.x(), 0.01);
    assert_approx_eq(h.y(), 0.02);
    assert_approx_eq(h.z(), 0.03);

    assert_approx_eq(i.x(), -0.1);
    assert_approx_eq(i.y(), -0.2);
    assert_approx_eq(i.z(), -0.3);
  }
}
