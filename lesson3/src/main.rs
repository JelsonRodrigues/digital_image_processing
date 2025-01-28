use clap::Parser;
use convolve2d::Matrix;
use image::{
    buffer::ConvertBuffer, DynamicImage, GenericImage, ImageBuffer, ImageDecoder, ImageReader, Luma,
};

#[derive(Debug, Parser)]
struct Args {
    #[clap(long)]
    img: String,
}

fn main() {
    let args = Args::parse();

    println!("Input image1: {}", args.img);

    let img = ImageReader::open(args.img).unwrap().decode().unwrap();

    let kernel = Some(convolve2d::kernel::box_blur(7));
    if let Some(kernel) = kernel {
        let luma_samples = img.to_luma32f().into_vec();
        let img = convolve2d::DynamicMatrix::new(
            img.width() as usize,
            img.height() as usize,
            luma_samples,
        )
        .unwrap()
        .map(|x| x as f64);
        let img = convolve2d::convolve2d(&img, &kernel);

        let img = img.map(|x| x as f32);

        let new_img: ImageBuffer<Luma<f32>, Vec<f32>> = ImageBuffer::from_vec(
            img.get_width() as u32,
            img.get_height() as u32,
            img.get_data().to_vec(),
        )
        .unwrap();

        let a: ImageBuffer<Luma<u8>, Vec<u8>> = new_img.convert();
        a.save("output.png").unwrap();
    } else {
        eprintln!("Error creating kernel");
    }
}
