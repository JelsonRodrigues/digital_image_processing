use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    // Input image to apply filter
    #[clap(short, long)]
    input_image: String,
}

fn main() {
    let args = Args::parse();

    println!("Input image: {}", args.input_image);
}
