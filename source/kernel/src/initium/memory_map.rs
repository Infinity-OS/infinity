#![allow(dead_code)]
use super::tag::{TagType, VerifyTag};

#[repr(C)]
pub struct MemoryMapTag {
    tag_type: u32,
    size: u32,

    /// Start of the memory range.
    start: usize,
    /// Size of the memory range.
    length: usize,
    /// Type of the memory range.
    memory_type: u8,
}

/// Possible memory range types.
#[derive(Debug, PartialEq, Eq)]
pub enum MemoryType {
    /// Free, usable memory.
    Free            = 0,
    /// Kernel image and other non-reclaimable data
    Allocated       = 1,
    /// Memory reclaimable when boot information is no longer needed
    Recaimable      = 2,
    /// Temprary page tables for the kernel.
    PageTables      = 3,
    /// Stack set up for the kernel
    Stack           = 4,
    /// Module data
    Modules         = 5,
}


impl MemoryMapTag {
    pub fn base_address(&self) -> usize {
        self.start as usize
    }

    pub fn length(&self) -> usize {
        self.length as usize
    }

    pub fn memory_type(&self) -> MemoryType {
        match self.memory_type {
            0 => MemoryType::Free,
            1 => MemoryType::Allocated,
            2 => MemoryType::Recaimable,
            3 => MemoryType::PageTables,
            4 => MemoryType::Stack,
            5 => MemoryType::Modules,
            _ => panic!("Invalid memory tag!")
        }
    }

    /// Check if the tag entry is an usable memory location
    pub fn is_unsable_region(&self) -> bool {
        self.memory_type() == MemoryType::Free
    }
}

pub struct MemoryMapIter {
    current_entry: *const MemoryMapTag
}

impl MemoryMapIter {
    pub fn new(current_entry: *const MemoryMapTag) -> MemoryMapIter {
        MemoryMapIter {
            current_entry: current_entry
        }
    }
}

impl Iterator for MemoryMapIter {
    type Item = &'static MemoryMapTag;

    fn next(&mut self) -> Option<Self::Item> {
        // Loop ulting the tag isn't a memoty tag
        loop {
            // get the current entry
            let current_entry = unsafe { &*self.current_entry };

            // check if the current tag is a memory tag
            if current_entry.tag_type == TagType::PhysicalMemory as u32 {
                // get the next tag location
                let mut next_entry = self.current_entry as usize;
                next_entry += current_entry.size as usize;
                // align at 8 bytes
                next_entry = ((next_entry-1) & !0x7) + 0x8;

                self.current_entry = next_entry as *const MemoryMapTag;

                if current_entry.is_unsable_region() {
                    return Some(current_entry);
                }
            } else {
                // We have fallen off the end of the memory map.
                return None;
            }
        }
    }
}
