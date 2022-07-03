use clap::Parser;
use fast_hilbert::*;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser)]
    input_filename: String,

    #[clap(short, long, value_parser)]
    output_filename: String,
}

fn main() {
    println!("Hilbertizer!");
    let args = Args::parse();

    let input_path = std::path::Path::new(&args.input_filename);
    let output_path = std::path::Path::new(&args.output_filename);

    let input_image = bmp::open(input_path).unwrap();
    let mut output_image = bmp::Image::new(input_image.get_width(), input_image.get_height());

    let image_length = input_image.get_width() * input_image.get_height();

    for idx in 0..image_length {
        let px = idx % input_image.get_width();
        let py = idx / input_image.get_width();
        let pixel = input_image.get_pixel(px, py);

        let (output_px, output_py) = h2xy(idx as u64);
        output_image.set_pixel(output_px, output_py, pixel);
    }

    output_image.save(output_path).unwrap();
}
