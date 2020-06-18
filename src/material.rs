use std::fmt::Debug;

use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::utils::random;
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
  pub albedo: Vec3,
  pub fuzz: f64,
}

impl Metal {
  pub fn new(albedo: Vec3) -> Metal {
    Metal { albedo, fuzz: 0.0 }
  }
  pub fn set_fuzz(&self, fuzz: f64) -> Self {
    let fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };
    Metal {
      albedo: self.albedo,
      fuzz,
    }
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
    let reflected = reflected + self.fuzz * Vec3::random_unit_vector();

    *scattered = Ray::new(record.point, reflected);
    *attenuation = self.albedo;
    scattered.direction.dot(record.normal) > 0.0
  }
  fn box_clone(&self) -> Box<dyn Material> {
    Box::new(Metal::new(self.albedo).set_fuzz(self.fuzz))
  }
}

#[derive(Debug)]
pub struct Dielectric {
  refract_index: f64,
}

impl Dielectric {
  pub fn new(refract_index: f64) -> Dielectric {
    Dielectric { refract_index }
  }
}

impl Material for Dielectric {
  fn scatter(
    &mut self,
    ray_in: &Ray,
    record: &mut HitRecord,
    attenuation: &mut Vec3,
    scattered: &mut Ray,
  ) -> bool {
    let etai_over_etat = if record.front_face {
      1.0 / self.refract_index
    } else {
      self.refract_index
    };

    let unit_direction = ray_in.direction.unit_vector();
    let cos_theta = (-unit_direction).dot(record.normal).min(1.0);
    let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

    let reflect = || {
      let reflected = Vec3::reflect(unit_direction, record.normal);
      Ray::new(record.point, reflected)
    };

    let refract = || {
      let refracted = Vec3::refract(unit_direction, record.normal, etai_over_etat);
      Ray::new(record.point, refracted)
    };

    *attenuation = Vec3::fill(1.0);

    if etai_over_etat * sin_theta > 1.0 {
      *scattered = reflect();
    } else {
      let reflect_prop = schlick(cos_theta, etai_over_etat);

      if random() < reflect_prop {
        *scattered = reflect();
      } else {
        *scattered = refract();
      }
    }
    true
  }
  fn box_clone(&self) -> Box<dyn Material> {
    Box::new(Dielectric::new(self.refract_index))
  }
}

fn schlick(cosine: f64, refract_index: f64) -> f64 {
  let r0 = (1.0 - refract_index) / (1.0 + refract_index);
  let r0 = r0.powi(2);
  r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
