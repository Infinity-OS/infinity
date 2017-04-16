pub const SYS_CLASS_FILE: usize = 0x2000_0000;

// Types of arguments
pub const SYS_ARG_MSLICE: usize = 0x0200_0000;

pub const SYS_FSTAT: usize = SYS_CLASS_FILE | SYS_ARG_MSLICE | 28;
