use crate::{geometry::{ray::Ray, vector_3d}, util::point::Point3D};

use super::{hit_record::HitRecord, hittable::{Hittable}, materials::Material};

pub struct Sphere {
    center: Point3D,
    radius: f64,
    material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3D, radius: f64, material: Box<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = vector_3d::dot(&oc, &ray.direction);
        let c = oc.length_squared() - (self.radius * self.radius);

        let discriminant = (half_b * half_b) - (a * c);
        if discriminant < 0.0 {
            return None;
        }
        let discriminant_root = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - discriminant_root) / a;
        if root < t_min || t_max < root {
            root = (-half_b + discriminant_root) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let point = ray.at(root);
        let outward_normal = (point - self.center) / self.radius;
        let record = HitRecord::new(
            ray.at(root),
            outward_normal,
            &*self.material,
            root,
            &ray
        );

        Some(record)
    }
}
