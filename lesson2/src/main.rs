use std::u8;

use clap::Parser;
use image::{DynamicImage, GenericImage, ImageBuffer, ImageDecoder, ImageReader};
use rayon::prelude::*;
#[derive(Debug, Parser)]
struct Args {
    #[clap(long)]
    img: String,
}

fn main() {
    let args = Args::parse();

    println!("Input image1: {}", args.img);

    let mut img = ImageReader::open(args.img).unwrap().decode().unwrap();

    let threshold = 64;

    img.as_mut
        .unwrap()
        .enumerate_pixels_mut()
        .for_each(|(x, y, pixel)| {
            pixel[0] = if pixel[0] < threshold { 0 } else { u8::MAX };
            pixel[1] = if pixel[1] < threshold { 0 } else { u8::MAX };
            pixel[2] = if pixel[2] < threshold { 0 } else { u8::MAX };
        });

    img.save("bin.png").unwrap();
}
