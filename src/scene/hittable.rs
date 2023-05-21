use crate::geometry::ray::Ray;

use super::{bounding_box::AxisAlignedBoundingBox, hit_record::HitRecord};

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time_0: f64, time_1: f64) -> Option<AxisAlignedBoundingBox>;
}

impl Hittable for Vec<Box<dyn Hittable>> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;

        let mut temp_record: Option<HitRecord> = None;
        for object in self.iter() {
            if let Some(record) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = record.t;
                temp_record = Some(record)
            }
        }

        return temp_record;
    }

    fn bounding_box(&self, time_0: f64, time_1: f64) -> Option<AxisAlignedBoundingBox> {
        let bounding_boxes: Option<Vec<AxisAlignedBoundingBox>> = self
            .iter()
            .map(|object| object.bounding_box(time_0, time_1))
            .collect();
        bounding_boxes?
            .into_iter()
            .reduce(|box_0, box_1| AxisAlignedBoundingBox::surrounding_box(&box_0, &box_1))
    }
}
