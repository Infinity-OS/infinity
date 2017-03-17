#![feature(const_fn)]

#[derive(Debug)]
struct BumpAllocator {
    heap_start: usize,
    heap_size: usize,
    next: usize,
}

impl BumpAllocator {
    /// Create a new allocator, which uses the memory in the
    /// range [heap_start, heap_start + heap_size).
    const fn new(heap_start: usize, heap_size: usize) -> BumpAllocator {
        BumpAllocator {
            heap_start: heap_start,
            heap_size: heap_size,
            next: heap_start,
        }
    }

    /// Allocates a block of memory with the given size and alignment.
    fn allocate(&mut self, size: usize, align: usize) -> Option<*mut u8> {
        let alloc_start = align_up(self.next, align);
        let alloc_end = alloc_start.saturating_add(size);

        if alloc_end <= self.heap_start + self.heap_size {
            self.next = alloc_end;
            Some(alloc_start as *mut u8)
        } else {
            None
        }
    }

    /// Align downwards. Returns the greatest x with alignment `align`
/// so that x <= addr. The alignment must be a power of 2.
    pub fn align_down(addr: usize, align: usize) -> usize {
        if align.is_power_of_two() {
            addr & !(align - 1)
        } else if align == 0 {
            addr
        } else {
            panic!("`align` must be a power of 2");
        }
    }

    /// Align upwards. Returns the smallest x with alignment `align`
    /// so that x >= addr. The alignment must be a power of 2.
    pub fn align_up(addr: usize, align: usize) -> usize {
        align_down(addr + align - 1, align)
    }
}