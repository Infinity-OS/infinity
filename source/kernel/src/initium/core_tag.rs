use tag::{TagType, VerifyTag};

#[repc(C, packed)]
pub struct CoreTag {
    /// Tag header
    tag_type: u32,
    size: u32,

    /// Physical address of the tag list
    tags_phys: usize,
    /// Total size of the tag list (rounded to 8 bytes)
    tags_size: u32,
    /// Pad
    _pad: u32,

    /// Physical add of the kernel image
    kernel_phys: usize,

    /// Virutal address of the boot stack
    stack_base: usize,
    /// Physical addres of the boot stack
    stack_phys: usize,
    /// Size of the stack
    stack_size: u32,
}

impl CoreTag {
    // TODO: implement the access functions
}
