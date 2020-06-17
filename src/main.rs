use std::env;
use std::io;

mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod utils;
mod vec3;
mod camera;

mod demo0;
mod demo1;
mod demo2;
mod demo3;
mod demo4;
mod demo5;
mod demo6;
mod demo7;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Expected one arguments. cargo run {number}");
    }

    if let Ok(n) = args[1].parse::<u8>() {
        match n {
            0 => demo0::run()?,
            1 => demo1::run()?,
            2 => demo2::run()?,
            3 => demo3::run()?,
            4 => demo4::run()?,
            5 => demo5::run()?,
            6 => demo6::run()?,
            7 => demo7::run()?,
            _ => panic!("demo{} is not implemented yet", n),
        }
    } else {
        panic!("Expected a number. cargo run {number}");
    }

    Ok(())
}
