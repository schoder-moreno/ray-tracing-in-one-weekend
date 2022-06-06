use nalgebra::Vector3;
use crate::core::Point3;
use crate::ray::Ray;

pub struct Camera{
    pub origin: Point3,
    pub horizontal: Vector3<f64>,
    pub vertical: Vector3<f64>,
    pub lower_left_corner: Point3
}

impl Camera {
    pub fn new(aspect_ratio: f64) -> Camera {
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;
        
        let origin = Point3::new(0., 0., 0.);
        let horizontal = Vector3::new(viewport_width, 0., 0.);
        let vertical = Vector3::new(0., viewport_height, 0.);

        return Camera {
            origin: origin,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: origin - horizontal/2. - vertical/2. - Vector3::new(0., 0., focal_length)
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        return Ray{
            origin: self.origin,
            direction: self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin
        };
    }
}
