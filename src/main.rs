mod align;
mod error;

use error::*;
use image::ImageReader;
use image::imageops::FilterType;
use std::io::Cursor;
use tapciify::{
    AsciiArtConverter, AsciiArtConverterOptions, CustomRatioResize, DEFAULT_FONT_RATIO,
};
/// Maximum allowed width
static MAX_WIDTH: u32 = 64;

/// Pre-included content within binary
static IMAGE: &[u8] = include_bytes!("../current-assets/tile.webp");

fn main() -> Result<()> {
    let cursor = Cursor::new(IMAGE);

    let img = ImageReader::new(cursor)
        .with_guessed_format()
        .map_err(Error::IO)
        .and_then(|img| img.decode().map_err(Error::ImageProcessing))?;

    let result = img
        .resize_custom_ratio(
            Some(MAX_WIDTH),
            None,
            DEFAULT_FONT_RATIO,
            FilterType::Triangle,
        )
        .ascii_art(&AsciiArtConverterOptions {
            colored: true,
            ..Default::default()
        })
        .map_err(Error::AsciiProcessing)?;

    println!(
        "{}\n\n{}",
        result,
        align::Alignment::from(' ').fill(MAX_WIDTH, "Hello, World!")
    );

    Ok(())
}
