use std::fs;
use std::io;
use std::time::Instant;

use crate::camera::Camera;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::utils::random;
use crate::vec3::Vec3;

static FILENAME: &'static str = "dist/10.ppm";

fn ray_color<W>(ray: &Ray, world: &W, depth: u64) -> Vec3
where
  W: Hittable,
{
  if depth <= 0 {
    return Vec3::fill(0.0);
  }

  let mut record = HitRecord::new();

  if world.hit(&ray, 0.001, f64::INFINITY, &mut record) {
    let mut scattered = Ray::new(Vec3::fill(0.0), Vec3::fill(0.0));
    let mut attenuation = Vec3::fill(1.0);
    let mut material = record.material.box_clone();

    if material.scatter(&ray, &mut record, &mut attenuation, &mut scattered) {
      return attenuation * ray_color(&scattered, world, depth - 1);
    } else {
      return Vec3::fill(0.0);
    }
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
  let max_depth = 50;

  let part0 = format!("P3\n{} {}\n255\n", image_width, image_height);
  let mut contents = String::from(part0);

  let viewport_height = 2.0;
  let viewport_width = aspect_ratio * viewport_height;
  let focal_length = 1.0;

  let origin = Vec3::fill(0.0);
  let horizontal = Vec3(viewport_width, 0.0, 0.0);
  let vertical = Vec3(0.0, viewport_height, 0.0);
  let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3(0.0, 0.0, focal_length);

  let camera = Camera::new(origin, lower_left_corner, horizontal, vertical);

  let mut world = HittableList::new();

  let mut sphere_0 = Sphere::new(Vec3(0.0, 0.0, -1.0), 0.5);
  let mut sphere_1 = Sphere::new(Vec3(0.0, -100.5, -1.0), 100.0);
  let mut sphere_2 = Sphere::new(Vec3(1.0, 0.0, -1.0), 0.5);
  let mut sphere_3 = Sphere::new(Vec3(-1.0, 0.0, -1.0), 0.5);

  let material_0 = Lambertian::new(Vec3(0.1, 0.2, 0.5));
  let material_1 = Lambertian::new(Vec3(0.8, 0.8, 0.0));
  let material_2 = Metal::new(Vec3(0.8, 0.6, 0.2)).set_fuzz(0.5);
  let material_3 = Dielectric::new(1.5);

  sphere_0.set_material(Box::new(material_0));
  sphere_1.set_material(Box::new(material_1));
  sphere_2.set_material(Box::new(material_2));
  sphere_3.set_material(Box::new(material_3));

  world.add(Box::new(sphere_0));
  world.add(Box::new(sphere_1));
  world.add(Box::new(sphere_2));
  world.add(Box::new(sphere_3));

  let start = Instant::now();

  for j in (0..image_height).rev() {
    for i in 0..image_width {
      let j = j as f64;
      let i = i as f64;
      let w = image_width as f64;
      let h = image_height as f64;

      let mut color = Vec3::fill(0.0);

      for _ in 0..samples_per_pixel {
        let u = (i + random()) / (w - 1.0);
        let v = (j + random()) / (h - 1.0);
        let ray = camera.get_ray(u, v);
        color = color + ray_color(&ray, &world, max_depth);
      }

      color = color / (samples_per_pixel as f64);
      // gamma-correct for gamma=2.0
      color = color.sqrt();
      contents.push_str(&color.to_rgb_string());
    }
  }

  let duration = start.elapsed();

  println!("Ray Tracing taked time: {:?}", duration);

  fs::write(FILENAME, contents.as_bytes())?;

  Ok(())
}
