use nalgebra::Vector3;
use crate::core::{Point3, degrees_to_radians};
use crate::ray::Ray;

pub struct Camera{
    pub origin: Point3,
    pub horizontal: Vector3<f64>,
    pub vertical: Vector3<f64>,
    pub lower_left_corner: Point3
}

impl Camera {
    pub fn new(lookfrom: Point3, lookat: Point3, vup: Vector3<f64>, vertical_fov: f64, aspect_ratio: f64) -> Camera {
        let theta = degrees_to_radians(vertical_fov);
        let h = (theta/2.).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        
        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);

        let origin = lookfrom;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;

        return Camera {
            origin: origin,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: origin - horizontal/2. - vertical/2. - w
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        return Ray{
            origin: self.origin,
            direction: self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin
        };
    }
}
