#[derive(Debug)]
#[repr(C, packed)]
pub struct CoreTag {
    /// Tag header
    tag_type: u32,
    size: u32,

    /// Physical address of the tag list
    tags_phys: usize,
    /// Total size of the tag list (rounded to 8 bytes)
    tags_size: u32,
    _pad: u32,

    /// Physical address of the kernel image
    kernel_phys: usize,

    /// Virtual address of the boot stack
    stack_base: usize,
    /// Physical address to the boot stack
    stack_phys: usize,
    /// Size of the boot stack
    stack_size: u32,
}

impl CoreTag {
    pub fn tags_phys_addr(&self) -> usize {
        self.tags_phys as usize
    }

    pub fn tags_size(&self) -> u32 {
        self.tags_size as u32
    }

    pub fn kernel_phys_addr(&self) -> usize {
        self.kernel_phys as usize
    }

    pub fn stack_base_addr(&self) -> usize {
        self.stack_base as usize
    }

    pub fn stack_size(&self) -> u32 {
        self.stack_size as u32
    }
}
