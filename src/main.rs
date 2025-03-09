use image::{DynamicImage, GenericImageView, ImageBuffer, ImageReader, Rgba, RgbaImage};
use std::env;
use std::mem::transmute;
use std::str;

fn prepend<T>(v: Vec<T>, s: &[T]) -> Vec<T>
where
    T: Clone,
{
    let mut tmp: Vec<_> = s.to_owned();
    tmp.extend(v);
    tmp
}

struct OffsetIter {
    current: usize,
    index: usize,
    offsets: Vec<u32>,
}

impl OffsetIter {
    fn new(offsets: Vec<u32>) -> Self {
        Self {
            current: 0,
            index: 0,
            offsets,
        }
    }
}

impl Iterator for OffsetIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.current + self.offsets[self.index] as usize;
        self.current = result + 1;
        self.index = (self.index + 1) % self.offsets.len();
        Some(result)
    }
}

struct MessageIter {
    message: Vec<u8>,
    bit_index: usize,
    index: usize,
}

impl MessageIter {
    fn new(message: &str) -> Self {
        let mut bytes = message.as_bytes().to_vec();
        let preamble: [u8; 8] = unsafe { transmute(bytes.len().to_be()) };
        bytes = prepend(bytes, &preamble);

        Self {
            message: bytes,
            bit_index: 0,
            index: 0,
        }
    }
}

impl Iterator for MessageIter {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.message.len() {
            return None;
        }

        let byte = self.message[self.index];
        let result = (byte >> (7 - self.bit_index)) & 1 == 1;
        self.bit_index = (self.bit_index + 1) % 8;

        if self.bit_index == 0 {
            self.index += 1;
        }

        Some(result)
    }
}

fn load_image(path: &str) -> Result<DynamicImage, Box<dyn std::error::Error>> {
    let img = ImageReader::open(path)?.decode()?;
    Ok(img)
}

fn encode(onto: DynamicImage, text: &str, offsets: &[u32]) -> RgbaImage {
    let mut result: RgbaImage = ImageBuffer::new(onto.dimensions().0, onto.dimensions().1);
    let mut message_iter = MessageIter::new(text);
    let mut offset_iter = OffsetIter::new(offsets.to_vec());
    let mut next_data_index = offset_iter.next().unwrap();

    result.enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        let source_pixel = onto.get_pixel(x, y).0;
        let index = (x + y * onto.dimensions().0) as usize;

        let mut red = source_pixel[0];
        let mut green = source_pixel[1];
        let mut blue = source_pixel[2];
        let mut alpha = source_pixel[3];

        if index == next_data_index {
            if let Some(bit) = message_iter.next() {
                red = if bit { red | 1 } else { red & !1 };
            }

            next_data_index = offset_iter.next().unwrap();
        }

        *pixel = Rgba([red, green, blue, alpha]);
    });

    result
}

fn to_bytes(message: Vec<bool>) -> Vec<u8> {
    let mut result = Vec::new();
    let mut byte = 0;

    for (i, bit) in message.iter().enumerate() {
        byte |= (*bit as u8) << (7 - i % 8);

        if i % 8 == 7 {
            result.push(byte);
            byte = 0;
        }
    }

    result
}

fn decode(from: DynamicImage, offsets: &[u32]) -> Option<String> {
    let mut message = Vec::new();
    let mut offset_iter = OffsetIter::new(offsets.to_vec());
    let mut next_data_index = offset_iter.next().unwrap();

    from.pixels().for_each(|pixel| {
        let source_pixel = pixel.2;
        let index = (pixel.0 + pixel.1 * from.dimensions().0) as usize;

        if index == next_data_index {
            message.push((source_pixel[0] & 1) == 1);
            next_data_index = offset_iter.next().unwrap();
        }
    });

    let bytes = to_bytes(message);
    let length = unsafe {
        u64::from_be(transmute([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]))
    };
    let message_bytes = &bytes[8..(8 + length as usize)];
    let message = str::from_utf8(&message_bytes).unwrap();
    Some(message.to_owned())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);

    // let maybe_img = load_image("in.png");

    // if maybe_img.is_err() {
    //     println!("Error loading image: {:?}", maybe_img.err());
    //     return;
    // }

    // let source = maybe_img.unwrap();
    // encode(source, "hello world", &[0]).save("out.png").unwrap();

    let maybe_img = load_image("out.png");

    if maybe_img.is_err() {
        println!("Error loading image: {:?}", maybe_img.err());
        return;
    }

    let source = maybe_img.unwrap();
    let result = decode(source, &[0]).unwrap();
    println!("Decoded message: {}", result);
}
