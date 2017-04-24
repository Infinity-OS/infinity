pub const SYS_CLASS_FILE: usize = 0x2000_0000;
pub const SYS_CLASS_PATH: usize=0x1000_0000;

// Types of arguments
pub const SYS_ARG_MSLICE: usize = 0x0200_0000;

// Return types
pub const SYS_RET_FILE: usize = 0x0010_0000;

pub const SYS_OPEN: usize =     SYS_CLASS_PATH | SYS_RET_FILE | 5;

pub const SYS_READ: usize   = SYS_CLASS_FILE | SYS_ARG_MSLICE | 3;
pub const SYS_FSTAT: usize  = SYS_CLASS_FILE | SYS_ARG_MSLICE | 28;
