//! Errors returned by the `iokit` crate.

use core::fmt;

/// Convenience result alias returned by the safe `IOKit` wrappers.
pub type Result<T> = std::result::Result<T, IoKitError>;

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
/// Errors produced by the safe wrappers around `IOReturn`- and `CFTypeRef`-based APIs.
pub enum IoKitError {
    /// Wraps a failing `IOReturn` from an `IOKit` framework call.
    IoReturn(&'static str, i32),
    /// Wraps an unexpected null pointer from an `IOKit` or Core Foundation call.
    UnexpectedNull(&'static str),
    /// Wraps an invalid text or binary payload passed to an `IOCFUnserialize*` entry point.
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
