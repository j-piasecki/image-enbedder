use image::{DynamicImage, GenericImageView, ImageBuffer, ImageReader, Rgba, RgbaImage};
use std::env;

fn load_image() -> Result<DynamicImage, Box<dyn std::error::Error>> {
    let img = ImageReader::open("in.png")?.decode()?;
    Ok(img)
}

fn encode(onto: DynamicImage, text: &str, offsets: &[u32]) -> RgbaImage {
    let mut result: RgbaImage = ImageBuffer::new(onto.dimensions().0, onto.dimensions().1);

    result.enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        let source_pixel = onto.get_pixel(x, y).0;
        *pixel = Rgba([
            source_pixel[0],
            source_pixel[1],
            source_pixel[2],
            source_pixel[3],
        ]);
    });

    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);

    let maybe_img = load_image();

    if maybe_img.is_err() {
        println!("Error loading image: {:?}", maybe_img.err());
        return;
    }

    let source = maybe_img.unwrap();
    encode(source, "hello world", &[0]).save("out.png").unwrap();
}
