
#[derive(Debug)]
pub enum Error {
    // Non-crate errors
    VarError,
    IOError(std::io::Error)
}

impl From<std::env::VarError> for Error {
    fn from(_: std::env::VarError) -> Self {
        Error::VarError
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IOError(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;