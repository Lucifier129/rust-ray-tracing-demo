use std::fs;
use std::io;

use crate::vec3::Vec3;

static FILENAME: &'static str = "dist/01.ppm";

pub fn run() -> io::Result<()> {
  let image_width = 256;
  let image_height = 256;

  let part0 = format!("P3\n{} {}\n255\n", image_width, image_height);

  let mut contents = String::from(part0);

  for j in (0..image_height).rev() {
    for i in 0..image_width {
      let j = j as f64;
      let i = i as f64;
      let w = image_width as f64;
      let h = image_height as f64;

      let color = Vec3(i / w, j / h, 0.25);

      contents.push_str(&color.to_rgb_string());
    }
  }

  fs::write(FILENAME, contents.as_bytes())?;

  Ok(())
}
