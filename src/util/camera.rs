use rand::Rng;

use crate::geometry::{
    ray::Ray,
    vector_3d::{self, Vector3D},
};

use super::point::Point3D;

pub struct Camera {
    /// Where the camera is located.
    origin: Point3D,
    /// The lower left corner in the scene that we want to render.
    lower_left_corner: Point3D,
    /// Vector representing the height of the scene (from camera perspective).
    horizontal: Vector3D,
    /// Vector representing the width of the scene (from camera perspective).
    vertical: Vector3D,
    /// "up" (y) vector of the camera.
    u: Vector3D,
    /// "right" (x) vector of the camera.
    v: Vector3D,
    /// Radius of the lens
    lens_radius: f64,
    /// Shutter open time
    start_time: f64,
    // Shutter close time
    end_time: f64,
}

impl Camera {
    /// Returns a new camera
    ///
    /// # Arguments
    ///
    /// * `look_from` - The point where the camera is located.
    /// * `look_at` - The point that the camera is pointed at.
    /// * `v_up` - The rotation of the camera.
    /// * `vertical_fov` - Vertical field of view.
    /// * `aspect_ratio` - Vertical to horizontal ratio.
    /// * `aperature` - How large the camera lens is.
    /// * `focus_distance` - What distance the camera is focusing on.
    /// * `start_time` - When the shutter opens.
    /// * `end_time` - When the shutter closes.
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
