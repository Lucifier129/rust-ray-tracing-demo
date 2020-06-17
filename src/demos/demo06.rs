use std::fs;
use std::io;

use crate::camera::Camera;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::utils::random;
use crate::vec3::Vec3;

static FILENAME: &'static str = "dist/06.ppm";

fn ray_color<W>(ray: &Ray, world: &W) -> Vec3
where
  W: Hittable,
{
  let mut record = HitRecord::new();

  if world.hit(&ray, 0.0, f64::INFINITY, &mut record) {
    return 0.5 * (record.normal + Vec3(1.0, 1.0, 1.0));
  }

  let unit_direction = Vec3::unit_vector(&ray.direction);
  let t = 0.5 * (unit_direction.y() + 1.0);

  let from = Vec3(1.0, 1.0, 1.0);
  let to = Vec3(0.5, 0.7, 1.0);

  Vec3::lerp(t, from, to)
}

pub fn run() -> io::Result<()> {
  let aspect_ratio = 16.0 / 9.0;
  let image_width = 384;
  let image_height = ((image_width as f64) / aspect_ratio) as i32;

  let samples_per_pixel = 50;

  let part0 = format!("P3\n{} {}\n255\n", image_width, image_height);
  let mut contents = String::from(part0);

  let viewport_height = 2.0;
  let viewport_width = aspect_ratio * viewport_height;
  let focal_length = 1.0;

  let origin = Vec3(0.0, 0.0, 0.0);
  let horizontal = Vec3(viewport_width, 0.0, 0.0);
  let vertical = Vec3(0.0, viewport_height, 0.0);
  let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3(0.0, 0.0, focal_length);

  let camera = Camera::new(origin, lower_left_corner, horizontal, vertical);

  let mut world = HittableList::new();

  let sphere_0 = Sphere::new(Vec3(0.0, 0.0, -1.0), 0.5);
  let sphere_1 = Sphere::new(Vec3(0.0, -100.5, -1.0), 100.0);

  world.add(Box::new(sphere_0));
  world.add(Box::new(sphere_1));

  for j in (0..image_height).rev() {
    for i in 0..image_width {
      let j = j as f64;
      let i = i as f64;
      let w = image_width as f64;
      let h = image_height as f64;

      let mut color = Vec3(0.0, 0.0, 0.0);

      for _ in 0..samples_per_pixel {
        let u = (i + random()) / (w - 1.0);
        let v = (j + random()) / (h - 1.0);
        let ray = camera.get_ray(u, v);
        color = color + ray_color(&ray, &world);
      }

      color = color / (samples_per_pixel as f64);
      contents.push_str(&color.to_rgb_string());
    }
  }

  fs::write(FILENAME, contents.as_bytes())?;

  Ok(())
}
