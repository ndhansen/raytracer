pub mod geometry;
pub mod scene;
pub mod util;

use std::sync::atomic::{AtomicI32, Ordering};

use geometry::{ray::Ray, vector_3d};
use rand::Rng;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use scene::hittable::Hittable;
use util::color::Color;

use crate::{geometry::vector_3d::Vector3D, scene::{materials::{Dielectric, Lambertian, Material, Metal}, sphere::Sphere}, util::{camera::Camera, color::Pixel, point::Point3D}};

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

fn random_scene() -> Vec<Box<dyn Hittable>> {
    let mut world: Vec<Box<dyn Hittable>> = vec![];

    let ground_material = Box::new(Lambertian::new(&Color::new(0.5, 0.5, 0.5)));
    world.push(Box::new(Sphere::new(Point3D::new(0.0, -1000.0, 0.0), 1000.0, ground_material)));

    let mut generator = rand::thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let choosen_material = generator.gen_range(0.0..0.5);
            let center = Point3D::new(
                (a as f64) + 0.9 * generator.gen_range(0.0..1.0),
                0.2,
                (b as f64) + 0.9 * generator.gen_range(0.0..1.0),
            );

            if (center - Point3D::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Box<dyn Material> = if choosen_material < 0.8 {
                    // diffuse
                    let albedo = Color::all_random() * Color::all_random();
                    Box::new(Lambertian::new(&albedo))
                } else if choosen_material < 0.95 {
                    let albedo = Color::random(0.5, 1.0);
                    let fuzz = generator.gen_range(0.0..0.5);
                    Box::new(Metal::new(&albedo, fuzz))
                } else {
                    Box::new(Dielectric::new(1.5))
                };
                world.push(Box::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }

    let dielectric = Box::new(Dielectric::new(1.5));
    world.push(Box::new(Sphere::new(Point3D::new(0.0, 1.0, 0.0), 1.0, dielectric)));

    let lambertian = Box::new(Lambertian::new(&Color::new(0.4, 0.2, 0.1)));
    world.push(Box::new(Sphere::new(Point3D::new(-4.0, 1.0, 0.0), 1.0, lambertian)));

    let metal = Box::new(Metal::new(&Color::new(0.7, 0.6, 0.5), 0.0));
    world.push(Box::new(Sphere::new(Point3D::new(4.0, 1.0, 0.0), 1.0, metal)));

    world
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: i32 = 920;
    const IMAGE_HEIGHT: i32 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: i32 = 50;

    // World
    let world = random_scene();

    // Camera
    let look_from = Point3D::new(13.0, 2.0, 3.0);
    let look_at = Point3D::new(0.0, 0.0, 0.0);
    let v_up = Vector3D::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperature = 0.1;
    let camera = Camera::new(
        look_from,
        look_at,
        v_up,
        20.0,
        ASPECT_RATIO,
        aperature,
        dist_to_focus,
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
