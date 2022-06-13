use nalgebra::Vector3;
use crate::core::{Point3, degrees_to_radians, random_in_unit_disk};
use crate::ray::Ray;

pub struct Camera{
    pub origin: Point3,
    pub horizontal: Vector3<f64>,
    pub vertical: Vector3<f64>,
    pub lower_left_corner: Point3,
    pub u: Vector3<f64>, 
    pub v: Vector3<f64>, 
    pub w: Vector3<f64>,
    pub lens_radius: f64
}

impl Camera {
    pub fn new(lookfrom: Point3, lookat: Point3, vup: Vector3<f64>, vertical_fov: f64, aspect_ratio: f64, aperture: f64, focus_dist: f64) -> Camera {
        let theta = degrees_to_radians(vertical_fov);
        let h = (theta/2.).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        
        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);

        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;

        return Camera {
            origin: origin,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: origin - horizontal/2. - vertical/2. - focus_dist*w,
            u: u,
            v: v,
            w: w,
            lens_radius: aperture/2.
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;

        return Ray{
            origin: self.origin + offset,
            direction: self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin - offset
        };
    }
}
