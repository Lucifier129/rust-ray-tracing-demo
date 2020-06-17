use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
  pub origin: Vec3,
  pub lower_left_corner: Vec3,
  pub horizontal: Vec3,
  pub vertical: Vec3,
}

impl Camera {
  pub fn new(origin: Vec3, lower_left_corner: Vec3, horizontal: Vec3, vertical: Vec3) -> Camera {
    Camera {
      origin,
      lower_left_corner,
      horizontal,
      vertical,
    }
  }
  pub fn get_ray(&self, u: f64, v: f64) -> Ray {
    let Camera {
      origin,
      lower_left_corner,
      horizontal,
      vertical,
    } = *self;

    let direction = lower_left_corner + u * horizontal + v * vertical;

    Ray::new(origin, direction)
  }
}
