use crate::{
    geometry::{ray::Ray, vector_3d},
    scene::hit_record::HitRecord,
    util::color::Color,
};

use super::Material;

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: &Color) -> Metal {
        Metal { albedo: *albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = vector_3d::unit_vector(&ray.direction).reflect(&record.normal);
        let scattered = Ray::new(record.p, reflected);
        let attenuation = self.albedo;
        if vector_3d::dot(&scattered.direction, &record.normal) > 0.0 {
            return Some((attenuation, scattered));
        }
        None
    }
}
