use crate::util::vector_3d::Vector3D;

/// Color type, to distinguish colors from vectors
pub type Color = Vector3D;

impl Color {
    pub fn write_color(&self) {
        println!(
            "{} {} {}",
            255.99 * self.x(),
            255.99 * self.y(),
            255.99 * self.z()
        )
    }
}
