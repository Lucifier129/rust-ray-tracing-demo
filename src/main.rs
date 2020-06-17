use std::env;
use std::io;

mod camera;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod utils;
mod vec3;
mod material;

mod demo0;
mod demo1;
mod demo2;
mod demo3;
mod demo4;
mod demo5;
mod demo6;
mod demo7;
mod demo8;
mod demo9;

type Demos = Vec<Box<dyn Fn() -> io::Result<()>>>;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Expected one arguments. cargo run {number}");
    }

    let demos: Demos = vec![
        Box::new(demo0::run),
        Box::new(demo1::run),
        Box::new(demo2::run),
        Box::new(demo3::run),
        Box::new(demo4::run),
        Box::new(demo5::run),
        Box::new(demo6::run),
        Box::new(demo7::run),
        Box::new(demo8::run),
        Box::new(demo9::run),
    ];

    let length = demos.len();

    // run all demo
    if args[1] == "*" {
        for run in demos.iter() {
            run()?;
        }
        return Ok(());
    }

    // run specified demo
    if let Ok(n) = args[1].parse::<usize>() {
        if n < length {
            &demos[n]()?;
            return Ok(());
        } else {
            panic!("demo{} is not implemented yet", n)
        }
    } else {
        panic!("Expected a number. cargo run {number}");
    }
}
