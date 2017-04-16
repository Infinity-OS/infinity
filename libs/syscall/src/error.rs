//! # Syscall errors

use core::{fmt, result};

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

    /// Get the value from a Result object.
    pub fn mux(result: Result<usize>) -> usize {
        match result {
            Ok(value) => value,
            Err(error) => -error.error_code as usize,
        }
    }

    /// Check if the passed value is a success code or an error.
    ///
    /// If it is an error return the correspondent error object, otherwise just return a Ok.
    pub fn demux(value: usize) -> Result<usize> {
        let error_code = -(value as i32);
        if error_code >= 1 && error_code < STR_STATE.len() as i32 {
            Err(Error::new(error_code))
        } else {
            Ok(value)
        }
    }

    /// Convert the state to a string.
    pub fn text(&self) -> &str {
        if let Some(description) = STR_STATE.get(self.error_code as usize) {
            description
        } else {
            "Unknown Error"
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        f.write_str(self.text())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        f.write_str(self.text())
    }
}

/// No such file or directory
pub const ENOENT: i32 = 2;
/// No such process
pub const ESRCH: i32 = 3;
/// Bad file number
pub const EBADF: i32 = 9;
/// No such device
pub const ENODEV: i32 = 19;
/// Not a directory
pub const ENOTDIR: i32 = 20;
/// Too many open files
pub const EMFILE: i32 = 24;
/// Function not implemented
pub const ENOSYS: i32 = 38;

/// A string representation of each available state.
pub static STR_STATE: [&'static str; 39] = [
    "Success",
    "",
    "No such file or directory",
    "No such process",
    "",
    "",
    "",
    "",
    "",
    "Bad file number",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "No such device",
    "Not a directory",
    "",
    "",
    "",
    "Too many open files",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "Function not implemented"
];
