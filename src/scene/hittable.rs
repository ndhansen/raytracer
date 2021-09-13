use crate::{geometry::{ray::Ray, vector_3d::{self, Vector3D}}, util::point::Point3D};

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub p: Point3D,
    pub normal: Vector3D,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn empty() -> HitRecord {
        HitRecord {
            p: Point3D::empty(),
            normal: Vector3D::empty(),
            t: 0.0,
            front_face: true,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vector3D) {
        self.front_face = vector_3d::dot(&ray.direction, outward_normal) < 0.0;
        self.normal = if self.front_face { *outward_normal } else { -*outward_normal};
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool;
}