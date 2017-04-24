//! System call handlers

// import external syscall lib
extern crate syscall;

pub use self::syscall::{error, scheme, flag, data, number};

// export everything
pub use self::fs::*;
pub use self::process::*;

use self::error::{Error, Result};

/// Filesystem syscalls
pub mod fs;

/// Process syscalls
pub mod process;


