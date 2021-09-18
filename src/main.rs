pub mod geometry;
pub mod scene;
pub mod util;

use std::rc::Rc;

use geometry::{ray::Ray, vector_3d};
use scene::hittable::Hittable;
use util::color::Color;
use rand::Rng;

use crate::{scene::sphere::Sphere, util::{camera::Camera, point::Point3D}};

fn ray_color(ray: &Ray, world: &dyn Hittable) -> Color {
    match world.hit(ray, 0.0, f64::INFINITY) {
        Some(record) => {
            return 0.5 * (record.normal + Color::new(1.0, 1.0, 1.0));
        }
        None => {
            let unit_direction = vector_3d::unit_vector(&ray.direction);
            let t = 0.5 * (unit_direction.y() + 1.0);
            return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
        }
    }
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 100;

    // World
    let mut world: Vec<Rc<dyn Hittable>> = vec![];
    let sphere_one = Rc::new(Sphere::new(Point3D::new(0.0, 0.0, -1.0), 0.5));
    world.push(sphere_one);
    let sphere_two = Rc::new(Sphere::new(Point3D::new(0.0, -100.5, -1.0), 100.0));
    world.push(sphere_two);

    // Camera
    let camera = Camera::new();

    // Render
    let mut generator = rand::thread_rng();

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for row in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {:03}", row);
        for column in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = ((column as f64) + generator.gen_range(0.0..1.0)) / ((IMAGE_WIDTH - 1) as f64);
                let v = ((row as f64) + generator.gen_range(0.0..1.0)) / ((IMAGE_HEIGHT - 1) as f64);
                let ray = camera.get_ray(u, v);
                pixel_color += &ray_color(&ray, &world);
            }

            pixel_color.write_color(SAMPLES_PER_PIXEL);
        }
    }
    eprintln!("\nDone!");
}
