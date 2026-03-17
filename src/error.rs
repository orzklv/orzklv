use image::ImageError;
use std::io;
use tapciify::AsciiArtConverterError;
use thiserror::Error;

/// Self provided errors
#[derive(Error, Debug)]
pub enum Error {
    #[error("trouble while processing file: {0}")]
    IO(#[from] io::Error),
    #[error("something happened while processing image: {0}")]
    ImageProcessing(#[from] ImageError),
    #[error("ascii conversion error: {0}")]
    AsciiProcessing(#[from] AsciiArtConverterError),
}

/// Private Result type
pub type Result<T> = std::result::Result<T, Error>;
