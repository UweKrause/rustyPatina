use image::io::Reader as ImageReader;

use clap::Parser;

fn main() {
    let args = Cli::parse();

    let path = args.path;
    let out = args.out;

    let img = ImageReader::open(path).expect("PROBLEM").decode();
    let blurred = img.expect("PROBLEM 2").blur(5.5);
    blurred.save(out).expect("PROBLEM 3");
}

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,

    out: std::path::PathBuf,
}



