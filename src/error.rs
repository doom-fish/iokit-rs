//! Errors returned by the `iokit` crate.

use core::fmt;

pub type Result<T> = std::result::Result<T, IoKitError>;

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum IoKitError {
    IoReturn(&'static str, i32),
    UnexpectedNull(&'static str),
    InvalidArgument(String),
}

impl fmt::Display for IoKitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IoReturn(operation, status) => {
                write!(f, "{operation} failed: IOReturn 0x{status:08x}")
            }
            Self::UnexpectedNull(what) => write!(f, "{what} returned NULL"),
            Self::InvalidArgument(message) => write!(f, "invalid argument: {message}"),
        }
    }
}

impl std::error::Error for IoKitError {}
