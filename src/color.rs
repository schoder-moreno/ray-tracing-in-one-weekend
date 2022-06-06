use nalgebra::Vector3;

pub trait Color {
    fn write_color(&self) -> [u8; 3];
}

impl Color for Vector3<f64> {
    fn write_color(&self) -> [u8; 3] {
        return [(255.999 * self.x) as u8, (255.999 * self.y) as u8, (255.999 * self.z) as u8];
    }
}