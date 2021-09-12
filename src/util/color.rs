use crate::geometry::vector_3d::Vector3D;

/// Color type, to distinguish colors from vectors
pub type Color = Vector3D;

#[derive(Debug, PartialEq)]
struct Pixel(f64, f64, f64);

impl Color {
    fn color_codes(&self) -> Pixel {
        Pixel(255.99 * self.x(), 255.99 * self.y(), 255.99 * self.z())
    }

    pub fn write_color(&self) {
        let pixel = self.color_codes();
        println!("{} {} {}", pixel.0, pixel.1, pixel.2)
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn it_writes_the_correct_color_codes_zeroes() {
        let color = Color::empty();
        let expected = Pixel(0.0, 0.0, 0.0);
        let actual = color.color_codes();
        assert_relative_eq!(expected.0, actual.0, epsilon = 0.0001);
        assert_relative_eq!(expected.1, actual.1, epsilon = 0.0001);
        assert_relative_eq!(expected.2, actual.2, epsilon = 0.0001);
    }

    #[test]
    fn it_writes_the_correct_color_codes() {
        let color = Color::new(0.01, 0.5, 0.99);
        let expected = Pixel(2.5599, 127.995, 253.4301);
        let actual = color.color_codes();
        assert_relative_eq!(expected.0, actual.0, epsilon = 0.0001);
        assert_relative_eq!(expected.1, actual.1, epsilon = 0.0001);
        assert_relative_eq!(expected.2, actual.2, epsilon = 0.0001);
    }
}
