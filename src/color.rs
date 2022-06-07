use nalgebra::clamp;
use crate::core::Color;

pub trait Scale {
    fn to_rgb_scale(&self, samples_per_pixel: u32) -> Color;
}

impl Scale for Color {
    fn to_rgb_scale(&self, samples_per_pixel: u32) -> Color {
        let mut r = self.x;
        let mut g = self.y;
        let mut b = self.z;

        let scale = 1. / samples_per_pixel as f64;
        r = (scale * r).sqrt();
        g = (scale * g).sqrt();
        b = (scale * b).sqrt();

        return Color::new(
            (255.999 * clamp(r, 0., 0.999)),
            (255.999 * clamp(g, 0., 0.999)),
            (255.999 * clamp(b, 0., 0.999))
        );
    }
}