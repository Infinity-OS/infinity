//! Memory logic for the context sub-system.
//!
//! Some parts of this code are based on the Redox OS.

use alloc::arc::{Arc, Weak};
use spin::Mutex;

use arch::memory::paging::{ActivePageTable, Page, PageIter, VirtualAddress};
use arch::memory::paging::entry::EntryFlags;
use arch::start;

#[derive(Clone, Debug)]
pub enum SharedMemory {
    Owned(Arc<Mutex<Memory>>),
    Borrowed(Weak<Mutex<Memory>>)
}

impl SharedMemory {
    /// Mark as borrowed.
    pub fn borrow(&self) -> SharedMemory {
        match *self {
            SharedMemory::Owned(ref memory_lock) => SharedMemory::Borrowed(Arc::downgrade(memory_lock)),
            SharedMemory::Borrowed(ref memory_lock) => SharedMemory::Borrowed(memory_lock.clone())
        }
    }
}

#[derive(Debug)]
pub struct Memory {
    /// Start address for the memory zone.
    start: VirtualAddress,
    /// Size of the address space.
    size: usize,
    /// Flags for this address space.
    flags: EntryFlags
}

impl Memory {
    /// Create a new Memory instance.
    pub fn new(start: VirtualAddress, size: usize, flags: EntryFlags, clear: bool) -> Self {
        let mut memory = Memory {
            start,
            size,
            flags
        };

        // map the memory and clean it if requested
        memory.map(clear);

        memory
    }

    /// Get the start address for this memory space.
    pub fn start_address(&self) -> VirtualAddress {
        self.start
    }

    /// Get the size of the address space.
    pub fn size(&self) -> usize {
        self.size
    }

    /// Get the flags associated to this memory zone.
    pub fn flags(&self) -> EntryFlags {
        self.flags
    }

    /// Get an iterator with the page range for this memory zone.
    pub fn pages(&self) -> PageIter {
        let start_page = Page::containing_address(self.start);
        let end_page = Page::containing_address((self.start as usize + self.size - 1) as VirtualAddress);
        Page::range_inclusive(start_page, end_page)
    }

    /// Map a new space on the virtual memory for this memory zone.
    fn map(&mut self, clean: bool) {
        // create a new active page table
        let mut active_table = unsafe { ActivePageTable::new() };

        // get memory controller
        if let Some(ref mut memory_controller) = *::MEMORY_CONTROLLER.lock() {
            for page in self.pages() {
                memory_controller.map(&mut active_table, page, self.flags);
            }
        } else {
            panic!("Memory controller required");
        }
    }
}
