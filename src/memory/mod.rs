pub use self::area_frame_allocator::AreaFrameAllocator;
pub use self::paging::remap_the_kernel;
use self::paging::PhysicalAddress;
use multiboot2::BootInformation;

mod area_frame_allocator;
mod paging;
mod stack_allocator;

/// Size of a page
pub const PAGE_SIZE: usize = 4096;

/// Initialize the memory system
pub fn init(boot_info: &BootInformation) {
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

    // initialize the frame allocator
    let mut frame_allocator = AreaFrameAllocator::new(kernel_start as usize,
                                                      kernel_end as usize,
                                                      boot_info.start_address(),
                                                      boot_info.end_address(),
                                                      memory_map_tag.memory_areas());

    // remap the kernel
    remap_the_kernel(&mut frame_allocator, boot_info);
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame {
    number: usize,
}

impl Frame {
    fn containing_address(address: usize) -> Frame {
        Frame { number: address / PAGE_SIZE }
    }

    fn start_address(&self) -> PhysicalAddress {
        self.number * PAGE_SIZE
    }

    fn range_inclusive(start: Frame, end: Frame) -> FrameIter {
        FrameIter {
            start: start,
            end: end
        }
    }

    fn clone(&self) -> Frame {
        Frame { number: self.number }
    }
}

struct FrameIter {
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
