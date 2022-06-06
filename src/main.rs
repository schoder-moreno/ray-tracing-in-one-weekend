use image::{RgbImage, ImageBuffer, Rgb};
use nalgebra::{Vector3};
use color::Color;

mod color;

fn main()
{
    // Image

    let image_width: u32 = 256;
    let image_height: u32 = 256;

    // Render

    let mut buffer: RgbImage = ImageBuffer::new(image_width, image_height);
    for (x, y, pixel) in buffer.enumerate_pixels_mut(){
        if x == 0
        {
            print!("\rScanlines remaining: {} ", image_height-y-1);
        }
        
        let pixel_color = Vector3::new(x as f64 / (image_width - 1) as f64, (image_height - y) as f64 / (image_height - 1) as f64, 0.25);
        *pixel = Rgb(pixel_color.write_color());
    }

    match buffer.save("image.png")
    {
        Err(e) => eprintln!("Error when writing to file: {}", e),
        Ok(()) => println!("Done")
    }
}