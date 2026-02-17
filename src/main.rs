use image::ImageReader;
use image::imageops::FilterType;
use std::io::Cursor;
use tapciify::{
    AsciiArtConverter, AsciiArtConverterOptions, CustomRatioResize, DEFAULT_FONT_RATIO,
};

fn main() {
    let img = ImageReader::new(Cursor::new(include_bytes!(
        "../past-assets/assets-anime/Konata.webp"
    )))
    .with_guessed_format()
    .expect("Couldn't read image format")
    .decode()
    .expect("Failed to decode image");

    let result = img
        .resize_custom_ratio(Some(64), None, DEFAULT_FONT_RATIO, FilterType::Triangle)
        .ascii_art(&AsciiArtConverterOptions {
            colored: true,
            ..Default::default()
        })
        .expect("Could not convert image to ascii art");

    println!("{}\nHewwo, GitHub! ^w^", result);
}
