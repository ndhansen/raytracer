use crate::util::{color::Color, point::Point3D};

use super::Texture;

#[derive(Debug, Clone)]
pub struct SolidColorTexture {
    color: Color,
}

impl SolidColorTexture {
    pub fn new(color: Color) -> SolidColorTexture {
        SolidColorTexture { color }
    }

    pub fn from_rgb(r: f64, g: f64, b: f64) -> SolidColorTexture {
        SolidColorTexture {
            color: Color::new(r, g, b),
        }
    }
}

impl Texture for SolidColorTexture {
    fn color(&self, _u: f64, _v: f64, _point: &Point3D) -> Color {
        self.color
    }
}
