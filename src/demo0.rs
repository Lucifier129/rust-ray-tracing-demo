use std::fs;
use std::io;

static FILENAME: &'static str = "images/00.ppm";

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

      let r = i / w;
      let g = j / h;
      let b = 0.25;

      let ir = (255.999 * r) as u32;
      let ig = (255.999 * g) as u32;
      let ib = (255.999 * b) as u32;
      let content = format!("{} {} {}\n", ir, ig, ib);

      contents.push_str(&content);
    }
  }

  fs::write(FILENAME, contents.as_bytes())?;

  Ok(())
}
