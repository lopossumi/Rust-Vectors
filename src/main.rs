use image::{RgbImage, ImageBuffer, Rgb};

mod vector;
use vector::Vec3;

fn main() {

    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;

    let mut buffer: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    
    for (x, y, pixel) in buffer.enumerate_pixels_mut(){
        let vector = Vec3::new(
            x as f64 / (IMAGE_WIDTH-1) as f64,
            y as f64 / (IMAGE_HEIGHT-1) as f64,
            0.25);
        let color = vector.to_rgb();
        *pixel = Rgb(color);
    }

    match buffer.save("image.png") {
        Err(e) => eprintln!("Error writing file: {}", e),
        Ok(()) => println!("Done."),
    };
}