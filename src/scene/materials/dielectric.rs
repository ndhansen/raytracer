use crate::{
    geometry::{ray::Ray, vector_3d},
    scene::hit_record::HitRecord,
    util::color::Color,
};
use rand::Rng;

use super::Material;

pub struct Dielectric {
    index_of_refraction: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Dielectric {
        Dielectric {
            index_of_refraction,
        }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let r_zero = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r_zero = r_zero.powi(2);
        r_zero + (1.0 - r_zero) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Color, Ray)> {
        let refraction_ratio = if record.front_face {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };
        let unit_direction = vector_3d::unit_vector(&ray.direction);
        let cos_theta = (-unit_direction).dot(&record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let mut generator = rand::thread_rng();
        let random_double = generator.gen_range(0.0..1.0);
        let reflectance = Dielectric::reflectance(cos_theta, refraction_ratio);
        let direction = if cannot_refract || reflectance > random_double {
            // eprintln!("reflected with {} vs {} random! (cannot_refract: {})", reflectance, random_double, cannot_refract);
            unit_direction.reflect(&record.normal)
        } else {
            unit_direction.refract(&record.normal, refraction_ratio)
        };

        Some((
            Color::new(1.0, 1.0, 1.0),
            Ray::new(record.p, direction, Some(ray.time)),
        ))
    }
}
