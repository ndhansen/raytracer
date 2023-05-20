use core::fmt;
use std::ops;

use approx::{abs_diff_eq, relative_eq, AbsDiffEq, RelativeEq};
use rand::Rng;

/// A 3D point
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vector3D {
    /// A 3D vector must have 3 points
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3D {
    /// Returns a new point around the center of the coordiante space.
    pub fn empty() -> Vector3D {
        Vector3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    /// Creates a new point given all coordinates in space.
    pub fn new(x: f64, y: f64, z: f64) -> Vector3D {
        Vector3D { x, y, z }
    }

    /// Generate a random vector in a cube.
    pub fn random(min: f64, max: f64) -> Vector3D {
        let mut generator = rand::thread_rng();
        Vector3D {
            x: generator.gen_range(min..max),
            y: generator.gen_range(min..max),
            z: generator.gen_range(min..max),
        }
    }

    /// Generate a random vector in a cube spanning from 0 to 1.
    pub fn all_random() -> Vector3D {
        Vector3D::random(0.0, 1.0)
    }

    /// Generate a random vector in a sphere.
    pub fn random_in_unit_sphere() -> Vector3D {
        loop {
            let vec = Vector3D::random(-1.0, 1.0);
            if vec.length_squared() >= 1.0 {
                continue;
            }
            return vec;
        }
    }

    pub fn random_unit_vector() -> Vector3D {
        unit_vector(&Vector3D::random_in_unit_sphere())
    }

    /// Generate a random vector in a circle.
    pub fn random_in_unit_disk() -> Vector3D {
        loop {
            let mut generator = rand::thread_rng();
            let random_vector = Vector3D::new(
                generator.gen_range(-1.0..1.0),
                generator.gen_range(-1.0..1.1),
                0.0,
            );
            if random_vector.length_squared() >= 1.0 {
                continue;
            }
            return random_vector;
        }
    }

    /// Get the x-axis coordinate
    pub fn x(&self) -> f64 {
        self.x
    }

    /// Get the y-axis coordinate
    pub fn y(&self) -> f64 {
        self.y
    }

    /// Get the z-axis coordinate
    pub fn z(&self) -> f64 {
        self.z
    }

    /// Get the squared distance from the centre of coordinate space
    pub fn length_squared(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    /// Gets the distance between the center of the coordianate space and the vector
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    /// Calculate the dot product of the vector
    pub fn dot(&self, second_vector: &Vector3D) -> f64 {
        dot(self, second_vector)
    }

    /// Calcualte the cross product of two vectors
    pub fn cross(&self, second_vector: &Vector3D) -> Vector3D {
        cross(self, second_vector)
    }

    /// Returns zero if the vector is close to zero in all dimensions
    pub fn near_zero(&self) -> bool {
        const MARGIN: f64 = 1e-8;
        (self.x.abs() < MARGIN) && (self.y.abs() < MARGIN) && (self.z.abs() < MARGIN)
    }

    pub fn reflect(&self, other: &Vector3D) -> Vector3D {
        *self - (2.0 * self.dot(other) * other)
    }

    pub fn refract(&self, other: &Vector3D, etai_over_etat: f64) -> Vector3D {
        let cos_theta = (-self).dot(other).min(1.0);
        let r_out_perp = etai_over_etat * (self + (cos_theta * other));
        let r_out_parallel = -((1.0 - r_out_perp.length_squared()).abs().sqrt()) * other;
        r_out_perp + r_out_parallel
    }
}

pub fn unit_vector(vec: &Vector3D) -> Vector3D {
    vec / vec.length()
}

/// Calculate the dot product of the vector
pub fn dot(first: &Vector3D, second: &Vector3D) -> f64 {
    first.x * second.x + first.y * second.y + first.z * second.z
}

/// Calcualte the cross product of two vectors
pub fn cross(first: &Vector3D, second: &Vector3D) -> Vector3D {
    Vector3D {
        x: first.y * second.z - first.z * second.y,
        y: first.z * second.x - first.x * second.z,
        z: first.x * second.y - first.y * second.x,
    }
}

impl fmt::Display for Vector3D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl AbsDiffEq for Vector3D {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        f64::EPSILON
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        abs_diff_eq!(self.x, other.x, epsilon = epsilon)
            && abs_diff_eq!(self.y, other.y, epsilon = epsilon)
            && abs_diff_eq!(self.z, other.z, epsilon = epsilon)
    }
}

impl RelativeEq for Vector3D {
    fn default_max_relative() -> Self::Epsilon {
        f64::EPSILON
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        relative_eq!(
            self.x,
            other.x,
            epsilon = epsilon,
            max_relative = max_relative
        ) && relative_eq!(
            self.y,
            other.y,
            epsilon = epsilon,
            max_relative = max_relative
        ) && relative_eq!(
            self.z,
            other.z,
            epsilon = epsilon,
            max_relative = max_relative
        )
    }
}

// Mathematical operations

impl ops::Add<Vector3D> for Vector3D {
    type Output = Vector3D;

    fn add(self, rhs: Vector3D) -> Self::Output {
        &self + rhs
    }
}

impl ops::Add<Vector3D> for &Vector3D {
    type Output = Vector3D;

    fn add(self, rhs: Vector3D) -> Self::Output {
        Vector3D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Neg for Vector3D {
    type Output = Vector3D;

    fn neg(self) -> Self::Output {
        -&self
    }
}

impl ops::Neg for &Vector3D {
    type Output = Vector3D;

    fn neg(self) -> Self::Output {
        Vector3D {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::Sub<Vector3D> for Vector3D {
    type Output = Vector3D;

    fn sub(self, rhs: Vector3D) -> Self::Output {
        Vector3D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Mul<Vector3D> for Vector3D {
    type Output = Vector3D;

    fn mul(self, rhs: Vector3D) -> Self::Output {
        Vector3D {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl ops::Mul<f64> for Vector3D {
    type Output = Vector3D;

    fn mul(self, rhs: f64) -> Self::Output {
        &self * rhs
    }
}

impl ops::Mul<f64> for &Vector3D {
    type Output = Vector3D;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector3D {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Mul<Vector3D> for f64 {
    type Output = Vector3D;

    fn mul(self, rhs: Vector3D) -> Self::Output {
        &rhs * self
    }
}

impl ops::Mul<&Vector3D> for f64 {
    type Output = Vector3D;

    fn mul(self, rhs: &Vector3D) -> Self::Output {
        rhs * self
    }
}

impl ops::Div<f64> for Vector3D {
    type Output = Vector3D;

    fn div(self, rhs: f64) -> Self::Output {
        &self / rhs
    }
}

impl ops::Div<f64> for &Vector3D {
    type Output = Vector3D;

    fn div(self, rhs: f64) -> Self::Output {
        (1 as f64 / rhs) * self
    }
}

// Assignment operations

impl ops::AddAssign<&Vector3D> for Vector3D {
    fn add_assign(&mut self, rhs: &Vector3D) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::MulAssign<f64> for Vector3D {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl ops::DivAssign<f64> for Vector3D {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs
    }
}

// Array access

impl ops::Index<usize> for Vector3D {
    type Output = f64;

    /// Get a part of the point by index
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index in to 3D vector out of bounds!"),
        }
    }
}

impl ops::IndexMut<usize> for Vector3D {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index in to 3D vector out of bounds!"),
        }
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn it_has_properties() {
        let vec = Vector3D::new(0.2, 0.1, -5.1);
        assert_eq!(0.2, vec.x());
        assert_eq!(0.1, vec.y());
        assert_eq!(-5.1, vec.z());
    }

    #[test]
    fn new_empty_properties_are_zero() {
        let vec = Vector3D::empty();
        let expected = Vector3D::new(0.0, 0.0, 0.0);
        assert_relative_eq!(expected, vec);
    }

    #[test]
    fn can_index_in_to_vector() {
        let vec = Vector3D::new(0.2, 0.1, -5.1);
        assert_eq!(vec[0], vec.x());
        assert_eq!(vec[1], vec.y());
        assert_eq!(vec[2], vec.z());
    }

    #[test]
    fn can_index_and_change_vector() {
        let mut vec = Vector3D::empty();
        vec[0] = 2.1;
        assert_eq!(2.1, vec[0]);
        vec[1] = -2.1;
        assert_eq!(-2.1, vec[1]);
        vec[2] = 0.1;
        assert_eq!(0.1, vec[2]);
    }

    #[test]
    fn length_squared_of_vector() {
        let vec = Vector3D::new(1.0, 2.0, 3.0);
        assert_eq!(14.0, vec.length_squared())
    }

    #[test]
    fn length_of_vector() {
        let vec = Vector3D::new(0.0, 3.0, 4.0);
        assert_eq!(5.0, vec.length());
    }

    #[test]
    fn add_two_vectors() {
        let vec_one = Vector3D::new(-2.0, 3.0, 1.4);
        let vec_two = Vector3D::new(2.1, -1.0, -0.5);
        let sum = vec_one + vec_two;
        let expected = Vector3D::new(0.1, 2.0, 0.9);
        assert_relative_eq!(expected, sum);
    }

    #[test]
    fn subtract_two_vectors() {
        let vec_one = Vector3D::new(-2.0, 3.0, 1.4);
        let vec_two = Vector3D::new(2.1, -1.0, -0.5);
        let sub = vec_one - vec_two;
        let expected = Vector3D::new(-4.1, 4.0, 1.9);
        assert_relative_eq!(expected, sub);
    }

    #[test]
    fn negate_a_vector() {
        let vec_one = Vector3D::new(-2.0, 3.0, 1.4);
        let neg = -vec_one;
        let expected = Vector3D::new(2.0, -3.0, -1.4);
        assert_relative_eq!(expected, neg);
    }

    #[test]
    fn multiply_vector_by_vector() {
        let vec_one = Vector3D::new(-2.0, 3.0, 1.4);
        let vec_multiplier = Vector3D::new(0.5, 1.0, 3.0);
        let multiplied = vec_one * vec_multiplier;
        let expected = Vector3D {
            x: -1.0,
            y: 3.0,
            z: 4.2,
        };
        assert_relative_eq!(expected, multiplied);
    }

    #[test]
    fn multiply_vector_by_scalar() {
        let vec = Vector3D::new(-2.0, 0.0, 1.3);
        let multiplied = vec * 2.0;
        let expected = Vector3D::new(-4.0, 0.0, 2.6);
        assert_relative_eq!(expected, multiplied);
    }

    #[test]
    fn multiply_scalar_by_vector() {
        let vec = Vector3D::new(-2.0, 0.0, 1.3);
        let multiplied = 2.0 * vec;
        let expected = Vector3D::new(-4.0, 0.0, 2.6);
        assert_relative_eq!(expected, multiplied);
    }

    #[test]
    fn divide_vector_by_scalar() {
        let vec = Vector3D::new(-2.0, 0.0, 1.3);
        let divided = &vec / 2.0;
        let expected = Vector3D::new(-1.0, 0.0, 0.65);
        assert_relative_eq!(expected, divided);
    }

    #[test]
    fn add_and_assign_vector_by_vector() {
        let mut vec = Vector3D::new(-2.0, 0.0, 1.3);
        let vec_to_add = Vector3D::new(3.0, -1.0, 0.5);
        vec += &vec_to_add;
        let expected = Vector3D::new(1.0, -1.0, 1.8);
        assert_relative_eq!(expected, vec);
    }

    #[test]
    fn multiply_and_assign_vector_by_scalar() {
        let mut vec = Vector3D::new(-2.0, 0.0, 1.3);
        vec *= 2.0;
        let expected = Vector3D::new(-4.0, 0.0, 2.6);
        assert_relative_eq!(expected, vec);
    }

    #[test]
    fn divide_and_assign_vector_by_scalar() {
        let mut vec = Vector3D::new(-2.0, 0.0, 1.3);
        vec /= 2.0;
        let expected = Vector3D::new(-1.0, 0.0, 0.65);
        assert_relative_eq!(expected, vec);
    }

    #[test]
    fn dot_product_with_another_vector() {
        let vec_one = Vector3D::new(1.3, -2.4, 5.0);
        let vec_two = Vector3D::new(0.2, 1.0, -1.5);
        let dot_product = vec_one.dot(&vec_two);
        assert_relative_eq!(-9.64, dot_product);
    }

    #[test]
    fn cross_product_with_another_vector() {
        let vec_one = Vector3D::new(1.3, -2.4, 5.0);
        let vec_two = Vector3D::new(0.2, 1.0, -1.5);
        let cross_product = vec_one.cross(&vec_two);
        let expected = Vector3D::new(-1.4, 2.95, 1.78);
        assert_relative_eq!(expected, cross_product, epsilon = 0.001);
    }

    #[test]
    fn unit_vector_from_vector() {
        let vec = Vector3D::new(1.3, -2.4, 5.0);
        let normalized = unit_vector(&vec);
        let expected = Vector3D::new(0.22821, -0.42131, 0.87773);
        assert_relative_eq!(expected, normalized, epsilon = 0.001);
    }
}
