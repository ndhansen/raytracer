use rand::Rng;

use crate::geometry::{
    ray::Ray,
    vector_3d::{self, Vector3D},
};

use super::point::Point3D;

pub struct Camera {
    origin: Point3D,
    lower_left_corner: Point3D,
    horizontal: Vector3D,
    vertical: Vector3D,
    u: Vector3D,
    v: Vector3D,
    lens_radius: f64,
    start_time: f64, // Shutter open time
    end_time: f64,   // Shutter close time
}

impl Camera {
    pub fn new(
        look_from: Point3D,
        look_at: Point3D,
        v_up: Vector3D,
        vertical_fov: f64,
        aspect_ratio: f64,
        aperature: f64,
        focus_distance: f64,
        start_time: f64,
        end_time: f64,
    ) -> Camera {
        let theta = vertical_fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = vector_3d::unit_vector(&(look_from - look_at));
        let u = vector_3d::unit_vector(&v_up.cross(&w));
        let v = w.cross(&u);

        let origin = look_from;
        let horizontal = focus_distance * viewport_width * u;
        let vertical = focus_distance * viewport_height * v;
        let lower_left_corner = origin - (horizontal / 2.0) - (vertical / 2.0) - focus_distance * w;

        let lens_radius = aperature / 2.0;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            lens_radius,
            start_time,
            end_time,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vector3D::random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
        let mut rng = rand::thread_rng();

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + (s * self.horizontal) + (t * self.vertical)
                - self.origin
                - offset,
            Some(rng.gen_range(self.start_time..self.end_time)),
        )
    }
}
