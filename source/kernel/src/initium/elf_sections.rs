use core::{slice,str};

// The size of an ELF header tag, in bytes. This was calculated
// from reading the ELF format specification.
const ELF_SECTION_HEADER_SIZE: u64 = 64;

#[derive(Debug)]
#[repr(C, packed)]
pub struct SectionTag {
    /// Tag header
    tag_type: u32,
    size: u32,

    /// Number of sections headers
    num: u32,
    /// Size of each section header
    entry_size: u32,
    /// Section name string table index
    shndx: u32,

    /// Pad
    _pad: u32,

    /// First sections
    first_section: ElfSectionHeader
}

impl SectionTag {
    /// Get all the ELF sections
    pub fn elf_sections(&'static self) -> ElfSectionIter {
        // HACK: calculate the next section address, to avoid the 0x00 empty sections
        let next_sections_addr = (&self.first_section as *const ElfSectionHeader as u64) + self.entry_size as u64;
        let next_section = unsafe { &*(next_sections_addr as *const ElfSectionHeader) };

        ElfSectionIter {
            current_section: &next_section,
            section_index: 0,
            total_sections: self.num -1 as u32,
            entry_size: self.entry_size as u64
        }
    }

    /// Get the number of sections
    pub fn section_count(&self) -> usize {
        self.num as usize
    }

    /// get the size of each section header
    pub fn entry_size(&self) -> usize {
        self.entry_size as usize
    }

    /// Get the table index
    pub fn string_table_index(&self) -> usize {
        self.shndx as usize
    }

    pub fn string_table(&self) -> &'static StringTable {
        let string_table_header = unsafe {
            (&self.first_section as *const ElfSectionHeader).offset(self.shndx as isize)
        };

        let string_table_ptr = unsafe {
            &*((*string_table_header).section_start_address() as *const StringTable)
        };

        string_table_ptr
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct ElfSectionHeader {
    sh_name: u32,
    sh_type: u32,
    sh_flags: u64,
    sh_addr: u64,
    sh_offset: u64,
    sh_size: u64,
    sh_link: u32,
    sh_info: u32,
    sh_addralign: u64,
    sh_entsize: u64
}

impl ElfSectionHeader {
    /// The type of an ELF section.
    pub fn section_type(&self) -> usize {
        self.sh_type as usize
    }

    /// The start address of an ELF section.
    pub fn section_start_address(&self) -> usize {
        self.sh_addr as usize
    }

    /// The end address of an ELF section.
    pub fn section_end_address(&self) -> usize {
        (self.sh_addr + self.sh_size) as usize
    }

    /// The size of an ELF section in bytes.
    pub fn size_bytes(&self) -> usize {
        self.sh_size as usize
    }

    /// ELF section flags.
    pub fn flags(&self) -> usize {
        self.sh_flags as usize
    }

    /// The size of an ELF header entry.
    pub fn entry_size(&self) -> usize {
        self.sh_entsize as usize
    }
}

struct ElfSectionIter {
    current_section: &'static ElfSectionHeader,
    section_index: u32,
    total_sections: u32,
    entry_size: u64
}

impl Iterator for ElfSectionIter {
    type Item = &'static ElfSectionHeader;

    fn next(&mut self) -> Option<Self::Item> {
        if self.section_index >= self.total_sections {
            return None;
        } else {
            // store the current section
            let section = self.current_section;

            // calculate the next section address
            let next_section_addr = (self.current_section as *const ElfSectionHeader as u64) + self.entry_size;

            // get the next section
            let next_section = unsafe {
                &*(next_section_addr as *const ElfSectionHeader)
            };

            // replace the current section with the next one
            self.current_section = next_section;
            self.section_index += 1;

            // return the current section
            Some(section)
        }
    }
}

pub struct StringTable {
    first_char: u8
}

impl StringTable {
    pub fn section_name(&self, section: &ElfSectionHeader) -> &'static str {
        let string_ptr = unsafe {
            (&self.first_char as *const u8).offset(section.sh_name as isize)
        };

        let string_length = unsafe {
            let mut length = 0;
            let mut current_char = *string_ptr.offset(length as isize);

            while current_char != 0x00 {
                length += 1;
                current_char = *string_ptr.offset(length as isize);
            }

            length
        };

        let string_slice = unsafe {
            slice::from_raw_parts(string_ptr, string_length)
        };

        str::from_utf8(string_slice).unwrap()
    }
}
