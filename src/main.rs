use crate::core::{Color, Point3};
use image::{RgbImage, ImageBuffer, Rgb};
use color::Scale;
use nalgebra::Vector3;
use ray::Ray;
use camera::Camera;
use hittable::{HittableList, HitRecord};
use sphere::Sphere;

mod core;
mod color;
mod ray;
mod camera;
mod hittable;
mod sphere;

// Image
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

fn main()
{
    // World

    let mut world = HittableList::new();
    world.push(Sphere {center: Point3::new(0., 0., -1.), radius: 0.5});
    world.push(Sphere {center: Point3::new(0., -100.5, -1.), radius: 100.});

    // Camera

    let camera = Camera::new(ASPECT_RATIO);

    // Render

    let mut buffer: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    for (x, y, pixel) in buffer.enumerate_pixels_mut(){
        if x == 0
        {
            print!("\rScanlines remaining: {} ", IMAGE_HEIGHT-y-1);
        }

        let ray = create_ray(&camera, x, y);
        let pixel_color = ray_color(&ray, &world); 
        *pixel = Rgb(pixel_color.to_rgb_scale());
    }

    match buffer.save("image.png")
    {
        Err(e) => eprintln!("Error when writing to file: {}", e),
        Ok(()) => println!("Done")
    }
}

fn create_ray(camera: &Camera, x: u32, y: u32) -> Ray {
    let u = x as f64 / (IMAGE_WIDTH - 1) as f64;
    let v = (IMAGE_HEIGHT - y) as f64 / (IMAGE_HEIGHT - 1) as f64;
    return Ray::new(&camera, u, v);
}

fn ray_color(ray: &Ray, world: &HittableList) -> Color {
    let mut record = HitRecord{point: Point3::new(0., 0., 0.), normal: Vector3::new(0.,0.,0.), t:0. , front_face:false };
    if world.hit(ray, 0., f64::MAX, &mut record){
        return 0.5 * (record.normal + Color::new(1., 1., 1.));
    }

    let unit_direction = ray.direction.normalize();
    let t = 0.5*(unit_direction.y + 1.0);
    return (1.0-t)*Color::new(1.0, 1.0, 1.0) + t*Color::new(0.5, 0.7, 1.0);
}