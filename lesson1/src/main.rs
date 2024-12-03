use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    #[clap(short, long)]
    input_image: String,
}

fn main() {
    let args = Args::parse();

    println!("Input image: {}", args.input_image);
}
