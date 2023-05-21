use crate::{geometry::ray::Ray, util::point::Point3D};

pub struct AxisAlignedBoundingBox {
    minimum: Point3D,
    maximum: Point3D,
}

impl AxisAlignedBoundingBox {
    pub fn new(minimum: Point3D, maximum: Point3D) -> AxisAlignedBoundingBox {
        AxisAlignedBoundingBox { minimum, maximum }
    }

    pub fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> bool {
        for axis in 0..3 {
            let t_0 = f64::min(
                (self.minimum[axis] - ray.origin[axis]) / ray.direction[axis],
                (self.maximum[axis] - ray.origin[axis]) / ray.direction[axis],
            );
            let t_1 = f64::max(
                (self.minimum[axis] - ray.origin[axis]) / ray.direction[axis],
                (self.maximum[axis] - ray.origin[axis]) / ray.direction[axis],
            );
            t_min = f64::max(t_0, t_min);
            t_max = f64::min(t_1, t_max);
            if t_max < t_min {
                return false;
            }
        }
        true
    }
}
