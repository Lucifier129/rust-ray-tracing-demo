use std::env;
use std::io;

mod camera;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;
mod utils;
mod vec3;

mod demos;

type Demos = Vec<Box<dyn Fn() -> io::Result<()>>>;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Expected one arguments. cargo run {number}");
    }

    let demos: Demos = vec![
        Box::new(demos::demo00::run),
        Box::new(demos::demo01::run),
        Box::new(demos::demo02::run),
        Box::new(demos::demo03::run),
        Box::new(demos::demo04::run),
        Box::new(demos::demo05::run),
        Box::new(demos::demo06::run),
        Box::new(demos::demo07::run),
        Box::new(demos::demo08::run),
        Box::new(demos::demo09::run),
        Box::new(demos::demo10::run),
        Box::new(demos::demo11::run),
        Box::new(demos::demo12::run),
        Box::new(demos::demo13::run),
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
