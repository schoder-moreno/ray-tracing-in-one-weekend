use crate::utils::{Point3, Color, random_vector3, random_vector3_range};
use image::{RgbImage, ImageBuffer};
use camera::Camera;
use nalgebra::Vector3;
use rand::Rng;
use utils::to_rgb;
use world::World;
use material::Material;
use renderer::Renderer;
use sphere::Sphere;
use rayon::prelude::*;

mod utils;
mod ray;
mod camera;
mod world;
mod sphere;
mod renderer;
mod material;

// Image
const ASPECT_RATIO: f64 = 3.0 / 2.0;
const IMAGE_WIDTH: u32 = 1200;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: u32 = 5;
const MAX_DEPTH: u32 = 50;

fn main()
{
    // World

    let world = random_scene();

    // Camera

    let lookfrom = Point3::new(13., 2., 3.);
    let lookat = Point3::new(0.,0.,0.);
    let vup = Vector3::new(0.,1.,0.);
    let vertical_fov = 20.0;
    let aperture = 0.1;
    let focus_dist = 10.0;

    let camera = Camera::new(lookfrom, lookat, vup, vertical_fov, ASPECT_RATIO, aperture, focus_dist);

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
    buffer.enumerate_pixels_mut()
        .par_bridge()
        .for_each(|(x, y, pixel)| {
            if x == 0 {
                print!("\rScanlines remaining: {} ", IMAGE_HEIGHT-y-1);
            }

            *pixel = to_rgb(renderer.render_pixel(x, y));
        }
    );

    match buffer.save("image.png")
    {
        Err(e) => eprintln!("Error when writing to file: {}", e),
        Ok(()) => println!("Done")
    }
}

fn random_scene() -> World {
    let mut world = World::new();

    let ground_material = Material::Lambertian{albedo: Color::new(0.5, 0.5, 0.5)};
    world.push(Sphere {center: Point3::new(0., -1000., 0.), radius: 1000., material: ground_material});

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand::random::<f64>();
            let center = Point3::new(a as f64 + rand::random::<f64>(), 0.2, b as f64 + 0.9*rand::random::<f64>());

            if (center - Point3::new(4., 0.2, 0.)).magnitude() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = random_vector3().component_mul(&random_vector3());
                    let material = Material::Lambertian { albedo: albedo };
                    world.push(Sphere {center: center, radius: 0.2, material: material});
                } else if choose_mat < 0.95 {
                    // meta
                    let albedo = random_vector3_range(0.5, 1.);
                    let fuzz = rand::thread_rng().gen_range(0.0..0.5);
                    let material = Material::Metal { albedo: albedo, fuzz: fuzz };
                    world.push(Sphere {center: center, radius: 0.2, material: material});
                } else {
                    // glass
                    let material = Material::Dielectric { index_of_refraction: 1.5 };
                    world.push(Sphere {center: center, radius: 0.2, material: material});
                }
            }
        }
    }

    let material1 = Material::Dielectric { index_of_refraction: 1.5 };
    world.push(Sphere {center: Point3::new(0., 1., 0.), radius: 1., material: material1});

    let material2 = Material::Lambertian {albedo: Color::new(0.4, 0.2, 0.1)};
    world.push(Sphere {center: Point3::new(-4., 1., 0.), radius: 1., material: material2});

    let material3 = Material::Metal {albedo: Color::new(0.7, 0.6, 0.5), fuzz: 0.};
    world.push(Sphere {center: Point3::new(4., 1., 0.), radius: 1., material: material3});

    return world;
}



