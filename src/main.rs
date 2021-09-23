pub mod geometry;
pub mod scene;
pub mod util;

use std::sync::atomic::{AtomicI32, Ordering};

use geometry::{ray::Ray, vector_3d};
use rand::Rng;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use scene::hittable::Hittable;
use util::color::Color;

use crate::{geometry::vector_3d::Vector3D, scene::{materials::{Dielectric, Lambertian, Metal}, sphere::Sphere}, util::{camera::Camera, color::Pixel, point::Point3D}};

fn ray_color(ray: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(record) = world.hit(ray, 0.001, f64::INFINITY) {
        match record.material.scatter(ray, &record) {
            Some((attenuation, scattered)) => {
                return attenuation * ray_color(&scattered, world, depth - 1);
            }
            None => {
                return Color::new(0.0, 0.0, 0.0);
            }
        }
    }

    let unit_direction = vector_3d::unit_vector(&ray.direction);
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 600;
    const IMAGE_HEIGHT: i32 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: i32 = 50;

    // World
    let mut world: Vec<Box<dyn Hittable>> = vec![];

    let material_ground = Box::new(Lambertian::new(&Color::new(0.8, 0.8, 0.0)));
    let material_center = Box::new(Lambertian::new(&Color::new(0.1, 0.2, 0.5)));
    let material_left = Box::new(Dielectric::new(1.5));
    let material_left_2 = Box::new(Dielectric::new(1.5));
    let material_right = Box::new(Metal::new(&Color::new(0.8, 0.6, 0.2), 0.0));

    world.push(Box::new(Sphere::new(
        Point3D::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.push(Box::new(Sphere::new(
        Point3D::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.push(Box::new(Sphere::new(
        Point3D::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.push(Box::new(Sphere::new(
        Point3D::new(-1.0, 0.0, -1.0),
        -0.4,
        material_left_2,
    )));
    world.push(Box::new(Sphere::new(
        Point3D::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    // Camera
    let camera = Camera::new(
        Point3D::new(-2.0, 2.0 , 1.0),
        Point3D::new(0.0, 0.0, -1.0),
        Vector3D::new(0.0, 1.0, 0.0),
        20.0,
        ASPECT_RATIO
    );

    // Render
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    let remaining_scanlines = AtomicI32::new(IMAGE_HEIGHT);
    let mut pixels: Vec<Pixel> = vec![];
    let pixel_row = (0..(IMAGE_HEIGHT * IMAGE_WIDTH))
        .into_par_iter()
        .map(|count| (IMAGE_HEIGHT - (count / IMAGE_WIDTH), count % IMAGE_WIDTH))
        .map(|(row, column)| {
            if column % IMAGE_WIDTH == 0 {
                let remaining = remaining_scanlines.fetch_sub(1, Ordering::Relaxed);
                eprint!("\rScanlines remaining: {:04}", remaining);
            }

            let mut generator = rand::thread_rng();

            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u =
                    ((column as f64) + generator.gen_range(0.0..1.0)) / ((IMAGE_WIDTH - 1) as f64);
                let v =
                    ((row as f64) + generator.gen_range(0.0..1.0)) / ((IMAGE_HEIGHT - 1) as f64);
                let ray = camera.get_ray(u, v);
                pixel_color += &ray_color(&ray, &world, MAX_DEPTH);
            }

            pixel_color.color_code(SAMPLES_PER_PIXEL)
        })
        .collect::<Vec<Pixel>>();
    pixels.extend(pixel_row);

    // Write the pixels to the file
    for pixel in pixels {
        pixel.write_color();
    }

    eprintln!("\nDone!");
}
