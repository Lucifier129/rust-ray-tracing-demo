#[warn(dead_code)]
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

pub struct HittableList {
  pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
  pub fn new() -> HittableList {
    HittableList { objects: vec![] }
  }

  pub fn clear(&mut self) {
    self.objects = vec![];
  }

  pub fn add(&mut self, object: Box<dyn Hittable + Send>) {
    self.objects.push(object);
  }
}

impl Hittable for HittableList {
  fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
    let mut inner_record = HitRecord::new();
    let mut hit_anything = false;
    let mut closest_so_far = t_max;

    for object in &self.objects {
      if object.hit(ray, t_min, closest_so_far, &mut inner_record) {
        hit_anything = true;
        closest_so_far = inner_record.t;
      }
    }

    if hit_anything {
      *record = inner_record;
    }

    hit_anything
  }
}
