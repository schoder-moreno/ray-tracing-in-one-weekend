use image::{RgbImage, ImageBuffer, Rgb};

fn main()
{
    // Image

    let image_width: u32 = 256;
    let image_height: u32 = 256;

    // Render

    let mut buffer: RgbImage = ImageBuffer::new(image_width, image_height);
    for (x, y, pixel) in buffer.enumerate_pixels_mut(){
        let r = x as f64 / (image_width - 1) as f64;
        let g = (image_height - y) as f64 / (image_height - 1) as f64;
        let b = 0.25;

        let ir = (255.999 * r) as u8;
        let ig = (255.999 * g) as u8;
        let ib = (255.999 * b) as u8;
        
        *pixel = Rgb([ir, ig, ib]);
    }

    match buffer.save("image.png")
    {
        Err(e) => eprintln!("Error when writing to file: {}", e),
        Ok(()) => println!("Done")
    }
}