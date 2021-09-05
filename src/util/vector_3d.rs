use core::fmt;
use std::ops;

/// A 3D point
#[derive(Debug, PartialEq)]
pub struct Vector3D {
    /// A 3D vector must have 3 points
    pub points: (f64, f64, f64),
}

impl Vector3D {
    /// Returns a new point around the center of the coordiante space.
    pub fn empty() -> Vector3D {
        Vector3D {
            points: (0.0, 0.0, 0.0),
        }
    }

    /// Creates a new point given all coordinates in space.
    pub fn new(x: f64, y: f64, z: f64) -> Vector3D {
        Vector3D { points: (x, y, z) }
    }

    /// Get the x-axis coordinate
    pub fn x(&self) -> f64 {
        self.points.0
    }

    /// Get the y-axis coordinate
    pub fn y(&self) -> f64 {
        self.points.1
    }

    /// Get the z-axis coordinate
    pub fn z(&self) -> f64 {
        self.points.2
    }

    /// Get the squared distance from the centre of coordinate space
    pub fn length_squared(&self) -> f64 {
        self.points.0.powi(2) + self.points.1.powi(2) + self.points.2.powi(2)
    }

    /// Gets the distance between the center of the coordianate space and the vector
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    /// Calculate the dot product of the vector
    pub fn dot(&self, second_vector: &Vector3D) -> f64 {
        self[0] * second_vector[0] + self[1] * second_vector[1] + self[2] * second_vector[2]
    }

    /// Calcualte the cross product of two vectors
    pub fn cross(&self, second_vector: &Vector3D) -> Vector3D {
        Vector3D {
            points: (
                self[1] * second_vector[2] - self[2] * second_vector[1],
                self[2] * second_vector[0] - self[0] * second_vector[2],
                self[0] * second_vector[1] - self[1] * second_vector[0],
            ),
        }
    }

    /// Calcualte the unit vector
    pub fn unit_vector(&self) -> Vector3D {
        self / self.length()
    }
}

impl fmt::Display for Vector3D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self[0], self[1], self[2])
    }
}

// Mathematical operations

impl ops::Add<&Vector3D> for &Vector3D {
    type Output = Vector3D;

    fn add(self, rhs: &Vector3D) -> Self::Output {
        Vector3D {
            points: (self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2]),
        }
    }
}

impl ops::Neg for &Vector3D {
    type Output = Vector3D;

    fn neg(self) -> Self::Output {
        Vector3D {
            points: (-self[0], -self[1], -self[2]),
        }
    }
}

impl ops::Sub<&Vector3D> for &Vector3D {
    type Output = Vector3D;

    fn sub(self, rhs: &Vector3D) -> Self::Output {
        Vector3D {
            points: (
                self[0] - rhs[0],
                self[1] - rhs[1],
                self[2] - rhs[2],
            ),
        }
    }
}

impl ops::Mul<&Vector3D> for &Vector3D {
    type Output = Vector3D;

    fn mul(self, rhs: &Vector3D) -> Self::Output {
        Vector3D {
            points: (
                self[0] * rhs[0],
                self[1] * rhs[1],
                self[2] * rhs[2],
            ),
        }
    }
}

impl ops::Mul<f64> for &Vector3D {
    type Output = Vector3D;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector3D {
            points: (
                self[0] * rhs,
                self[1] * rhs,
                self[2] * rhs,
            ),
        }
    }
}

impl ops::Mul<&Vector3D> for f64 {
    type Output = Vector3D;

    fn mul(self, rhs: &Vector3D) -> Self::Output {
        rhs * self
    }
}

impl ops::Div<f64> for &Vector3D {
    type Output = Vector3D;

    fn div(self, rhs: f64) -> Self::Output {
        (1 as f64 / rhs) * self
    }
}

// Assignment operations

impl ops::AddAssign<&Vector3D> for &Vector3D {
    fn add_assign(&mut self, rhs: &Vector3D) {
        self[0] += rhs[0];
        self[1] += rhs[1];
        self[2] += rhs[2];
    }
}

impl ops::MulAssign<f64> for &Vector3D {
    fn mul_assign(&mut self, rhs: f64) {
        self[0] *= rhs;
        self[1] *= rhs;
        self[2] *= rhs;
    }
}

impl ops::DivAssign<f64> for &Vector3D {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs
    }
}

// Array access

impl ops::Index<usize> for &Vector3D {
    type Output = f64;

    /// Get a part of the point by index
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.points.0,
            1 => &self.points.1,
            2 => &self.points.2,
            _ => panic!("Index in to 3D vector out of bounds!"),
        }
    }
}

impl ops::IndexMut<usize> for &Vector3D {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self[0],
            1 => &mut self[1],
            2 => &mut self[2],
            _ => panic!("Index in to 3D vector out of bounds!"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_has_properties() {
        let vec = Vector3D::new(0.2, 0.1, 5.1);
        assert_eq!()
    }

    #[test]
    fn new_empty_properties_are_zero() {

    }

    #[test]
    fn can_index_in_to_vector() {

    }

    #[test]
    fn can_index_and_change_vector() {

    }

    #[test]
    fn length_squared_of_vector() {

    }

    #[test]
    fn length_of_vector() {

    }

    #[test]
    fn add_two_vectors() {

    }

    #[test]
    fn subtract_two_vectors() {

    }

    #[test]
    fn negate_a_vector() {

    }

    #[test]
    fn multiply_vector_by_vector() {

    }

    #[test]
    fn multiply_vector_by_scalar() {

    }

    #[test]
    fn multiply_scalar_by_vector() {

    }

    #[test]
    fn divide_vector_by_scalar() {

    }

    #[test]
    fn add_and_assign_vector_by_vector() {

    }

    #[test]
    fn multiply_and_assign_vector_by_scalar() {

    }

    #[test]
    fn divide_and_assign_vector_by_scalar() {

    }

    #[test]
    fn dot_product_with_another_vector() {

    }

    #[test]
    fn cross_product_with_another_vector() {

    }

    #[test]
    fn unit_vector_from_vector() {

    }
}
