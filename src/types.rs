
#[derive(Debug)]
pub enum Error {
    StartPathNotSetError,
    // Non-crate errors
    VarError(std::env::VarError),
    IOError(std::io::Error)
}

impl From<std::env::VarError> for Error {
    fn from(e: std::env::VarError) -> Self {
        Error::VarError(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IOError(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;