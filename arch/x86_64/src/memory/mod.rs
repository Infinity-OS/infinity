pub use self::area_frame_allocator::AreaFrameAllocator;
pub use self::paging::ActivePageTable;
pub use self::paging::remap_the_kernel;
pub use self::stack_allocator::Stack;

use self::paging::PhysicalAddress;
use multiboot2::BootInformation;

/// Frame allocator.
mod area_frame_allocator;

/// Paging system.
pub mod paging;

/// Stack allocator.
mod stack_allocator;

/// Size of a page
pub const PAGE_SIZE: usize = 4096;

/// A memory map area
#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct MemoryArea {
    pub base_addr: u64,
    pub length: u64,
    pub _type: u32,
    pub acpi: u32
}

#[derive(Clone)]
pub struct MemoryAreaIter {
    index: usize
}

impl MemoryAreaIter {
    pub fn new() -> Self {
        MemoryAreaIter {
            index: 0
        }
    }
}

impl Iterator for MemoryAreaIter {
    type Item = &'static MemoryArea;
    fn next(&mut self) -> Option<&'static MemoryArea> {
        while self.index < unsafe { MEMORY_MAP.len() } {
            // get the entry in the current index
            let entry = unsafe { &MEMORY_MAP[self.index] };

            // increment the index
            self.index += 1;

            if entry._type == 1 {
                return Some(entry)
            }
        }

        None
    }
}

/// The current memory map.
static mut MEMORY_MAP: [MemoryArea; 512] = [MemoryArea { base_addr: 0, length: 0, _type: 0, acpi: 0 }; 512];

/// Initialize the memory system
///
/// ## Returns
///
/// The memory controller that servers a simple interface to manage the memory allocation.
pub fn init(cpu_id: usize, boot_info: &BootInformation) -> (MemoryController, usize) {
    // insure that this function is only called once
    assert_has_not_been_called!("memory::init must be called only once");

    // get the bootloader memory tag
    let memory_map_tag = boot_info.memory_map_tag().expect("Memory map tag required");

    // get the elf sections bootloader tag
    let elf_sections_tag = boot_info.elf_sections_tag().expect("Elf sections tag required");

    // get the kernel start address
    let kernel_start = elf_sections_tag.sections().map(|s| s.addr).min().unwrap();

    // get the kernel end address
    let kernel_end = elf_sections_tag.sections().map(|s| s.addr + s.size).max().unwrap();

    // make a entire copy from the multiboot areas. This is needed in order to put the MemoryController available to the kernel.
    unsafe {
        let mut index = 0;
        for cur_area in memory_map_tag.memory_areas() {
            let mut entry = &mut MEMORY_MAP[index];

            entry.base_addr = cur_area.base_addr;
            entry.length = cur_area.length;
            entry._type = 1;

            // increment the index
            index += 1;
        }
    }

    // initialize the frame allocator
    let mut frame_allocator = AreaFrameAllocator::new(kernel_start as usize,
                                                      kernel_end as usize,
                                                      boot_info.start_address(),
                                                      boot_info.end_address(),
                                                      MemoryAreaIter::new());
    // remap the kernel
    let (mut active_table, tcb_offset) = unsafe { remap_the_kernel(cpu_id, &mut frame_allocator, boot_info) };

    // remap heap
    use self::paging::Page;
    use hole_list_allocator::{HEAP_START, HEAP_SIZE};

    let heap_start_page = Page::containing_address(HEAP_START);
    let heap_end_page = Page::containing_address(HEAP_START + HEAP_SIZE - 1);

    for page in Page::range_inclusive(heap_start_page, heap_end_page) {
        active_table.map(page, paging::WRITABLE, &mut frame_allocator);
    }

    // remap Stack
    let stack_allocator = {
        // calculate the start and end address of the stack
        let stack_alloc_start = heap_end_page + 1;
        let stack_alloc_end = stack_alloc_start + 100;

        // create a new page range with the stack start address and end address
        let stack_alloc_range = Page::range_inclusive(stack_alloc_start, stack_alloc_end);

        // create a StackAllocator instance
        stack_allocator::StackAllocator::new(stack_alloc_range)
    };

    // create the memory controller instance
    let memory_controller = MemoryController {
        active_table: active_table,
        frame_allocator: frame_allocator,
        stack_allocator: stack_allocator
    };

    // returns the memory controller and the tcb offset
    (memory_controller, tcb_offset)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame {
    number: usize,
}

impl Frame {
    pub fn containing_address(address: usize) -> Frame {
        Frame { number: address / PAGE_SIZE }
    }

    pub fn start_address(&self) -> PhysicalAddress {
        self.number * PAGE_SIZE
    }

    pub fn range_inclusive(start: Frame, end: Frame) -> FrameIter {
        FrameIter {
            start: start,
            end: end
        }
    }

    fn clone(&self) -> Frame {
        Frame { number: self.number }
    }
}

pub struct FrameIter {
    start: Frame,
    end: Frame,
}

impl Iterator for FrameIter {
    type Item = Frame;

    fn next(&mut self) -> Option<Frame> {
        if self.start <= self.end {
            let frame = self.start.clone();
            self.start.number += 1;
            Some(frame)
        } else {
            None
        }
    }
}

pub trait FrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame>;
    fn deallocate_frame(&mut self, frame: Frame);
}

pub struct MemoryController {
    pub active_table: paging::ActivePageTable,
    pub frame_allocator: AreaFrameAllocator,
    stack_allocator: stack_allocator::StackAllocator,
}

impl MemoryController {
    /// Alloc a new stack with the given size.
    ///
    /// ## Params
    /// * `size_in_pages` - number of pages to alloc.
    ///
    /// ## Returns
    /// Stack instance.
    pub fn alloc_stack(&mut self, size_in_pages: usize) -> Option<Stack> {
        let &mut MemoryController {
            ref mut active_table,
            ref mut frame_allocator,
            ref mut stack_allocator } = self;

        stack_allocator.alloc_stack(active_table, frame_allocator, size_in_pages)
    }

    /// Maps the page to the frame with the provided flags.
    /// The `PRESENT` flag is added by default. Needs a `FrameAllocator` as it might need to create
    /// new page tables.
    pub fn map_to(&mut self, page: paging::Page, frame: Frame, flags: paging::entry::EntryFlags) {
        self.active_table.map_to(page, frame, flags, &mut self.frame_allocator);
    }

    /// Maps the page to some free frame with the provided flags.
    /// The free frame is allocated from the given `FrameAllocator`.
    pub fn map(&mut self, active_table: &mut ActivePageTable, page: paging::Page, flags: paging::entry::EntryFlags) {
        active_table.map(page, flags, &mut self.frame_allocator);
    }

    /// Flush the TLB table
    pub fn flush_all(&mut self) {
        self.active_table.flush_all();
    }
}
