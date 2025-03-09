use crate::color_channel::{Channel, ChannelIter};
use crate::offset_iter::OffsetIter;
use image::{DynamicImage, GenericImageView};
use std::mem::transmute;
use std::str;

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

pub fn decode(
    from: DynamicImage,
    channels: Vec<Channel>,
    offsets: &[u32],
    skip: u32,
) -> Option<String> {
    let mut message = Vec::new();
    let mut channel_iter = ChannelIter::new(channels);
    let mut offset_iter = OffsetIter::new(offsets.to_vec(), skip);
    let mut next_data_index = offset_iter.next().unwrap();

    from.pixels().for_each(|pixel| {
        let source_pixel = pixel.2;
        let index = (pixel.0 + pixel.1 * from.dimensions().0) as usize;

        if index == next_data_index {
            let channel = channel_iter.next().unwrap();

            if channel.red {
                message.push((source_pixel[0] & 1) == 1);
            }

            if channel.green {
                message.push((source_pixel[1] & 1) == 1);
            }

            if channel.blue {
                message.push((source_pixel[2] & 1) == 1);
            }

            if channel.alpha {
                message.push((source_pixel[3] & 1) == 1);
            }

            next_data_index = offset_iter.next().unwrap();
        }
    });

    let bytes = to_bytes(message);
    let length = unsafe {
        u64::from_be(transmute([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]))
    };

    if length as usize > bytes.len() - 8 {
        return None;
    }

    let message_bytes = &bytes[8..(8 + length as usize)];

    str::from_utf8(&message_bytes).ok().map(|s| s.to_owned())
}
