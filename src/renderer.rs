use nalgebra::{Vector3,clamp};

use crate::{utils::{Color, Point3}, world::{World, HitRecord}, camera::Camera, ray::Ray, material::Material};

pub struct Renderer {
    pub world: World,
    pub camera: Camera,
    pub image_width: u32,
    pub image_height: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32
}

impl Renderer {

    pub fn render_pixel(&self, x: u32, y: u32) -> Color {
        let mut pixel_color = Color::new(0., 0., 0.);
        for _ in 0..self.samples_per_pixel {
            let ray = self.create_ray(x, y);
            pixel_color += self.ray_color(&ray, self.max_depth); 
        }

        return self.to_rgb_scale(&pixel_color);
    }

    fn create_ray(&self, x: u32, y: u32) -> Ray {
        let u = (x as f64 + rand::random::<f64>()) / (self.image_width - 1) as f64;
        let v = ((self.image_height - y)as f64 + rand::random::<f64>()) / (self.image_height - 1) as f64;
        return self.camera.get_ray(u, v);
    }

    fn ray_color(&self, ray: &Ray, depth: u32) -> Color {
        if depth <= 0 {
            return Color::new(0., 0., 0.);
        }

        let mut record = HitRecord{point: Point3::new(0., 0., 0.), normal: Vector3::new(0.,0.,0.), t:0. , front_face:false, material: Material::Lambertian { albedo: Color::new(0.,0.,0.) }};

        if self.world.hit(ray, 0.001, f64::MAX, &mut record){
            let (scattered, attenuation, scattered_ray) = record.material.scatter(ray, &record);
            if scattered {
                return attenuation.component_mul(&self.ray_color(&scattered_ray, depth-1));
            }
            return Color::new(0.,0.,0.); 
        }

        let unit_direction = ray.direction.normalize();
        let t = 0.5*(unit_direction.y + 1.0);
        return (1.0-t)*Color::new(1.0, 1.0, 1.0) + t*Color::new(0.5, 0.7, 1.0);
    }

    fn to_rgb_scale(&self, color: &Color) -> Color {
        let mut r = color.x;
        let mut g = color.y;
        let mut b = color.z;

        let scale = 1. / self.samples_per_pixel as f64;
        r = (scale * r).sqrt();
        g = (scale * g).sqrt();
        b = (scale * b).sqrt();

        return Color::new(
            255.999 * clamp(r, 0., 0.999),
            255.999 * clamp(g, 0., 0.999),
            255.999 * clamp(b, 0., 0.999)
        );
    }
}