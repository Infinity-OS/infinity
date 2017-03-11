#![allow(dead_code)]

/// Tag containing page table information (AMD64)
#[repr(C, packed)]
pub struct PageTablesTag {
    tag_type: u32,
    size: u32,

    /// Physical address of the page directory
    pml4: usize,
    /// Virtual address of recursive mapping
    map: usize,
}

impl PageTablesTag {
    pub fn pml4(&self) -> usize {
        self.pml4 as usize
    }

    pub fn map(&self) -> usize {
        self.map as usize
    }
}
