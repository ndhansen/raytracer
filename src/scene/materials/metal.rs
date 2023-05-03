use crate::{
    geometry::{
        ray::Ray,
        vector_3d::{self, Vector3D},
    },
    scene::hit_record::HitRecord,
    util::color::Color,
};

use super::Material;

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: &Color, fuzz: f64) -> Metal {
        let clamped_fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };
        Metal {
            albedo: *albedo,
            fuzz: clamped_fuzz,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = vector_3d::unit_vector(&ray.direction).reflect(&record.normal);
        let scattered = Ray::new(
            record.p,
            reflected + self.fuzz * Vector3D::random_in_unit_sphere(),
            Some(ray.time),
        );
        let attenuation = self.albedo;
        if vector_3d::dot(&scattered.direction, &record.normal) > 0.0 {
            return Some((attenuation, scattered));
        }
        None
    }
}
