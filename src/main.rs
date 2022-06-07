use crate::core::{Color, Point3, random_in_unit_sphere};
use image::{RgbImage, ImageBuffer, Rgb};
use color::Scale;
use nalgebra::Vector3;
use ray::Ray;
use camera::Camera;
use hittable::{HittableList, HitRecord};
use renderer::Renderer;
use sphere::Sphere;

mod core;
mod color;
mod ray;
mod camera;
mod hittable;
mod sphere;
mod renderer;

// Image
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: u32 = 100;
const MAX_DEPTH: u32 = 50;

fn main()
{
    // World

    let mut world = HittableList::new();
    world.push(Sphere {center: Point3::new(0., 0., -1.), radius: 0.5});
    world.push(Sphere {center: Point3::new(0., -100.5, -1.), radius: 100.});

    // Camera

    let camera = Camera::new(ASPECT_RATIO);

    // Render

    let renderer = Renderer {
        world: world,
        camera: camera, 
        image_width: IMAGE_WIDTH,
        image_height: IMAGE_HEIGHT,
        samples_per_pixel: SAMPLES_PER_PIXEL,
        max_depth: MAX_DEPTH
    };

    let mut buffer: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    for (x, y, pixel) in buffer.enumerate_pixels_mut(){
        if x == 0
        {
            print!("\rScanlines remaining: {} ", IMAGE_HEIGHT-y-1);
        }

        let pixel_color = renderer.render_pixel(x, y); 
        *pixel = Rgb([pixel_color.x as u8, pixel_color.y as u8, pixel_color.z as u8]);
    }

    match buffer.save("image.png")
    {
        Err(e) => eprintln!("Error when writing to file: {}", e),
        Ok(()) => println!("Done")
    }
}



