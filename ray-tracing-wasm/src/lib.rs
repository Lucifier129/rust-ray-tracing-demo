use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn logv(x: &JsValue);

    #[wasm_bindgen(js_namespace = console, js_name = time)]
    fn start_time(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = timeEnd)]
    fn end_time(s: &str);
}

extern crate ray_tracing;

use ray_tracing::camera::ExposureCamera;
use ray_tracing::hittable::{HitRecord, Hittable};
use ray_tracing::hittable_list::HittableList;
use ray_tracing::material::{Dielectric, Lambertian, Material, Metal};
use ray_tracing::ray::Ray;
use ray_tracing::sphere::{MovingSphere, Sphere};
use ray_tracing::utils::random;
use ray_tracing::utils::random_in;
use ray_tracing::vec3::Vec3;

#[wasm_bindgen]
pub fn test_random() -> f64 {
    random()
}

fn ray_color<W>(ray: &Ray, world: &W, depth: usize) -> Vec3
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

fn random_scene(small_sphere_counts: i32) -> HittableList {
    let count = ((small_sphere_counts as f64).sqrt() / 2.0) as i32;
    let start = -count;
    let end = count;
    let mut world = HittableList::new();
    let mut ground = Sphere::new(Vec3(0.0, -1000.0, 0.0), 1000.0);
    let ground_material = Lambertian::new(Vec3(0.5, 0.5, 0.5));
    ground.set_material(Box::new(ground_material));
    world.add(Box::new(ground));
    let create_sphere = |center: Vec3, material: Box<dyn Material>| -> Box<dyn Hittable + Send> {
        if random() < 0.7 {
            let mut sphere = Sphere::new(center, 0.2);
            sphere.set_material(material);
            Box::new(sphere)
        } else {
            let mut moving_sphere = MovingSphere::new(
                center,
                center + Vec3(0.0, 0.5 * random(), 0.0),
                0.0,
                1.0,
                0.2,
            );
            moving_sphere.set_material(material);
            Box::new(moving_sphere)
        }
    };
    for a in start..end {
        for b in start..end {
            let a = a as f64;
            let b = b as f64;
            let choose_material = random();
            let center = Vec3(a + 0.9 * random(), 0.2, b + 0.9 * random());
            let point = Vec3(4.0, 0.2, 0.0);

            if (center - point).len() > 2.0 {
                if choose_material < 0.8 {
                    // diffuse
                    let albedo = Vec3::random() * Vec3::random();
                    let material = Lambertian::new(albedo);
                    let sphere = create_sphere(center, Box::new(material));
                    world.add(sphere);
                } else if choose_material < 0.95 {
                    // metal
                    let albedo = Vec3::random();
                    let fuzz = random_in(0.0, 0.5);
                    let material = Metal::new(albedo).set_fuzz(fuzz);
                    let sphere = create_sphere(center, Box::new(material));
                    world.add(sphere);
                } else {
                    // glass
                    let material = Dielectric::new(1.5);
                    let sphere = create_sphere(center, Box::new(material));
                    world.add(sphere);
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

#[wasm_bindgen]
pub struct Scene {
    image_width: usize,
    image_height: usize,
    max_depth: usize,
    camera: ExposureCamera,
    world: HittableList,
}

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
impl Scene {
    #[wasm_bindgen(constructor)]
    pub fn new(image_width: usize, image_height: usize, max_depth: usize) -> Scene {
        init_panic_hook();
        let aspect_ratio = (image_width as f64) / (image_height as f64);
        let look_from = Vec3(13.0, 2.0, 3.0);
        let look_at = Vec3(0.0, 0.0, 0.0);
        let vup = Vec3(0.0, 1.0, 0.0);
        let dist_to_focus = 10.0;
        let aperture = 0.0;

        let camera = ExposureCamera::new(
            look_from,
            look_at,
            vup,
            20.0,
            aspect_ratio,
            aperture,
            dist_to_focus,
            0.0,
            1.0,
        );

        let world = HittableList::new();

        Scene {
            image_width,
            image_height,
            max_depth,
            camera,
            world,
        }
    }

    pub fn process_keyboard(&mut self, direction: u8) {
        self.camera.process_keyboard(direction);
    }

    pub fn random_scene(&mut self, small_sphere_counts: i32) {
        self.world = random_scene(small_sphere_counts);
    }

    pub fn render_by_position(&self, x: usize, y: usize) -> Vec<f64> {
        if x > self.image_width - 1 {
            panic!(
                "x = {} is out of scope, max number is {}",
                x,
                self.image_width - 1
            )
        }

        if y > self.image_height - 1 {
            panic!(
                "y = {} is out of scope, max number is {}",
                y,
                self.image_height - 1
            )
        }

        let image_width = self.image_width;
        let image_height = self.image_height;
        let camera = &self.camera;
        let world = &self.world;
        let max_depth = self.max_depth;

        let x = x as f64;
        let y = y as f64;
        let w = image_width as f64;
        let h = image_height as f64;
        let u = (x + random()) / (w - 1.0);
        let v = (y + random()) / (h - 1.0);
        let ray = camera.get_ray(u, v);
        let color = ray_color(&ray, world, max_depth);

        vec![color.r(), color.g(), color.b()]
    }

    pub fn render(&self) -> Vec<f64> {
        let image_width = self.image_width;
        let image_height = self.image_height;
        let mut pixels = vec![];

        for j in (0..image_height).rev() {
            for i in 0..image_width {
                let mut rgb = self.render_by_position(i, j);
                pixels.append(&mut rgb);
            }
        }

        pixels
    }
}
