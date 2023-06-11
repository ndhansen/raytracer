use std::cmp::Ordering;

use rand::Rng;

use crate::scene::{
    bounding_box::AxisAlignedBoundingBox, hit_record::HitRecord, hittable::Hittable,
};

use super::ray::Ray;

pub struct BvhNode {
    left: Option<Box<dyn Hittable>>,
    right: Option<Box<dyn Hittable>>,
    bounding_box: AxisAlignedBoundingBox,
}

impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.bounding_box.hit(ray, t_min, t_max) {
            return None;
        }

        let left_hit = match &self.left {
            Some(hittable) => hittable.hit(ray, t_min, t_max),
            _ => None,
        };
        let left_time = match left_hit {
            Some(rec) => rec.t,
            None => t_max,
        };

        let right_hit = match &self.right {
            Some(hittable) => hittable.hit(ray, t_min, left_time),
            _ => None,
        };

        if right_hit.is_some() {
            return right_hit;
        }
        left_hit
    }

    fn bounding_box(&self, _time_0: f64, _time_1: f64) -> Option<AxisAlignedBoundingBox> {
        Some(self.bounding_box)
    }
}

impl BvhNode {
    pub fn new(mut src_objects: Vec<Box<dyn Hittable>>, time_0: f64, time_1: f64) -> BvhNode {
        if src_objects.len() == 0 {
            panic!("Can't construct BVH node without anything in it.")
        }

        let mut generator = rand::thread_rng();
        let axis = generator.gen_range(0..2);

        let (left, right) = if src_objects.len() == 1 {
            // If we have one element, put it in the left
            (Some(src_objects.remove(0)), None)
        } else if src_objects.len() == 2 {
            // If we have two elements, put one in each side
            let compare_results = BvhNode::box_compare(&src_objects[0], &src_objects[1], axis);
            let second_item = src_objects.remove(1);
            let first_item = src_objects.remove(0);
            if compare_results.is_lt() {
                (Some(first_item), Some(second_item))
            } else {
                (Some(second_item), Some(first_item))
            }
        } else {
            // If we have more than two elements, split the vector and call recursively
            src_objects.sort_by(|left, right| BvhNode::box_compare(left, right, axis));

            let mid = src_objects.len() / 2;
            let right_objects = src_objects.split_off(mid);
            let left_objects = src_objects;
            (
                Some(Box::new(BvhNode::new(left_objects, time_0, time_1)) as Box<dyn Hittable>),
                Some(Box::new(BvhNode::new(right_objects, time_0, time_1)) as Box<dyn Hittable>),
            )
        };

        let left_box = match &left {
            Some(left) => left.bounding_box(time_0, time_1),
            _ => None,
        };
        let right_box = match &right {
            Some(right) => right.bounding_box(time_0, time_1),
            _ => None,
        };
        let surrounding_box = match (left_box, right_box) {
            (Some(left_box), Some(right_box)) => {
                AxisAlignedBoundingBox::surrounding_box(&left_box, &right_box)
            }
            (Some(left_box), None) => left_box,
            (None, Some(right_box)) => right_box,
            (None, None) => panic!("Tree didn't contain left or right node."),
        };
        BvhNode {
            left,
            right,
            bounding_box: surrounding_box,
        }
    }

    fn box_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>, axis: usize) -> Ordering {
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

#[cfg(test)]
mod tests {
    use crate::{
        geometry::vector_3d::Vector3D,
        scene::{materials::Dielectric, sphere::Sphere},
    };

    use super::*;

    #[test]
    #[should_panic]
    fn cant_construct_empty_bvh() {
        BvhNode::new(vec![], 0.0, 1.0);
    }

    /// Given a sphere and a ray pointing at that sphere, check that we get a hit
    #[test]
    fn can_hit_sinlge_sphere() {
        let hittables: Vec<Box<dyn Hittable>> = vec![Box::new(Sphere::new(
            Vector3D::new(0.0, 0.0, 5.0),
            1.0,
            Box::new(Dielectric::new(1.0)),
        ))];
        let bvh = BvhNode::new(hittables, 0.0, 1.0);
        let ray = Ray::new(
            Vector3D::new(0.0, 0.0, 0.0),
            Vector3D::new(0.0, 0.0, 10.0),
            Some(0.0),
        );
        assert!(bvh.hit(&ray, 0.0, 1.0).is_some());
    }

    /// Test that we can't a sphere that is in the direction of the ray, but outside the time
    /// window
    #[test]
    fn cant_hit_sinlge_sphere_outside_time_window() {
        let hittables: Vec<Box<dyn Hittable>> = vec![Box::new(Sphere::new(
            Vector3D::new(0.0, 0.0, 5.0),
            1.0,
            Box::new(Dielectric::new(1.0)),
        ))];
        let bvh = BvhNode::new(hittables, 0.0, 1.0);
        let ray = Ray::new(
            Vector3D::new(0.0, 0.0, 0.0),
            Vector3D::new(0.0, 0.0, 1.0),
            Some(0.0),
        );
        assert!(bvh.hit(&ray, 0.0, 1.0).is_none());
    }

    #[test]
    fn can_hit_multiple() {
        let hittables: Vec<Box<dyn Hittable>> = vec![
            Box::new(Sphere::new(
                Vector3D::new(0.0, -2.0, 5.0),
                1.0,
                Box::new(Dielectric::new(1.0)),
            )),
            Box::new(Sphere::new(
                Vector3D::new(0.0, 2.0, 5.0),
                1.0,
                Box::new(Dielectric::new(1.0)),
            )),
            Box::new(Sphere::new(
                Vector3D::new(2.0, 0.0, 5.0),
                1.0,
                Box::new(Dielectric::new(1.0)),
            )),
            Box::new(Sphere::new(
                Vector3D::new(-2.0, 0.0, 5.0),
                1.0,
                Box::new(Dielectric::new(1.0)),
            )),
        ];
        let bvh = BvhNode::new(hittables, 0.0, 1.0);

        let ray = Ray::new(
            Vector3D::new(0.0, 0.0, 0.0),
            Vector3D::new(0.0, 4.0, 10.0),
            Some(0.0),
        );
        assert!(bvh.hit(&ray, 0.0, 1.0).is_some());

        let ray = Ray::new(
            Vector3D::new(0.0, 0.0, 0.0),
            Vector3D::new(0.0, -4.0, 10.0),
            Some(0.0),
        );
        assert!(bvh.hit(&ray, 0.0, 1.0).is_some());

        let ray = Ray::new(
            Vector3D::new(0.0, 0.0, 0.0),
            Vector3D::new(4.0, 0.0, 10.0),
            Some(0.0),
        );
        assert!(bvh.hit(&ray, 0.0, 1.0).is_some());

        let ray = Ray::new(
            Vector3D::new(0.0, 0.0, 0.0),
            Vector3D::new(-4.0, 0.0, 10.0),
            Some(0.0),
        );
        assert!(bvh.hit(&ray, 0.0, 1.0).is_some());
    }

    #[test]
    fn cant_hit_between_multiple() {
        let hittables: Vec<Box<dyn Hittable>> = vec![
            Box::new(Sphere::new(
                Vector3D::new(0.0, -2.0, 5.0),
                1.0,
                Box::new(Dielectric::new(1.0)),
            )),
            Box::new(Sphere::new(
                Vector3D::new(0.0, 2.0, 5.0),
                1.0,
                Box::new(Dielectric::new(1.0)),
            )),
        ];
        let bvh = BvhNode::new(hittables, 0.0, 1.0);
        let ray = Ray::new(
            Vector3D::new(0.0, 0.0, 0.0),
            Vector3D::new(0.0, 0.0, 10.0),
            Some(0.0),
        );
        assert!(bvh.hit(&ray, 0.0, 1.0).is_none());
    }
}
