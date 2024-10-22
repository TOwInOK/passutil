use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Not found path: {0}")]
    NotFound(String),
    #[error("Iced happends")]
    Iced(#[from] iced::Error),
    #[error("fs error")]
    Fs(#[from] std::io::Error),
}
