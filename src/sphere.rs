use crate::hittable::{HitRecord, Hittable};
use crate::material::{DefaultMaterial, Material};
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct Sphere {
  pub center: Vec3,
  pub radius: f64,
  pub material: Box<dyn Material>,
}

impl Sphere {
  pub fn new(center: Vec3, radius: f64) -> Sphere {
    let material = Box::new(DefaultMaterial::new());
    Sphere {
      center,
      radius,
      material,
    }
  }
  pub fn set_material<M>(&mut self, material: Box<M>)
  where
    M: Material,
  {
    self.material = material.box_clone();
  }
}

fn in_min_max(t: f64, min: f64, max: f64) -> bool {
  if min > max {
    panic!(
      "Expected max > min, but received min is {}, max is {}",
      min, max
    );
  }
  t < max && t > min
}

impl Hittable for Sphere {
  fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
    let Sphere {
      center,
      radius,
      material,
    } = self;
    let center = *center;
    let radius = *radius;
    let oc = ray.origin - center;
    let a = ray.direction.length_squared();
    let half_b = oc.dot(ray.direction);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant > 0.0 {
      let root = discriminant.sqrt();
      let mut update_record = |temp: f64| {
        let point = ray.at(temp);
        let outward_normal = (point - center) / radius;

        record.set_t(temp);
        record.set_point(point);
        record.set_face_normal(&ray, outward_normal);
        record.set_material(material.box_clone());
      };

      let temp = (-half_b - root) / a;

      if in_min_max(temp, t_min, t_max) {
        update_record(temp);
        return true;
      }

      let temp = (-half_b + root) / a;

      if in_min_max(temp, t_min, t_max) {
        update_record(temp);
        return true;
      }
    }

    return false;
  }
}
