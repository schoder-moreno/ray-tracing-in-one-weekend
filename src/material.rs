use nalgebra::Vector3;

use crate::{ray::Ray, world::HitRecord, utils::{Color, random_in_unit_sphere, random_unit_vector}};

#[derive(Clone)]
pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: f64 },
    Dielectric {index_of_refraction: f64}
}

impl Material {
    pub fn scatter(&self, incoming_ray: &Ray, record: &HitRecord) -> (bool, Color, Ray) {
        match self {
            Material::Lambertian { albedo } => scatter_lambertian(record, albedo), 
            Material::Metal { albedo, fuzz } => scatter_metal(incoming_ray, record, fuzz, albedo), 
            Material::Dielectric { index_of_refraction } => scatter_dielectric(record, index_of_refraction, incoming_ray) 
        }   
    }
}

fn scatter_lambertian(record: &HitRecord, albedo: &Color) -> (bool, Color, Ray) {
    let mut scatter_direction = record.normal + random_unit_vector();
    if scatter_direction.argmax().1 < 1.0e-8_f64 {
        scatter_direction = record.normal;
    }
    let scattered_ray = Ray::new(record.point, scatter_direction);
    let attenuation = Color::new(albedo.x, albedo.y, albedo.z);

    return (true, attenuation, scattered_ray);
}

fn scatter_metal(incoming_ray: &Ray, record: &HitRecord, fuzz: &f64, albedo: &Color) -> (bool, Color, Ray) {
    let reflected = reflect(incoming_ray.direction.normalize(), record.normal);
    let scattered_ray = Ray::new(record.point, reflected + (if *fuzz < 1. {*fuzz} else {1.})*random_in_unit_sphere());
    let attenuation = Color::new(albedo.x, albedo.y, albedo.z);

    return (scattered_ray.direction.dot(&record.normal) > 0., attenuation, scattered_ray);
}

fn scatter_dielectric(record: &HitRecord, index_of_refraction: &f64, incoming_ray: &Ray) -> (bool, Color, Ray) {
    let refraction_ratio = if record.front_face {1./index_of_refraction} else {*index_of_refraction};
    let unit_direction = incoming_ray.direction.normalize();
    let cos_theta = f64::min(-unit_direction.dot(&record.normal), 1.0);
    let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();
    let cannot_refract = refraction_ratio * sin_theta > 1.0;
    let direction: Vector3<f64>;
    if cannot_refract || reflectance(cos_theta, refraction_ratio) > rand::random::<f64>(){
        direction = reflect(unit_direction, record.normal);
    }
    else {
        direction = refract(unit_direction, record.normal, refraction_ratio);
    }
    let scattered_ray = Ray::new(record.point, direction);
    let attenuation = Color::new(1., 1., 1.);

    return (true, attenuation, scattered_ray);
}

fn reflect(v: Vector3<f64>, n: Vector3<f64>) -> Vector3<f64> {
    return v - 2.*v.dot(&n)*n;
}
fn refract(incoming_unit_vector: Vector3<f64>, normal: Vector3<f64>, etai_over_etat: f64) -> Vector3<f64> {
    let cos_theta = f64::min(-incoming_unit_vector.dot(&normal), 1.0);
    let r_out_perp = etai_over_etat * (incoming_unit_vector + cos_theta*normal);
    let r_out_parallel = -((1.0 - r_out_perp.magnitude_squared()).abs()).sqrt()*normal;
    return r_out_perp + r_out_parallel;
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    // Use Schlick's approximation for reflectance.
    let mut r0 = (1.-ref_idx) / (1.+ref_idx);
    r0 *= r0;

    return r0 + (1.-r0)*f64::powf(1.-cosine, 5.);
} 