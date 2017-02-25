use core::mem;

use super::memory_map::{MemoryMapTag,MemoryMapIter};
use super::tag::{TagType, Tag, TagIter};
use super::elf_sections::SectionTag;
use super::core_information::CoreTag;

/// Load the Initium tags
pub unsafe fn load(initium_addr: usize) -> &'static InitiumBootInfo {
    let initium_info = InitiumBootInfo::from_raw_parts(initium_addr);
    // assert!(initium_info.is_valid());
    initium_info
}

#[derive(Debug)]
#[repr(C)]
pub struct InitiumBootInfo {
    first_tag: Tag
}

impl InitiumBootInfo {
    unsafe fn from_raw_parts(initium_addr: usize) -> &'static InitiumBootInfo {
        &*(initium_addr as *const InitiumBootInfo)
    }

    /// Get the core information
    pub fn core_information(&self) -> Option<&CoreTag> {
        self.cast_find_tag::<CoreTag>(TagType::CoreInformation)
    }

    pub fn memory_map(&self) -> MemoryMapIter {
        let first_mem_tag = self.cast_find_tag::<MemoryMapTag>(TagType::PhysicalMemory);
        MemoryMapIter::new(first_mem_tag.unwrap())
    }

    pub fn elf_sections(&self) -> Option<&SectionTag> {
        self.cast_find_tag::<SectionTag>(TagType::ElfSections)
    }

    pub fn tags(&self) -> TagIter {
        TagIter::new(&self.first_tag)
    }

    pub fn find_tag(&self, tag_type: TagType) -> Option<&'static Tag> {
        self.tags().find(|tag| { tag.tag_type() == tag_type as usize })
    }

    pub fn cast_find_tag<T>(&self, tag_type: TagType) -> Option<&T> {
        self.find_tag(tag_type).map(|tag_ptr| {
            unsafe {
                tag_ptr.cast::<T>()
            }
        })
    }
}
