pub mod geometry;
pub mod util;

use geometry::{ray::Ray, vector_3d};
use util::color::Color;

use crate::{geometry::vector_3d::Vector3D, util::point::Point3D};

fn ray_color(ray: &Ray) -> Color {
    let unit_direction = vector_3d::unit_vector(&ray.direction);
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as i32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3D::empty();
    let horizontal = Vector3D::new(viewport_width, 0.0, 0.0);
    let vertical = Vector3D::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - (horizontal / 2.0) - (vertical / 2.0) - Vector3D::new(0.0, 0.0, focal_length);

    // Render
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for row in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {}", row);
        for column in 0..IMAGE_WIDTH {
            let u = (column as f64) / ((IMAGE_WIDTH - 1) as f64);
            let v = (row as f64) / ((IMAGE_HEIGHT - 1) as f64);
            let ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color = ray_color(&ray);

            pixel_color.write_color();
        }
    }
    eprintln!("\nDone!");
}
