#![allow(dead_code)]
use core::mem;

pub trait VerifyTag {
    fn is_valid(&self) -> bool;
}

/// A Initium tag struture is a queryable blob of bytes. The implementation presently
/// assumes that the size is at least 8 bytes (for the end tag), and does not check this.
#[derive(Copy, Clone, Debug)]
pub enum TagType {
    EndTag           = 0,   // End of tag list.
    CoreInformation  = 1,   // Core information tag (always present).
    KernelOption     = 2,   // Kernel option.
    PhysicalMemory   = 3,   // Physical memory range.
    VirtualMemory    = 4,   // Virtual memory range.
    PageTables       = 5,   // Page table information (architecture-specific).
    BootModule       = 6,   // Boot module.
    VideoInformation = 7,   // Video mode information.
    BootDevice       = 8,   // Boot device information.
    LogBuffer        = 9,   // Kernel log buffer.
    ElfSections      = 10,  // ELF section information.
    BIOSE820         = 11,  // BIOS address range descriptor (PC-specific).
    EFIInformation   = 12,  // EFI friwmware information
}

/// Initium information tag structure.
#[derive(Debug)]
#[repr(C)]
pub struct Tag {
    pub tag_type: u32,
    pub size: u32,
    // The tag data follows these two fields.
}

impl Tag {
    fn is_end_tag(&self) -> bool {
        self.tag_type == TagType::EndTag as u32
    }

    pub fn size(&self) -> usize {
        self.size as usize
    }

    pub fn tag_type(&self) -> usize {
        self.tag_type as usize
    }

    /// The function `cast` casts a generic `Tag` to a particular Initium heade tag. This function
    /// is really dangerous and should only be used for parsing the Initium info struct.
    pub unsafe fn cast<T>(&self) -> &T {
        mem::transmute::<&Tag, &T>(self)
    }
}

pub struct TagIter {
    current: *const Tag,
}

impl TagIter {
    pub fn new(first_tag: *const Tag) -> TagIter {
        TagIter {
            current: first_tag
        }
    }
}

impl Iterator for TagIter {
    // create a new type for the tags
    type Item = &'static Tag;

    /// Iterate all tags
    ///
    /// ## Return
    ///
    /// Returns the reference for the next tag on the list, until it doesn't found the end tag
    fn next(&mut self) -> Option<&'static Tag> {
        let current = unsafe { &*self.current };

        match current {
            // end tag
            &Tag{ tag_type: 0, .. } => None,
            tag => {
                // Jump to the next tag
                let mut tag_address = self.current as usize;
                tag_address += tag.size as usize;
                // align at 8 bytes
                tag_address = ((tag_address-1) & !0x7) + 0x8;
                self.current = tag_address as *const Tag;

                Some(tag)
            },
        }
    }
}
