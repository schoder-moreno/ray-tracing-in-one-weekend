use image::{RgbImage, ImageBuffer, Rgb};
use color::{Color, Scale};
use nalgebra::Vector3;
use ray::Ray;
use camera::{Camera, Point3};

mod color;
mod ray;
mod camera;


// Image
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

fn main()
{
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
        let pixel_color = ray_color(&ray); 
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

fn hit_sphere(center: Point3, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin - center;
    let a = ray.direction.dot(&ray.direction);
    let b = 2.0 * oc.dot(&ray.direction);
    let c = oc.dot(&oc) - radius*radius;
    let discriminant = b*b - 4.0*a*c;

    if discriminant < 0. {
        return -1.;
    } else {
        return (-b - discriminant.sqrt()) / (2.*a);
    }
}

fn ray_color(ray: &Ray) -> Color {
    let t = hit_sphere(Point3::new(0.,0.,-1.), 0.5, ray);

    if t > 0. {
        let normal = (ray.at(t) - Vector3::new(0., 0., -1.)).normalize();
        return 0.5*Color::new(normal.x+1., normal.y+1., normal.z+1.);
    }

    let unit_direction = ray.direction.normalize();
    let t = 0.5*(unit_direction.y + 1.0);
    return (1.0-t)*Color::new(1.0, 1.0, 1.0) + t*Color::new(0.5, 0.7, 1.0);
}