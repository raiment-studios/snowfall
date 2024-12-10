use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Bincode(#[from] bincode::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("Invalid file header '{0}'")]
    FileHeader(String),

    #[error("Invalid file version")]
    FileVersion(String),
}
