use nalgebra::Vector3;

use crate::{utils::Point3};

pub struct Ray {
    pub origin: Vector3<f64>,
    pub direction: Vector3<f64>
}

impl Ray {
    pub fn new(origin: Point3, direction: Vector3<f64>) -> Ray {
        return Ray { origin: origin, direction: direction }
    }

    pub fn at(&self, t: f64) -> Vector3<f64> {
        return self.origin + t*self.direction;
    }
}