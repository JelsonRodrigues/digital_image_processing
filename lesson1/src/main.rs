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

    img.as_mut_rgba8()
        .unwrap()
        .pixels_mut()
        .into_iter()
        .for_each(|pixel| {
            pixel[0] = u8::MAX - pixel[0];
            pixel[1] = u8::MAX - pixel[1];
            pixel[2] = u8::MAX - pixel[2];
        });

    // save inverted image
    img.save("inverted.png").unwrap();
}
