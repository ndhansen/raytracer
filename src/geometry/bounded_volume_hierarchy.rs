use std::cmp::Ordering;

use rand::Rng;

use crate::scene::{
    bounding_box::AxisAlignedBoundingBox,
    hit_record::HitRecord,
    hittable::{self, Hittable},
};

use super::ray::Ray;

pub struct BvhNode {
    left: Box<dyn Hittable>,
    right: Box<dyn Hittable>,
    bounding_box: AxisAlignedBoundingBox,
}

impl BvhNode {
    pub fn new(
        src_objects: &Vec<Box<dyn Hittable>>,
        start: usize,
        end: usize,
        time_0: f64,
        time_1: f64,
    ) -> BvhNode {
        let mut generator = rand::thread_rng();
        let axis = generator.gen_range(0..2);
        let object_span = end - start;

        let (left, right) = if object_span == 1 {
            (src_objects[start], src_objects[start])
        } else if object_span == 2 {
            if BvhNode::box_compare(src_objects[start], src_objects[start + 1], axis) {
                (src_objects[start], src_objects[start + 1])
            } else {
                (src_objects[start + 1], src_objects[start])
            }
        } else {
            src_objects.sort_by(|left, right| BvhNode::box_compare(left, right, axis));

            let mid = start + object_span / 2;
            (
                Box::new(BvhNode::new(src_objects, start, mid, time_0, time_1)),
                Box::new(BvhNode::new(src_objects, mid, end, time_0, time_1)),
            )
        };

        let left_box = left.bounding_box(time_0, time_1);
        let right_box = right.bounding_box(time_0, time_1);
    }

    fn box_compare(a: Box<dyn Hittable>, b: Box<dyn Hittable>, axis: usize) -> Ordering {
        let left_box = a.bounding_box(0.0, 0.0).unwrap();
        let right_box = b.bounding_box(0.0, 0.0).unwrap();

        if left_box.minimum[axis] < right_box.minimum[axis] {
            return Ordering::Less;
        } else if left_box.minimum[axis] > right_box.minimum[axis] {
            return Ordering::Greater;
        }
        Ordering::Equal
    }
}

impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.bounding_box.hit(ray, t_min, t_max) {
            return None;
        }

        let left_hit = self.left.hit(ray, t_min, t_max);
        let left_time = match left_hit {
            Some(rec) => rec.t,
            None => t_max,
        };
        let right_hit = self.right.hit(ray, t_min, left_time);

        if right_hit.is_some() {
            return right_hit;
        }
        left_hit
    }

    fn bounding_box(&self, time_0: f64, time_1: f64) -> Option<AxisAlignedBoundingBox> {
        Some(self.bounding_box)
    }
}
