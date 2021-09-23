use crate::geometry::{ray::Ray, vector_3d::{self, Vector3D}};

use super::point::Point3D;

pub struct Camera {
    origin: Point3D,
    lower_left_corner: Point3D,
    horizontal: Vector3D,
    vertical: Vector3D,
}

impl Camera {
    pub fn new(
        look_from: Point3D,
        look_at: Point3D,
        v_up: Vector3D,
        vertical_fov: f64,
        aspect_ratio: f64,
    ) -> Camera {
        let theta = vertical_fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = vector_3d::unit_vector(&(look_from - look_at));
        let u = vector_3d::unit_vector(&v_up.cross(&w));
        let v = w.cross(&u);

        let origin = look_from;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - (horizontal / 2.0) - (vertical / 2.0) - w;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + (s * self.horizontal) + (t * self.vertical) - self.origin,
        )
    }
}
