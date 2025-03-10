use clap::Parser;

mod color_channel;
mod decode;
mod encode;
mod message_iter;
mod offset_iter;
mod utils;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(short, long)]
    output: Option<String>,

    #[arg(short, long, default_value_t = 0)]
    skip: u32,

    #[arg(short, long)]
    message: Option<String>,

    #[arg(short, long, num_args = 1.., value_delimiter = ',')]
    channels: Option<Vec<String>>,

    #[arg(short, long, num_args = 1.., value_delimiter = ',')]
    pattern: Option<Vec<u32>>,

    #[arg(short, long, action)]
    decode: bool,
}

fn create_out_name(input: &str) -> String {
    let mut parts: Vec<&str> = input.split('.').collect();
    let ext = parts.pop().unwrap();
    let name = parts.join(".");
    format!("{}-out.{}", name, ext)
}

fn build_channels(channels: Vec<String>) -> Vec<color_channel::Channel> {
    channels
        .iter()
        .map(|s| {
            let lowercase = s.to_lowercase();
            color_channel::Channel {
                red: lowercase.contains('r'),
                green: lowercase.contains('g'),
                blue: lowercase.contains('b'),
                alpha: lowercase.contains('a'),
            }
        })
        .collect()
}

fn main() {
    let args = Args::parse();
    let output = args.output.unwrap_or_else(|| create_out_name(&args.input));
    let channels = args.channels.map(build_channels).unwrap_or_else(|| {
        vec![color_channel::Channel {
            red: true,
            green: true,
            blue: true,
            alpha: false,
        }]
    });
    let offsets = args.pattern.unwrap_or_else(|| vec![0]);
    let maybe_img = utils::load_image(&args.input);

    if maybe_img.is_err() {
        println!("Error loading image: {:?}", maybe_img.err());
        return;
    }

    let input = maybe_img.unwrap();

    if args.decode {
        if let Some(result) = decode::decode(input, channels, &offsets, args.skip) {
            println!("{}", result);
        } else {
            println!("Error decoding message");
        }
    } else {
        if args.message.is_none() {
            println!("No message provided");
            return;
        }
        let message = args.message.unwrap();

        match encode::encode(input, &message, channels, &offsets, args.skip) {
            Ok(result) => {
                result.save(output).unwrap();
                println!("Message encoded.");
            }
            Err(e) => {
                println!("Error encoding message: {}", e);
            }
        }
    }
}
