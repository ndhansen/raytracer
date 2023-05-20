use crate::{geometry::ray::Ray, util::point::Point3D};

use super::{
    hit_record::HitRecord, hittable::Hittable, materials::Material, moving_sphere::MovingSphere,
};

/// A static sphere. Internally a moving sphere that starts and ends in the same place.
pub struct Sphere {
    sphere: MovingSphere,
}

impl Sphere {
    pub fn new(center: Point3D, radius: f64, material: Box<dyn Material>) -> Sphere {
        Sphere {
            sphere: MovingSphere::new(center, center, radius, material, 0.0, 1.1),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sphere.hit(ray, t_min, t_max)
    }
}
