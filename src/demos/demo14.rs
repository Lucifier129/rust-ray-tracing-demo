use std::fs;
use std::io;
use std::time::Instant;

use crate::camera::LensCamera;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::utils::random;
use crate::utils::random_in;
use crate::vec3::Vec3;

extern crate threadpool;

use std::sync::mpsc::channel;
use std::sync::Arc;
use threadpool::ThreadPool;

static FILENAME: &'static str = "dist/14.ppm";

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
    let mut attenuation = Vec3::fill(0.0);
    let mut material = record.material.box_clone();

    if material.scatter(&ray, &mut record, &mut attenuation, &mut scattered) {
      return attenuation * ray_color(&scattered, world, depth - 1);
    } else {
      return Vec3::fill(0.0);
    }
  }

  let unit_direction = ray.direction.unit_vector();
  let t = 0.5 * (unit_direction.y() + 1.0);

  let from = Vec3(1.0, 1.0, 1.0);
  let to = Vec3(0.5, 0.7, 1.0);

  Vec3::lerp(t, from, to)
}

fn random_scene() -> HittableList {
  let mut world = HittableList::new();

  let mut ground = Sphere::new(Vec3(0.0, -1000.0, 0.0), 1000.0);
  let ground_material = Lambertian::new(Vec3(0.5, 0.5, 0.5));
  ground.set_material(Box::new(ground_material));

  world.add(Box::new(ground));

  for a in -11..11 {
    for b in -11..11 {
      let a = a as f64;
      let b = b as f64;
      let choose_material = random();
      let center = Vec3(a + 0.9 * random(), 0.2, b + 0.9 * random());

      let point = Vec3(4.0, 0.2, 0.0);

      if (center - point).len() > 0.9 {
        if choose_material < 0.8 {
          // diffuse
          let albedo = Vec3::random() * Vec3::random();
          let material = Lambertian::new(albedo);
          let mut sphere = Sphere::new(center, 0.2);
          sphere.set_material(Box::new(material));
          world.add(Box::new(sphere));
        } else if choose_material < 0.95 {
          // metal
          let albedo = Vec3::random();
          let fuzz = random_in(0.0, 0.5);
          let material = Metal::new(albedo).set_fuzz(fuzz);
          let mut sphere = Sphere::new(center, 0.2);
          sphere.set_material(Box::new(material));
          world.add(Box::new(sphere));
        } else {
          // glass
          let material = Dielectric::new(1.5);
          let mut sphere = Sphere::new(center, 0.2);
          sphere.set_material(Box::new(material));
          world.add(Box::new(sphere));
        }
      }
    }
  }

  let material = Dielectric::new(1.5);
  let mut sphere = Sphere::new(Vec3(0.0, 1.0, 0.0), 1.0);
  sphere.set_material(Box::new(material));
  world.add(Box::new(sphere));

  let material = Lambertian::new(Vec3(0.4, 0.2, 0.1));
  let mut sphere = Sphere::new(Vec3(-4.0, 1.0, 0.0), 1.0);
  sphere.set_material(Box::new(material));
  world.add(Box::new(sphere));

  let material = Metal::new(Vec3(0.7, 0.6, 0.5)).set_fuzz(0.0);
  let mut sphere = Sphere::new(Vec3(4.0, 1.0, 0.0), 1.0);
  sphere.set_material(Box::new(material));
  world.add(Box::new(sphere));

  world
}

type Pixels = Vec<Vec3>;

pub fn run() -> io::Result<()> {
  let aspect_ratio = 16.0 / 9.0;
  let image_width = 384;
  let image_height = ((image_width as f64) / aspect_ratio) as i32;

  let pixels_size = (image_width * image_height) as usize;

  let samples_per_pixel = 50;
  let max_depth = 50;

  let part0 = format!("P3\n{} {}\n255\n", image_width, image_height);
  let mut contents = String::from(part0);

  let look_from = Vec3(13.0, 2.0, 3.0);
  let look_at = Vec3(0.0, 0.0, 0.0);
  let vup = Vec3(0.0, 1.0, 0.0);
  let dist_to_focus = 10.0;
  let aperture = 0.1;

  let camera = LensCamera::new(
    look_from,
    look_at,
    vup,
    20.0,
    aspect_ratio,
    aperture,
    dist_to_focus,
  );

  let world = random_scene();

  let world = Arc::new(world);
  let camera = Arc::new(camera);

  let pool = ThreadPool::new(4);
  let (sender, receiver) = channel::<Pixels>();

  let start = Instant::now();

  println!("Ray Tracing start");

  for n in 0..samples_per_pixel {
    let sender = sender.clone();
    let world = Arc::clone(&world);
    let camera = Arc::clone(&camera);
    let tracing = move || {
      let mut pixels = Vec::<Vec3>::with_capacity(pixels_size);

      println!("Ray Tracing: start {}", n);

      for j in (0..image_height).rev() {
        for i in 0..image_width {
          let j = j as f64;
          let i = i as f64;
          let w = image_width as f64;
          let h = image_height as f64;
          let u = (i + random()) / (w - 1.0);
          let v = (j + random()) / (h - 1.0);
          let ray = camera.get_ray(u, v);
          let color = ray_color(&ray, &*world, max_depth);
          pixels.push(color);
        }
      }

      println!("Ray Tracing: finish {}", n);
      sender.send(pixels).expect("ray tracing failed");
    };
    pool.execute(tracing);
  }

  let mut colors = Vec::<Vec3>::with_capacity(pixels_size);

  for _ in 0..pixels_size {
    colors.push(Vec3::fill(0.0));
  }

  for pixels in receiver.iter().take(samples_per_pixel) {
    for (i, color) in colors.iter_mut().enumerate() {
      *color = *color + pixels[i];
    }
  }

  println!("ThreadPool taked time: {:?}", start.elapsed());

  for color in colors {
    let color = color / (samples_per_pixel as f64);
    // gamma-correct for gamma=2.0
    let color = color.sqrt();
    contents.push_str(&color.to_rgb_string());
  }

  println!("Ray Tracing taked time: {:?}", start.elapsed());

  fs::write(FILENAME, contents.as_bytes())?;

  Ok(())
}
