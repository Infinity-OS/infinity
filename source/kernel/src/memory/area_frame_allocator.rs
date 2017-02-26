use memory::{Frame, FrameAllocator};
use initium::memory_map::{MemoryMapIter, MemoryMapTag};

pub struct AreaFrameAllocator {
    //increased every time we return a frame.
    next_free_frame: Frame,
    //memory area that contains next_free_frame
    current_area: Option<&'static MemoryMapTag>,
    //ext_free_frame leaves this area, we will look for the next one in areas
    areas: MemoryMapIter,
    //used to avoid using already allocated zones
    kernel_start: Frame,
    kernel_end: Frame,
    initium_start: Frame,
    initium_end: Frame,
}

impl AreaFrameAllocator {
    pub fn new(kernel_start: usize,
               kernel_end: usize,
               initium_start: usize,
               initium_end: usize,
               memory_areas: MemoryMapIter)
               -> AreaFrameAllocator {
        let mut allocator = AreaFrameAllocator {
            next_free_frame: Frame::containing_address(0),
            current_area: None,
            areas: memory_areas,
            kernel_start: Frame::containing_address(kernel_start),
            kernel_end: Frame::containing_address(kernel_end),
            initium_start: Frame::containing_address(initium_start),
            initium_end: Frame::containing_address(initium_end),
        };
        allocator.choose_next_area();
        allocator
    }

    fn choose_next_area(&mut self) {
        self.current_area = self.areas
            .clone()
            .filter(|area| {
                let address = area. base_address() + area.length() - 1;
                Frame::containing_address(address as usize) >=
                    self.next_free_frame
            })
            .min_by_key(|area| area. base_address());

        if let Some(area) = self.current_area {
            let start_frame = Frame::containing_address(area. base_address() as usize);
            if self.next_free_frame < start_frame {
                self.next_free_frame = start_frame;
            }
        }
    }
}

impl FrameAllocator for AreaFrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame> {
        if let Some(area) = self.current_area {
            // "clone" the frame to return it if it's free. Frame doesn't
            // implement Clone, but we can construct an identical frame.
            let frame = Frame { number: self.next_free_frame.number };

            // the last frame of the current area
            let current_area_last_frame = {
                let address = area. base_address() + area.length() - 1;
                Frame::containing_address(address as usize)
            };

            if frame > current_area_last_frame {
                // all frames of current area are used, switch to next area
                self.choose_next_area();
            } else if frame >= self.kernel_start && frame <= self.kernel_end {
                // `frame` is used by the kernel
                self.next_free_frame = Frame { number: self.kernel_end.number + 1 };
            } else if frame >= self.initium_start && frame <= self.initium_end {
                // `frame` is used by the multiboot information structure
                self.next_free_frame = Frame { number: self.initium_end.number + 1 };
            } else {
                // frame is unused, increment `next_free_frame` and return it
                self.next_free_frame.number += 1;
                return Some(frame);
            }
            // `frame` was not valid, try it again with the updated `next_free_frame`
            self.allocate_frame()
        } else {
            None // no free frames left
        }
    }

    fn deallocate_frame(&mut self, _frame: Frame) {
        unimplemented!()
    }
}