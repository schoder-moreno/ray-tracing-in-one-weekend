use nalgebra::Vector3;

use crate::{ray::Ray, hittable::HitRecord, core::{Color, random_in_unit_sphere, random_unit_vector}};

#[derive(Clone)]
pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: f64 },
}

impl Material {
    pub fn scatter(&self, incoming_ray: &Ray, record: &mut HitRecord, attenuation: &mut Color, scattered_ray: &mut Ray) -> bool {
        match self {
            Material::Lambertian { albedo } => {
                let mut scatter_direction = record.normal + random_unit_vector();
                
                if scatter_direction.argmax().1 < 1.0e-8_f64 {
                    scatter_direction = record.normal;
                }

                *scattered_ray = Ray::new(record.point, scatter_direction);
                *attenuation = Color::new(albedo.x, albedo.y, albedo.z);
                return true;
            },
            Material::Metal { albedo, fuzz } => {
                let reflected = reflect(incoming_ray.direction.normalize(), record.normal);
                *scattered_ray = Ray::new(record.point, reflected + (if *fuzz < 1. {*fuzz} else {1.})*random_in_unit_sphere());
                *attenuation = Color::new(albedo.x, albedo.y, albedo.z);
                return scattered_ray.direction.dot(&record.normal) > 0.;
            },
        }   
    }
}

fn reflect(v: Vector3<f64>, n: Vector3<f64>) -> Vector3<f64> {
    return v - 2.*v.dot(&n)*n;
}
