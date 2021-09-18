extern crate num;

use crate::geometry::vector_3d::Vector3D;

/// Color type, to distinguish colors from vectors
pub type Color = Vector3D;

#[derive(Debug, PartialEq)]
struct Pixel {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    fn color_codes(&self, samples_per_pixel: i32) -> Pixel {
        let mut r = self.x();
        let mut g = self.y();
        let mut b = self.z();

        let scale = 1.0 / samples_per_pixel as f64;
        r *= scale;
        g *= scale;
        b *= scale;

        Pixel {
            r: 256.0 * num::clamp(r, 0.0, 0.999),
            g: 256.0 * num::clamp(g, 0.0, 0.999),
            b: 256.0 * num::clamp(b, 0.0, 0.999),
        }
    }

    pub fn write_color(&self, samples_per_pixel: i32) {
        let pixel = self.color_codes(samples_per_pixel);

        println!("{} {} {}", pixel.r, pixel.g, pixel.b)
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn it_gets_the_correct_color_codes_zeroes() {
        let color = Color::empty();
        let expected = Pixel {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        };
        let actual = color.color_codes(1);
        assert_relative_eq!(expected.r, actual.r, epsilon = 0.0001);
        assert_relative_eq!(expected.g, actual.g, epsilon = 0.0001);
        assert_relative_eq!(expected.b, actual.b, epsilon = 0.0001);
    }

    #[test]
    fn it_gets_the_correct_color_codes_random() {
        let color = Color::new(0.01, 0.5, 0.99);
        let expected = Pixel { r: 2.5599, g: 127.995, b: 253.4301 };
        let actual = color.color_codes(1);
        assert_relative_eq!(expected.r, actual.r, epsilon = 0.0001);
        assert_relative_eq!(expected.g, actual.g, epsilon = 0.0001);
        assert_relative_eq!(expected.b, actual.b, epsilon = 0.0001);
    }

    #[test]
    fn it_correctly_clamps_values() {
        let color = Color::new(-0.2, 0.5, 2.0);
        let expected = Pixel { r: 0.0, g: 128.0, b: 255.99 };
        let actual = color.color_codes(1);
        assert_relative_eq!(expected.r, actual.r, epsilon = 0.1);
        assert_relative_eq!(expected.g, actual.g, epsilon = 0.1);
        assert_relative_eq!(expected.b, actual.b, epsilon = 0.1);
    }

    #[test]
    fn it_correctly_scales_values() {
        let color = Color::new(0.0, 1.0, 2.0);
        let expected = Pixel { r: 0.0, g: 128.0, b: 255.99 };
        let actual = color.color_codes(2);
        assert_relative_eq!(expected.r, actual.r, epsilon = 0.1);
        assert_relative_eq!(expected.g, actual.g, epsilon = 0.1);
        assert_relative_eq!(expected.b, actual.b, epsilon = 0.1);
    }
}
