use crate::{
    geometry::{
        ray::Ray,
        vector_3d::{self, Vector3D},
    },
    scene::materials::Material,
    util::point::Point3D,
};

#[derive(Clone, Copy)]
pub struct HitRecord<'a> {
    pub p: Point3D,
    pub normal: Vector3D,
    pub material: &'a dyn Material,
    pub t: f64, // Time
    pub u: f64, // Location of hit on object
    pub v: f64, // Locaiton of hit on object
    pub front_face: bool,
}

impl<'a> HitRecord<'a> {
    pub fn new(
        p: Point3D,
        normal: Vector3D,
        material: &'a dyn Material,
        t: f64,
        u: f64,
        v: f64,
        ray: &Ray,
    ) -> HitRecord<'a> {
        let front_face = ray.direction.dot(&normal) < 0.0;
        HitRecord {
            p,
            normal: if front_face { normal } else { -normal },
            t,
            u,
            v,
            front_face,
            material,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vector3D) {
        self.front_face = vector_3d::dot(&ray.direction, outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}
