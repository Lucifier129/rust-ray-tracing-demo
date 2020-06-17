use std::fmt::Debug;

use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Material: Debug {
  fn scatter(
    &mut self,
    ray_in: &Ray,
    record: &mut HitRecord,
    attenuation: &mut Vec3,
    scattered: &mut Ray,
  ) -> bool;
  fn box_clone(&self) -> Box<dyn Material>;
}

#[derive(Debug)]
pub struct DefaultMaterial {}

impl DefaultMaterial {
  pub fn new() -> DefaultMaterial {
    DefaultMaterial {}
  }
}

impl Material for DefaultMaterial {
  fn scatter(
    &mut self,
    _ray_in: &Ray,
    _record: &mut HitRecord,
    _attenuation: &mut Vec3,
    _scattered: &mut Ray,
  ) -> bool {
    false
  }
  fn box_clone(&self) -> Box<dyn Material> {
    Box::new(DefaultMaterial::new())
  }
}

#[derive(Debug)]
pub struct Lambertian {
  albedo: Vec3,
}

impl Lambertian {
  pub fn new(albedo: Vec3) -> Lambertian {
    Lambertian { albedo }
  }
}

impl Material for Lambertian {
  fn scatter(
    &mut self,
    _ray_in: &Ray,
    record: &mut HitRecord,
    attenuation: &mut Vec3,
    scattered: &mut Ray,
  ) -> bool {
    let scatter_direction = record.normal + Vec3::random_unit_vector();
    *scattered = Ray::new(record.point, scatter_direction);
    *attenuation = self.albedo;
    true
  }
  fn box_clone(&self) -> Box<dyn Material> {
    Box::new(Lambertian::new(self.albedo))
  }
}

#[derive(Debug)]
pub struct Metal {
  albedo: Vec3,
}

impl Metal {
  pub fn new(albedo: Vec3) -> Metal {
    Metal { albedo }
  }
}

impl Material for Metal {
  fn scatter(
    &mut self,
    ray_in: &Ray,
    record: &mut HitRecord,
    attenuation: &mut Vec3,
    scattered: &mut Ray,
  ) -> bool {
    let reflected = Vec3::reflect(ray_in.direction.unit_vector(), record.normal);
    *scattered = Ray::new(record.point, reflected);
    *attenuation = self.albedo;
    scattered.direction.dot(record.normal) > 0.0
  }
  fn box_clone(&self) -> Box<dyn Material> {
    Box::new(Metal::new(self.albedo))
  }
}
