#![feature(split_array)]

pub mod geometry;
pub mod scene;
pub mod util;

use std::sync::atomic::{AtomicI32, Ordering};

use clap::{Parser, ValueEnum};
use geometry::{ray::Ray, vector_3d};
use rand::Rng;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use scene::hittable::Hittable;
use util::color::Color;

use crate::{scene::scene::Scene, util::color::Pixel};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Scene to render
    #[arg(value_enum, default_value_t = Scenes::Balls)]
    scene: Scenes,

    /// Samples to shoot per pixel
    #[arg(long, default_value_t = 100)]
    samples: i32,

    /// Width in pixels of the image
    #[arg(long, default_value_t = 400)]
    width: i32,

    /// Aspect ration (width)
    #[arg(long, default_value_t = 3)]
    aspect_ratio_width: i32,

    /// Aspect ration (height)
    #[arg(long, default_value_t = 2)]
    aspect_ratio_height: i32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Scenes {
    /// Many balls on a checkered surface
    Balls,
    /// Two massive balls
    TwoBalls,
}

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
    let args = Args::parse();

    // Image
    let aspect_ratio: f64 = args.aspect_ratio_width as f64 / args.aspect_ratio_height as f64;
    let image_width: i32 = args.width;
    let image_height: i32 = ((image_width as f64) / aspect_ratio) as i32;
    let samples_per_pixel: i32 = args.samples;
    const MAX_DEPTH: i32 = 50;

    // World
    let scene = match args.scene {
        Scenes::Balls => Scene::random_scene(aspect_ratio),
        Scenes::TwoBalls => Scene::random_scene(aspect_ratio),
    };

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);

    let remaining_scanlines = AtomicI32::new(image_height);
    let mut pixels: Vec<Pixel> = vec![];
    let pixel_row = (0..(image_height * image_width))
        .into_par_iter()
        .map(|count| (image_height - (count / image_width), count % image_width))
        .map(|(row, column)| {
            if column % image_width == 0 {
                let remaining = remaining_scanlines.fetch_sub(1, Ordering::Relaxed);
                eprint!("\rScanlines remaining: {:04}", remaining);
            }

            let mut generator = rand::thread_rng();

            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u =
                    ((column as f64) + generator.gen_range(0.0..1.0)) / ((image_width - 1) as f64);
                let v =
                    ((row as f64) + generator.gen_range(0.0..1.0)) / ((image_height - 1) as f64);
                let ray = scene.camera.get_ray(u, v);
                pixel_color += &ray_color(&ray, &scene.objects, MAX_DEPTH);
            }

            pixel_color.color_code(samples_per_pixel)
        })
        .collect::<Vec<Pixel>>();
    pixels.extend(pixel_row);

    // Write the pixels to the file
    for pixel in pixels {
        pixel.write_color();
    }

    eprintln!("\nDone!");
}
