use thiserror::Error;

/// アプリケーションのエラー型
#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Image processing error: {0}")]
    Image(#[from] image::ImageError),

    #[error("Logger initialization error: {0}")]
    Logger(#[from] fern::InitError),

    #[error("SetLogger error: {0}")]
    SetLogger(#[from] log::SetLoggerError),

    #[error("WalkDir error: {0}")]
    WalkDir(#[from] walkdir::Error),

    #[error("No valid image files found in directory: {0}")]
    NoImagesFound(String),

    #[error("Invalid file extension: {0}")]
    InvalidExtension(String),

    #[error("File name parsing error: {0}")]
    FileNameParsing(String),

    #[error("PDF creation error: {0}")]
    PdfCreation(String),
}

pub type Result<T> = std::result::Result<T, AppError>;