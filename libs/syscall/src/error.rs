//! # Syscall errors

use core::result;

/// Represents an error
#[derive(Eq, PartialEq)]
pub struct Error {
    pub error_code: i32
}

/// Type to be used as a return from a Syscall
pub type Result<T> = result::Result<T, Error>;

impl Error {
    /// Create a new instance for Error.
    pub fn new(error_code: i32) -> Self {
        Error { error_code }
    }
}

/// No such process
pub const ESRCH: i32 = 3;
/// No such device
pub const ENODEV: i32 = 19;
