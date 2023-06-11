use crate::util::{color::Color, point::Point3D};

pub trait Texture: Send + Sync {
    fn color(&self, u: f64, v: f64, point: &Point3D) -> Color;
}
