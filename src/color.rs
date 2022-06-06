use nalgebra::clamp;

use crate::core::Color;

pub trait Scale {
    fn to_rgb_scale(&self, samples_per_pixel: u32) -> [u8; 3];
}

impl Scale for Color {
    fn to_rgb_scale(&self, samples_per_pixel: u32) -> [u8; 3] {
        let mut r = self.x;
        let mut g = self.y;
        let mut b = self.z;

        let scale = 1. / samples_per_pixel as f64;
        r *= scale;
        g *= scale;
        b *= scale;

        return [
            (255.999 * clamp(r, 0., 0.999)) as u8,
            (255.999 * clamp(g, 0., 0.999)) as u8,
            (255.999 * clamp(b, 0., 0.999)) as u8
        ];
    }
}