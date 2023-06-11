use crate::{
    geometry::{ray::Ray, vector_3d::Vector3D},
    scene::{
        hit_record::HitRecord,
        textures::{SolidColorTexture, Texture},
    },
    util::color::Color,
};

use super::Material;

pub struct Lambertian<'a> {
    albedo: Box<dyn Texture + 'a>,
}

impl<'a> Lambertian<'a> {
    pub fn new(albedo: Color) -> Lambertian<'a> {
        Lambertian {
            albedo: Box::new(SolidColorTexture::new(albedo)),
        }
    }
}

impl<'a> Material for Lambertian<'a> {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = record.normal + Vector3D::random_unit_vector();

        // Catch a degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = record.normal;
        }

        let attenuation = self.albedo.color(record.u, record.v, &record.p);

        Some((
            attenuation,
            Ray::new(record.p, scatter_direction, Some(ray.time)),
        ))
    }
}
