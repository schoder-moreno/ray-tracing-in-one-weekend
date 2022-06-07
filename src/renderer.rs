use nalgebra::Vector3;

use crate::{core::{Color, Point3, random_in_unit_sphere}, hittable::{HittableList, HitRecord}, camera::Camera, ray::Ray, color::Scale};

pub struct Renderer {
    pub world: HittableList,
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

        return pixel_color.to_rgb_scale(self.samples_per_pixel);
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

        let mut record = HitRecord{point: Point3::new(0., 0., 0.), normal: Vector3::new(0.,0.,0.), t:0. , front_face:false };
        if self.world.hit(ray, 0.001, f64::MAX, &mut record){
            let target = record.point + record.normal + random_in_unit_sphere();
            return 0.5 * self.ray_color(&Ray::new(record.point, target - record.point), depth - 1);
        }

        let unit_direction = ray.direction.normalize();
        let t = 0.5*(unit_direction.y + 1.0);
        return (1.0-t)*Color::new(1.0, 1.0, 1.0) + t*Color::new(0.5, 0.7, 1.0);
    }
}