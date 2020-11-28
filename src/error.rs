pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IOError,
    NoneError,
    OutOfRangeError,
    StringConversionError,
    UnsupportedError,
    UninitializedError,
}

impl From<std::io::Error> for Error {
    fn from(_err: std::io::Error) -> Self {
        Error::IOError
    }
}

impl From<mmap::MapError> for Error {
    fn from(_err: mmap::MapError) -> Self {
        Error::IOError
    }
}

impl From<std::num::TryFromIntError> for Error {
    fn from(_err: std::num::TryFromIntError) -> Self {
        Error::OutOfRangeError
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(_err: std::str::Utf8Error) -> Self {
        Error::StringConversionError
    }
}
