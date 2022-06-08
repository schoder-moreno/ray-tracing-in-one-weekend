use nalgebra::Vector3;
use crate::core::{Point3, Color};
use crate::material::Material;
use crate::ray::Ray;

#[derive(Clone)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vector3<f64>,
    pub t: f64,
    pub front_face: bool,
    pub material: Material
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vector3<f64>){
        self.front_face = ray.direction.dot(&outward_normal) < 0.;
        self.normal = if self.front_face {outward_normal} else {-outward_normal};
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool;
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new () -> Self {
        Self { objects: Vec::new() }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn push<S: Hittable + 'static>(&mut self, object: S) {
        self.objects.push(Box::new(object));
    }
}

impl HittableList {
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        let mut temp_record = HitRecord{point: Point3::new(0.,0.,0.), normal: Vector3::new(0.,0.,0.), t: 0., front_face: false, material: Material::Lambertian{albedo: Color::new(0.,0.,0.)}};
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if object.hit(ray, t_min, closest_so_far, &mut temp_record){
                hit_anything = true;
                closest_so_far = temp_record.t;
                record.clone_from(&temp_record);
            }
        }

        return hit_anything;
    }
}