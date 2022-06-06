use crate::camera::Camera;
use nalgebra::Vector3;

pub struct Ray {
    pub origin: Vector3<f64>,
    pub direction: Vector3<f64>
}

impl Ray {
    pub fn new(camera: &Camera, u: f64, v: f64) -> Ray {
        Ray {
            origin: camera.origin,
            direction: camera.lower_left_corner + u*camera.horizontal + v*camera.vertical - camera.origin
        }
    }

    pub fn at(&self, t: f64) -> Vector3<f64> {
        return self.origin + t*self.direction;
    }
}