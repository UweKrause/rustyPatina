use anyhow::Context;
use std::path::PathBuf;

use clap::Parser;
use image::imageops::FilterType;
use image::io::Reader as ImageReader;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.subcommand {
        Subcommand::Blur(blur) => {
            let img = ImageReader::open(&blur.input.path)
                .with_context(|| format!("Failed to open input file: {:?}", blur.input.path))?
                .decode()
                .context("Failed to decode input image")?;

            let blurred = img.blur(blur.sigma);

            blurred.save("out.jpg")?;
        }
        Subcommand::Resize(resize) => {
            let img = ImageReader::open(resize.input.path)?.decode()?;
            let resized = img.resize(resize.width, resize.height, FilterType::Gaussian);
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

#[derive(Parser)]
struct InputPath {
    /// path to input
    path: PathBuf,
}

/// Image sampling Performs a Gaussian blur on the supplied image.
#[derive(Parser)]
struct Blur {
    #[clap(flatten)]
    input: InputPath,
    /// sigma is a measure of how much to blur by.
    sigma: f32,
}

/// Image sampling Resize the supplied image to the specified dimensions.
#[derive(Parser)]
struct Resize {
    #[clap(flatten)]
    input: InputPath,
    /// new width dimension.
    width: u32,
    /// new height dimension.
    height: u32,
}
