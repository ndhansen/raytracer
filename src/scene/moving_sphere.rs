use std::f64::consts::PI;

use crate::{
    geometry::{
        ray::Ray,
        vector_3d::{self, Vector3D},
    },
    util::point::Point3D,
};

use super::{
    bounding_box::AxisAlignedBoundingBox, hit_record::HitRecord, hittable::Hittable,
    materials::Material,
};

pub struct MovingSphere {
    start_center: Point3D,
    end_center: Point3D,
    radius: f64,
    material: Box<dyn Material>,
    start_time: f64,
    end_time: f64,
}

impl MovingSphere {
    pub fn new(
        start_center: Point3D,
        end_center: Point3D,
        radius: f64,
        material: Box<dyn Material>,
        start_time: f64,
        end_time: f64,
    ) -> MovingSphere {
        MovingSphere {
            start_center,
            end_center,
            radius,
            material,
            start_time,
            end_time,
        }
    }

    fn center(&self, time: f64) -> Point3D {
        self.start_center
            + ((time - self.start_time) / (self.end_time - self.start_time))
                * (self.end_center - self.start_center)
    }

    fn get_sphere_uv(point: &Point3D) -> (f64, f64) {
        let theta = (-point.y()).acos();
        let phi = (-point.z()).atan2(point.x()) + PI;

        let u = phi / (2.0 * PI);
        let v = theta / PI;
        (u, v)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center(ray.time);
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
        let outward_normal = (point - self.center(ray.time)) / self.radius;
        let (u, v) = MovingSphere::get_sphere_uv(&outward_normal);
        let record = HitRecord::new(
            ray.at(root),
            outward_normal,
            &*self.material,
            root,
            u,
            v,
            &ray,
        );

        Some(record)
    }

    fn bounding_box(&self, time_0: f64, time_1: f64) -> Option<AxisAlignedBoundingBox> {
        let start_box = AxisAlignedBoundingBox::new(
            self.center(time_0) - Vector3D::new(self.radius, self.radius, self.radius),
            self.center(time_1) + Vector3D::new(self.radius, self.radius, self.radius),
        );
        let end_box = AxisAlignedBoundingBox::new(
            self.center(time_1) - Vector3D::new(self.radius, self.radius, self.radius),
            self.center(time_1) + Vector3D::new(self.radius, self.radius, self.radius),
        );
        Some(AxisAlignedBoundingBox::surrounding_box(
            &start_box, &end_box,
        ))
    }
}
