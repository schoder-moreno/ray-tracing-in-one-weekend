use nalgebra::Vector3;
use rand::Rng;

pub type Point3 = Vector3<f64>;
pub type Color = Vector3<f64>;

pub const PI: f64 = 3.1415926535897932385;

pub fn random_vector3(min: f64, max: f64) -> Vector3<f64> {
    return Vector3::new(rand::thread_rng().gen_range(min..max), rand::thread_rng().gen_range(min..max), rand::thread_rng().gen_range(min..max));
}

pub fn random_in_unit_sphere() -> Vector3<f64> {
    loop {
        let p = random_vector3(-1., 1.);
        if p.magnitude_squared() >= 1. {
            continue;
        } 
        return p;
    }
}