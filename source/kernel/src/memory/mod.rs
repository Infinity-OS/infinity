pub use self::area_frame_allocator::AreaFrameAllocator;
pub use self::paging::test_paging;
use self::paging::PhysicalAddress;

mod paging;
mod area_frame_allocator;

/// Size of a page
pub const PAGE_SIZE: usize = 4096;

/// Describes a memory frame
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame {
    number: usize,
}

impl Frame {
    /// Corresponding frame for a physical address
    fn containing_address(address: usize) -> Frame {
        Frame{ number: address / PAGE_SIZE }
    }

    /// Get the frame's starting address
    fn start_address(&self) -> PhysicalAddress {
        self.number * PAGE_SIZE
    }
}

pub trait FrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame>;
    fn deallocate_frame(&mut self, frame: Frame);
}

