use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug, Clone)]
pub struct HitRecord {
  pub point: Vec3,
  pub normal: Vec3,
  pub t: f64,
  pub front_face: bool,
}

impl HitRecord {
  pub fn new() -> HitRecord {
    HitRecord {
      point: Vec3(0.0, 0.0, 0.0),
      normal: Vec3(0.0, 0.0, 0.0),
      t: 0.0,
      front_face: false,
    }
  }
  pub fn set_point(&mut self, point: Vec3) {
    self.point = point;
  }
  pub fn set_t(&mut self, t: f64) {
    self.t = t;
  }
  pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
    let front_face = ray.direction.dot(outward_normal) < 0.0;

    self.front_face = front_face;

    self.normal = if front_face {
      outward_normal
    } else {
      -outward_normal
    }
  }
}

pub trait Hittable {
  fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool;
}
