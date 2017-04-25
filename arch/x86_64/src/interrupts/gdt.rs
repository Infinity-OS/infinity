//! # Global Descriptor Table (GDT) manager module
//!
//! ## References
//! - [OSDev GDT](http://wiki.osdev.org/Global_Descriptor_Table)

use x86_64::structures::tss::TaskStateSegment;
use x86_64::structures::gdt::SegmentSelector;
use x86_64::PrivilegeLevel;

pub struct Gdt {
    table: [u64; 9],
    next_free: usize,
}

impl Gdt {
    pub fn new() -> Gdt {
        Gdt {
            table: [0; 9],
            next_free: 1
        }
    }

    /// Add a new entry to the GDT
    pub fn add_entry(&mut self, entry: Descriptor) -> SegmentSelector {
        let index = match entry {
            Descriptor::UserSegment(value) => self.push(value),
            Descriptor::SystemSegment(value_low, value_high) => {
                let index = self.push(value_low);
                self.push(value_high);
                index
            }
        };
        SegmentSelector::new(index as u16, PrivilegeLevel::Ring0)
    }

    /// Add a new entry to the GDT
    pub fn add_entry_user(&mut self, entry: Descriptor) -> SegmentSelector {
        let index = match entry {
            Descriptor::UserSegment(value) => self.push(value),
            Descriptor::SystemSegment(value_low, value_high) => {
                let index = self.push(value_low);
                self.push(value_high);
                index
            }
        };
        let new_seg = SegmentSelector::new(index as u16, PrivilegeLevel::Ring3);
        new_seg
    }

    /// This is used to control the number of entries on the GDT table.
    fn push(&mut self, value: u64) -> usize {
        if self.next_free < self.table.len() {
            let index = self.next_free;
            self.table[index] = value;
            self.next_free += 1;
            index
        } else {
            panic!("GDT full");
        }
    }

    /// Loads the GDT table into memory.
    pub fn load(&'static self) {
        use x86_64::instructions::tables::{DescriptorTablePointer, lgdt};
        use core::mem::size_of;

        let ptr = DescriptorTablePointer {
            base: self.table.as_ptr() as u64,
            limit: (self.table.len() * size_of::<u64>() - 1) as u16,
        };

        unsafe { lgdt(&ptr) };
    }
}

/// Represents a descriptor
pub enum Descriptor {
    UserSegment(u64),
    SystemSegment(u64, u64),
}

impl Descriptor {
    /// Creates a kernel mode segment
    pub fn kernel_code_segment() -> Descriptor {
        let flags = USER_SEGMENT | PRESENT | EXECUTABLE | LONG_MODE;
        Descriptor::UserSegment(flags.bits())
    }

    pub fn kernel_data_segment() -> Descriptor {
        let flags = USER_SEGMENT | PRESENT | LONG_MODE;
        Descriptor::UserSegment(flags.bits())
    }

    /// Creates a segment for the TLS
    pub fn thread_local_segment(offset: usize) -> Descriptor {
        use bit_field::BitField;

        // set the descriptor flags
        let flags = USER_SEGMENT | PRESENT | LONG_MODE;

        // get the bytes
        let mut bits = flags.bits();

        // set the offset
        let off = offset as u64;
        bits.set_bits(16..40, off.get_bits(0..24));
        bits.set_bits(56..64, off.get_bits(24..32));

        Descriptor::UserSegment(bits)
    }

    /// Create a new TSS segment
    ///
    /// ## Parameters
    ///
    /// * `tss` - Task state segment
    pub fn tss_segment(tss: &'static TaskStateSegment) -> Descriptor {
        use core::mem::size_of;
        use bit_field::BitField;

        let ptr = tss as *const _ as u64;

        let mut low = PRESENT.bits();
        // base
        low.set_bits(16..40, ptr.get_bits(0..24));
        low.set_bits(56..64, ptr.get_bits(24..32));
        // limit (the `-1` in needed since the bound is inclusive)
        low.set_bits(0..16, (size_of::<TaskStateSegment>() - 1) as u64);
        // type (0b1001 = available 64-bit tss)
        low.set_bits(40..44, 0b1001);

        let mut high = 0;
        high.set_bits(0..32, ptr.get_bits(32..64));

        Descriptor::SystemSegment(low, high)
    }

    /// Creates an user mode code segment
    pub fn user_code_segment() -> Descriptor {
        let flags = USER_SEGMENT | PRESENT | EXECUTABLE | LONG_MODE | RING_3;
        Descriptor::UserSegment(flags.bits())
    }

    /// Creates an user mode data segment
    pub fn user_data_segment() -> Descriptor {
        let flags = USER_SEGMENT | PRESENT | LONG_MODE | RING_3;
        Descriptor::UserSegment(flags.bits())
    }

    /// Creates an user mode TLS segment
    pub fn user_thread_local_segment(offset: usize) -> Descriptor {
        use bit_field::BitField;

        // set the descriptor flags
        let flags = USER_SEGMENT | PRESENT | LONG_MODE | RING_3;

        // get the bytes
        let mut bits = flags.bits();

        // set the offset
        let off = offset as u64;
        bits.set_bits(16..40, off.get_bits(0..24));
        bits.set_bits(56..64, off.get_bits(24..32));

        Descriptor::UserSegment(bits)
    }
}

bitflags! {
    flags DescriptorFlags: u64 {
        const CONFORMING        = 1 << 42,
        const EXECUTABLE        = 1 << 43,
        const USER_SEGMENT      = 1 << 44,
        const RING_3            = 3 << 45,
        const PRESENT           = 1 << 47,
        const LONG_MODE         = 1 << 53,
    }
}
