use crate::color_channel;
use crate::message_iter;
use crate::offset_iter;
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba, RgbaImage};

pub fn encode(
    onto: DynamicImage,
    text: &str,
    channels: Vec<color_channel::Channel>,
    offsets: &[u32],
    skip: u32,
) -> Result<RgbaImage, String> {
    let mut result: RgbaImage = ImageBuffer::new(onto.dimensions().0, onto.dimensions().1);
    let mut message_iter = message_iter::MessageIter::new(text);
    let mut channel_iter = color_channel::ChannelIter::new(channels);
    let mut offset_iter = offset_iter::OffsetIter::new(offsets.to_vec(), skip);
    let mut next_data_index = offset_iter.next().unwrap();

    result.enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        let source_pixel = onto.get_pixel(x, y).0;
        let index = (x + y * onto.dimensions().0) as usize;

        let mut red = source_pixel[0];
        let mut green = source_pixel[1];
        let mut blue = source_pixel[2];
        let mut alpha = source_pixel[3];

        if index == next_data_index {
            let channel = channel_iter.next().unwrap();

            if channel.red {
                if let Some(bit) = message_iter.next() {
                    red = if bit { red | 1 } else { red & !1 };
                }
            }

            if channel.green {
                if let Some(bit) = message_iter.next() {
                    green = if bit { green | 1 } else { green & !1 };
                }
            }

            if channel.blue {
                if let Some(bit) = message_iter.next() {
                    blue = if bit { blue | 1 } else { blue & !1 };
                }
            }

            if channel.alpha {
                if let Some(bit) = message_iter.next() {
                    alpha = if bit { alpha | 1 } else { alpha & !1 };
                }
            }

            next_data_index = offset_iter.next().unwrap();
        }

        *pixel = Rgba([red, green, blue, alpha]);
    });

    if message_iter.next().is_some() {
        return Err("Message too long or offsets too sparse".to_owned());
    } else {
        Ok(result)
    }
}
