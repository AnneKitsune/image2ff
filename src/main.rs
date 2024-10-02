use clap::Parser;
use image::ImageFormat;
use std::path::Path;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Input file path
    #[clap(value_parser)]
    input: String,

    /// Output file path
    #[clap(value_parser)]
    output: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Load the image from the input file
    let img = image::open(&args.input)?;

    // Convert the image to RGBA format
    let rgba_img = img.to_rgba16();

    // Open the output file for writing
    let output_path = Path::new(&args.output);
    //let file = File::create(output_path)?;

    // Save the image in farbfeld format
    rgba_img.save_with_format(output_path, ImageFormat::Farbfeld)?;

    Ok(())
}

