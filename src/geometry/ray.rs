use crate::util::point::Point3D;

use super::vector_3d::Vector3D;

#[derive(Debug, PartialEq)]
pub struct Ray {
    pub origin: Point3D,
    pub direction: Vector3D,
}

impl Ray {
    pub fn new(origin: Point3D, direction: Vector3D) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3D {
        self.origin + (t * &self.direction)
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn ray_anywhere_when_origin_and_direction_same() {
        let origin = Point3D::new(1.4, 0.1, -4.2);
        let direction = Vector3D::new(0.0, 0.0, 0.0);
        let ray = Ray::new(origin, direction);

        let expected = Vector3D::new(1.4, 0.1, -4.2);
        let actual = ray.at(0.0);
        assert_relative_eq!(expected, actual);

        let actual = ray.at(1.0);
        assert_relative_eq!(expected, actual);

        let actual = ray.at(0.33);
        assert_relative_eq!(expected, actual);
    }

    #[test]
    fn ray_works_with_any_points() {
        let origin = Point3D::empty();
        let direction = Vector3D::new(2.0, 2.0, 2.0);
        let ray = Ray::new(origin, direction);

        let expected = Vector3D::new(1.0, 1.0, 1.0);
        let actual = ray.at(0.5);
        assert_relative_eq!(expected, actual);
    }

    #[test]
    fn ray_works_at_endpoints() {
        let origin = Point3D::empty();
        let direction = Vector3D::new(1.4, 0.1, -4.2);
        let ray = Ray::new(origin, direction);

        let actual = ray.at(1.0);
        assert_relative_eq!(direction, actual);

        let actual = ray.at(0.0);
        assert_relative_eq!(origin, actual);
    }

    #[test]
    fn ray_gives_same_result_at_midpoint_when_variables_swapped() {
        let origin = Point3D::empty();
        let direction = Vector3D::new(2.0, 2.0, 2.0);
        let ray = Ray::new(origin, direction);

        let expected = Vector3D::new(1.0, 1.0, 1.0);
        let actual = ray.at(0.5);
        assert_relative_eq!(expected, actual);

        let ray = Ray::new(direction, -direction);
        let actual = ray.at(0.5);
        assert_relative_eq!(expected, actual);
    }
}
