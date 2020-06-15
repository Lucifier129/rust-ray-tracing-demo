use crate::vec3::Vec3;

pub fn run() {
  let a = Vec3(0.0, 0.0, 1.0);
  let b = Vec3(1.0, 1.0, 0.0);
  let c = &a + &b;

  println!("{:?} + {:?} = {:?}", a, b, c);
}
