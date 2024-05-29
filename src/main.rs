use anyhow::Context;
use std::path::PathBuf;

use clap::Parser;
use image::imageops::FilterType;
use image::io::Reader as ImageReader;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.subcommand {
        Subcommand::Blur(blur) => {
            let img = ImageReader::open(&blur.input)
                .with_context(|| format!("Failed to open input file: {:?}", blur.input))?
                .decode()
                .context("Failed to decode input image")?;

            let blurred = img.blur(blur.sigma);

            blurred.save("out.jpg")?;
        }
        Subcommand::Resize(resize) => {
            let img = ImageReader::open(resize.input)?.decode()?;
            let resized = img.resize(resize.w, resize.h, FilterType::Gaussian);
            resized.save("out.jpg")?;
        }
        Subcommand::Make => todo!(),
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
    Make,
}

/// Image sampling Performs a Gaussian blur on the supplied image.
#[derive(Parser)]
struct Blur {
    /// path to input
    input: PathBuf,
    /// sigma is a measure of how much to blur by.
    sigma: f32,
}

#[derive(Parser)]
struct Resize {
    input: PathBuf,
    w: u32,
    h: u32,
}
