use image::{DynamicImage, ImageReader};

pub fn prepend<T>(v: Vec<T>, s: &[T]) -> Vec<T>
where
    T: Clone,
{
    let mut tmp: Vec<_> = s.to_owned();
    tmp.extend(v);
    tmp
}

pub fn load_image(path: &str) -> Result<DynamicImage, Box<dyn std::error::Error>> {
    let img = ImageReader::open(path)?.decode()?;
    Ok(img)
}
