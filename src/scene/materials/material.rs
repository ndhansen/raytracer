use crate::{geometry::ray::Ray, scene::hit_record::HitRecord, util::color::Color};

pub trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Color, Ray)>;
}
