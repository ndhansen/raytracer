use crate::util::{color::Color, point::Point3D};

use super::{SolidColorTexture, Texture};

pub struct CheckerTexture {
    odd: Box<dyn Texture>,
    even: Box<dyn Texture>,
}

impl CheckerTexture {
    pub fn from_textures(even: Box<dyn Texture>, odd: Box<dyn Texture>) -> CheckerTexture {
        CheckerTexture { odd, even }
    }

    pub fn from_colors(even: Color, odd: Color) -> CheckerTexture {
        let even = Box::new(SolidColorTexture::new(even));
        let odd = Box::new(SolidColorTexture::new(odd));
        CheckerTexture { odd, even }
    }
}

impl Texture for CheckerTexture {
    fn color(&self, u: f64, v: f64, point: &Point3D) -> Color {
        let sines = (10.0 * point.x()).sin() * (10.0 * point.y()).sin() * (10.0 * point.z()).sin();
        if sines < 0.0 {
            return self.odd.color(u, v, point);
        }
        self.even.color(u, v, point)
    }
}
