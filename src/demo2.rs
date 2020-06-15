use std::fs;
use std::io;

use crate::ray::Ray;
use crate::vec3::Vec3;

static FILENAME: &'static str = "images/02.ppm";

fn lerp(t: f64, start: &Vec3, end: &Vec3) -> Vec3 {
  (1.0 - t) * start + (t * end)
}

fn ray_color(ray: &Ray) -> Vec3 {
  let unit_direction = Vec3::unit_vector(&ray.direction);
  let t = 0.5 * (unit_direction.y() + 1.0);

  let from = Vec3(1.0, 1.0, 1.0);
  let to = Vec3(0.5, 0.7, 1.0);

  lerp(t, &from, &to)
}

pub fn run() -> io::Result<()> {
  let aspect_ratio = 16.0 / 9.0;
  let image_width = 256;
  let image_height = 256;

  let part0 = format!("P3\n{} {}\n255\n", image_width, image_height);

  let viewport_height = 2.0;
  let viewport_width = aspect_ratio * viewport_height;
  let focal_length = 1.0;

  let origin = Vec3(0.0, 0.0, 0.0);
  let horizontal = Vec3(viewport_width, 0.0, 0.0);
  let vertical = Vec3(0.0, viewport_height, 0.0);

  let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3(0.0, 0.0, focal_length);

  let mut contents = String::from(part0);

  for j in (0..image_height).rev() {
    for i in 0..image_width {
      let j = j as f64;
      let i = i as f64;
      let w = image_width as f64;
      let h = image_height as f64;

      let u = i / (w - 1.0);
      let v = j / (h - 1.0);

      let direction = lower_left_corner + (u * horizontal) + (v * vertical) - origin;

      let ray = Ray::new(origin.clone(), direction);

      let color = ray_color(&ray);

      contents.push_str(&color.to_rgb_string());
    }
  }

  fs::write(FILENAME, contents.as_bytes())?;

  Ok(())
}
