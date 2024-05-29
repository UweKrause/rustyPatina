use std::path::PathBuf;

use clap::{Parser};
use image::imageops::FilterType;
use image::io::Reader as ImageReader;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.subcommand {
        Subcommand::Blur(blur) => {
            let img = ImageReader::open(blur.input)?.decode()?;
            let blurred = img.blur(blur.sigma);
            blurred.save("out.jpg")?;
        }
        Subcommand::Resize(resize) => {
            let img = ImageReader::open(resize.input)?.decode()?;
            let resized = img.resize(resize.w, resize.h, FilterType::Gaussian);
            resized.save("out.jpg")?;
        }
    }

    Ok(())
}

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    subcommand: Subcommand,
}

#[derive(clap::Subcommand)]
enum Subcommand {
    Blur(Blur),
    Resize(Resize),
}

#[derive(Parser)]
struct Blur {
    input: PathBuf,
    sigma: f32,
}

#[derive(Parser)]
struct Resize {
    input: PathBuf,
    w: u32,
    h: u32,
}



