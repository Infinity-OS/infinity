//! System call handlers

// import external syscall lib
extern crate syscall;

pub use self::syscall::{error, scheme, flag, data, number};

// export everything
pub use self::fs::*;

use self::error::{Error, Result};

/// Filesystem syscalls
pub mod fs;

