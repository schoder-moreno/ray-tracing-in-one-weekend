use nalgebra::Vector3;

pub type Color = Vector3<f64>;
pub trait Scale {
    fn to_rgb_scale(&self) -> [u8; 3];
}

impl Scale for Color {
    fn to_rgb_scale(&self) -> [u8; 3] {
        return [(255.999 * self.x) as u8, (255.999 * self.y) as u8, (255.999 * self.z) as u8];
    }
}