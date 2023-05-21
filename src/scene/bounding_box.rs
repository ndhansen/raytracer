use crate::{geometry::ray::Ray, util::point::Point3D};

pub struct AxisAlignedBoundingBox {
    minimum: Point3D,
    maximum: Point3D,
}

impl AxisAlignedBoundingBox {
    pub fn new(minimum: Point3D, maximum: Point3D) -> AxisAlignedBoundingBox {
        AxisAlignedBoundingBox { minimum, maximum }
    }

    pub fn surrounding_box(
        first_box: &AxisAlignedBoundingBox,
        second_box: &AxisAlignedBoundingBox,
    ) -> AxisAlignedBoundingBox {
        let small = Point3D::new(
            f64::min(first_box.minimum.x(), second_box.minimum.x()),
            f64::min(first_box.minimum.y(), second_box.minimum.y()),
            f64::min(first_box.minimum.z(), second_box.minimum.z()),
        );
        let big = Point3D::new(
            f64::max(first_box.maximum.x(), second_box.maximum.x()),
            f64::max(first_box.maximum.y(), second_box.maximum.y()),
            f64::max(first_box.maximum.z(), second_box.maximum.z()),
        );
        AxisAlignedBoundingBox {
            minimum: small,
            maximum: big,
        }
    }

    /// Checks if a ray intersects this bounding box between times t_min
    /// and t_max.
    ///
    /// For example, if the ray intersects the plane between time 4 and 6,
    /// but we have bounds of t_min and t_max of 1 and 3, then it has not
    /// gone through the bounding box, despite the fact that it will
    /// eventually pass through it.
    ///
    /// It does this by going through each axis, checking at which times
    /// the ray intersects with the axis. If the starting time of the
    /// intersection is after the last allowable time (t_max), then we
    /// know it does not intersect the axis in time. If it does not
    /// intersect one axis, we know it can't possibly intersect the
    /// plane between t_min and t_max.
    ///
    /// # Arguments
    ///
    /// * `ray` - The ray for which we want to check if it passes through the line.
    /// * `t_min` - The start time for which we want to check an intersection.
    /// * `t_max` - The end time for which we want to check if we have an
    /// intersection.
    pub fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> bool {
        for axis in 0..3 {
            let intersection_0 = (self.minimum[axis] - ray.origin[axis]) / ray.direction[axis];
            let intersection_1 = (self.maximum[axis] - ray.origin[axis]) / ray.direction[axis];
            let t_0 = f64::min(intersection_0, intersection_1);
            let t_1 = f64::max(intersection_0, intersection_1);
            let t_min = f64::max(t_0, t_min);
            let t_max = f64::min(t_1, t_max);
            if t_max < t_min {
                return false;
            }
        }
        true
    }

    /// Same as `hit`, but possibly faster? I have my doubts.
    pub fn optimized_hit(&self, ray: Ray, t_min: f64, t_max: f64) -> bool {
        for axis in 0..3 {
            let inverted_direction = 1.0 / ray.direction[axis];
            let mut t_0 = (self.minimum[axis] - ray.origin[axis]) * inverted_direction;
            let mut t_1 = (self.maximum[axis] - ray.origin[axis]) * inverted_direction;
            if inverted_direction < 0.0 {
                std::mem::swap(&mut t_0, &mut t_1);
            }
            let t_min = if t_0 > t_min { t_0 } else { t_min };
            let t_max = if t_1 < t_max { t_1 } else { t_max };
            if t_max < t_min {
                return false;
            }
        }
        true
    }
}
