#![allow(dead_code)]
use crate::ray::Ray;
use crate::utils::random;
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

pub struct PositionalCamera {
  pub origin: Vec3,
  pub lower_left_corner: Vec3,
  pub horizontal: Vec3,
  pub vertical: Vec3,
}

impl PositionalCamera {
  pub fn new(
    look_from: Vec3,
    look_at: Vec3,
    vup: Vec3,
    vfov: f64,
    aspect_ratio: f64,
  ) -> PositionalCamera {
    let theta = vfov.to_radians();
    let h = (theta / 2.0).tan();
    let viewport_height = 2.0 * h;
    let viewport_width = aspect_ratio * viewport_height;

    let w = (look_from - look_at).unit_vector();
    let u = vup.cross(w).unit_vector();
    let v = w.cross(u);

    let origin = look_from;
    let horizontal = viewport_width * u;
    let vertical = viewport_height * v;
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;

    PositionalCamera {
      origin,
      horizontal,
      vertical,
      lower_left_corner,
    }
  }

  pub fn get_ray(&self, s: f64, t: f64) -> Ray {
    let PositionalCamera {
      origin,
      lower_left_corner,
      horizontal,
      vertical,
    } = *self;

    let direction = lower_left_corner + s * horizontal + t * vertical - origin;

    Ray::new(origin, direction)
  }
}

pub struct LensCamera {
  pub origin: Vec3,
  pub lower_left_corner: Vec3,
  pub horizontal: Vec3,
  pub vertical: Vec3,
  u: Vec3,
  v: Vec3,
  w: Vec3,
  lens_radius: f64,
}

impl LensCamera {
  pub fn new(
    look_from: Vec3,
    look_at: Vec3,
    vup: Vec3,
    vfov: f64,
    aspect_ratio: f64,
    aperture: f64,
    focus_dist: f64,
  ) -> LensCamera {
    let theta = vfov.to_radians();
    let h = (theta / 2.0).tan();
    let viewport_height = 2.0 * h;
    let viewport_width = aspect_ratio * viewport_height;

    let w = (look_from - look_at).unit_vector();
    let u = vup.cross(w).unit_vector();
    let v = w.cross(u);

    let origin = look_from;
    let horizontal = focus_dist * viewport_width * u;
    let vertical = focus_dist * viewport_height * v;
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;
    let lens_radius = aperture / 2.0;

    LensCamera {
      origin,
      horizontal,
      vertical,
      lower_left_corner,
      u,
      v,
      w,
      lens_radius,
    }
  }

  pub fn get_ray(&self, s: f64, t: f64) -> Ray {
    let LensCamera {
      origin,
      lower_left_corner,
      horizontal,
      vertical,
      u,
      v,
      w: _w,
      lens_radius,
    } = *self;

    let rd = lens_radius * Vec3::random_in_unit_disk();
    let offset = u * rd.x() + v * rd.y();

    let direction = lower_left_corner + s * horizontal + t * vertical - origin - offset;

    Ray::new(origin + offset, direction)
  }
}

pub struct ExposureCamera {
  pub lens_camera: LensCamera,
  exposure_start_time: f64,
  exposure_end_time: f64,
  look_from: Vec3,
  look_at: Vec3,
  vup: Vec3,
  vfov: f64,
  aspect_ratio: f64,
  aperture: f64,
  focus_dist: f64,
}

impl ExposureCamera {
  pub fn new(
    look_from: Vec3,
    look_at: Vec3,
    vup: Vec3,
    vfov: f64,
    aspect_ratio: f64,
    aperture: f64,
    focus_dist: f64,
    exposure_start_time: f64,
    exposure_end_time: f64,
  ) -> ExposureCamera {
    let lens_camera = LensCamera::new(
      look_from,
      look_at,
      vup,
      vfov,
      aspect_ratio,
      aperture,
      focus_dist,
    );

    ExposureCamera {
      lens_camera,
      exposure_start_time,
      exposure_end_time,
      look_from,
      look_at,
      vup,
      vfov,
      aspect_ratio,
      aperture,
      focus_dist,
    }
  }

  pub fn get_ray(&self, s: f64, t: f64) -> Ray {
    let lens_camera = &self.lens_camera;
    let exposure_start_time = self.exposure_start_time;
    let exposure_end_time = self.exposure_end_time;

    let mut ray = lens_camera.get_ray(s, t);

    ray.time = exposure_start_time + random() * (exposure_end_time - exposure_start_time);

    ray
  }

  pub fn process_keyboard(&mut self, direction: u8) {
    let right = self.lens_camera.u;
    let front = (self.look_from - self.look_at).unit_vector();
    let velocity = 1.2;
    
    match direction {
      // forward
      0 => {
        let offset = -velocity * front;
        self.look_from = self.look_from + offset;
      },
      // backward
      1 => {
        let offset = velocity * front;
        self.look_from = self.look_from + offset;
      }
      // left
      2 => {
        let offset = -velocity * right;
        self.look_from = self.look_from + offset;
      }
      // right
      3 => {
        let offset = velocity * right;
        self.look_from = self.look_from + offset;
      }
      // unknown
      _ => return
    };

    self.update_camera();
  }

  fn update_camera(&mut self) {
    self.lens_camera = LensCamera::new(
      self.look_from,
      self.look_at,
      self.vup,
      self.vfov,
      self.aspect_ratio,
      self.aperture,
      self.focus_dist,
    );
  }
}
