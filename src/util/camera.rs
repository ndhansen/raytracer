use crate::geometry::{ray::Ray, vector_3d::Vector3D};

use super::point::Point3D;

pub struct Camera {
    origin: Point3D,
    lower_left_corner: Point3D,
    horizontal: Vector3D,
    vertical: Vector3D,
}

impl Camera {
    pub fn new() -> Camera {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Point3D::empty();
        let horizontal = Vector3D::new(viewport_width, 0.0, 0.0);
        let vertical = Vector3D::new(0.0, viewport_height, 0.0);

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin
                - (horizontal / 2.0)
                - (vertical / 2.0)
                - Vector3D::new(0.0, 0.0, focal_length),
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + (u * self.horizontal) + (v * self.vertical) - self.origin,
        )
    }
}