use crate::{
    geometry::{ray::Ray, vector_3d::Vector3D},
    scene::hit_record::HitRecord,
    util::color::Color,
};

use super::Material;

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: &Color) -> Lambertian {
        Lambertian { albedo: *albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = record.normal + Vector3D::random_unit_vector();

        // Catch a degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = record.normal;
        }

        Some((
            self.albedo,
            Ray::new(record.p, scatter_direction, Some(ray.time)),
        ))
    }
}
