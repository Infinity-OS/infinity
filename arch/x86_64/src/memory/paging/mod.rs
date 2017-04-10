//! # Paging
//! Some code was borrowed from [Phil Opp's Blog](http://os.phil-opp.com/modifying-page-tables.html)

use core::{mem, ptr};
use core::ops::{Add, Deref, DerefMut};
use memory::{PAGE_SIZE, Frame, FrameAllocator};
use self::temporary_page::TemporaryPage;
pub use self::mapper::Mapper;
pub use self::entry::*;
use multiboot2::BootInformation;

pub mod entry;
mod mapper;
mod table;
mod temporary_page;

const ENTRY_COUNT: usize = 512;

// TODO move this to a separated module
const KERNEL_PERCPU_OFFSET: usize = 0xc000_0000;
/// Size of kernel percpu variables
const KERNEL_PERCPU_SIZE: usize = 64 * 1024; // 64kb

pub type PhysicalAddress = usize;
pub type VirtualAddress = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Page {
    number: usize,
}

impl Page {
    pub fn containing_address(address: VirtualAddress) -> Page {
        assert!(address < 0x0000_8000_0000_0000 || address >= 0xffff_8000_0000_0000,
        "invalid address: 0x{:x}",
        address);
        Page { number: address / PAGE_SIZE }
    }

    pub fn start_address(&self) -> usize {
        self.number * PAGE_SIZE
    }

    fn p4_index(&self) -> usize {
        (self.number >> 27) & 0o777
    }
    fn p3_index(&self) -> usize {
        (self.number >> 18) & 0o777
    }
    fn p2_index(&self) -> usize {
        (self.number >> 9) & 0o777
    }
    fn p1_index(&self) -> usize {
        (self.number >> 0) & 0o777
    }

    pub fn range_inclusive(start: Page, end: Page) -> PageIter {
        PageIter {
            start: start,
            end: end,
        }
    }
}

impl Add<usize> for Page {
    type Output = Page;

    fn add(self, rhs: usize) -> Page {
        Page { number: self.number + rhs }
    }
}

#[derive(Clone)]
pub struct PageIter {
    start: Page,
    end: Page,
}

impl Iterator for PageIter {
    type Item = Page;

    fn next(&mut self) -> Option<Page> {
        if self.start <= self.end {
            let page = self.start;
            self.start.number += 1;
            Some(page)
        } else {
            None
        }
    }
}

pub struct ActivePageTable {
    mapper: Mapper,
}

impl Deref for ActivePageTable {
    type Target = Mapper;

    fn deref(&self) -> &Mapper {
        &self.mapper
    }
}

impl DerefMut for ActivePageTable {
    fn deref_mut(&mut self) -> &mut Mapper {
        &mut self.mapper
    }
}

impl ActivePageTable {
    pub unsafe fn new() -> ActivePageTable {
        ActivePageTable { mapper: Mapper::new() }
    }

    pub fn with<F>(&mut self,
                   table: &mut InactivePageTable,
                   temporary_page: &mut temporary_page::TemporaryPage,
                   f: F)
        where F: FnOnce(&mut Mapper)
    {
        use x86_64::registers::control_regs;

        {
            // backup the original CR3 address in order to restore it lately
            let backup = Frame::containing_address(control_regs::cr3().0 as usize);

            // map temporary_page to current p4 table
            let p4_table = temporary_page.map_table_frame(backup.clone(), self);

            // overwrite recursive mapping and flush the tlb to ensure the correct page translation
            self.p4_mut()[511].set(table.p4_frame.clone(), PRESENT | WRITABLE);

            // flush TLB
            self.flush_all();

            // execute f in the new context
            f(self);

            // restore recursive mapping to original p4 table and flush the tlb again
            p4_table[511].set(backup, PRESENT | WRITABLE);

            // flush TLB
            self.flush_all();
        }

        temporary_page.unmap(self);
    }

    /// Switch context
    pub fn switch(&mut self, new_table: InactivePageTable) -> InactivePageTable {
        use x86_64::PhysicalAddress;
        use x86_64::registers::control_regs;

        // store the ond table
        let old_table = InactivePageTable {
            p4_frame: Frame::containing_address(control_regs::cr3().0 as usize),
        };

        // switch to the new page table
        unsafe {
            control_regs::cr3_write(PhysicalAddress(new_table.p4_frame.start_address() as u64));
        }

        // return the old table
        old_table
    }

    /// Flush all the TLB table
    pub fn flush_all(&mut self) {
        use x86_64::instructions::tlb;

        unsafe { tlb::flush_all(); }
    }

    /// Get the CR3 address.
    pub unsafe fn address(&self) -> usize {
        use x86_64::registers::control_regs;
        control_regs::cr3().0 as usize
    }
}

pub struct InactivePageTable {
    p4_frame: Frame,
}

impl InactivePageTable {
    pub fn new(frame: Frame,
               active_table: &mut ActivePageTable,
               temporary_page: &mut TemporaryPage)
               -> InactivePageTable
    {
        {
            let table = temporary_page.map_table_frame(frame.clone(), active_table);

            // zero the page table
            table.zero();

            // set up recursive mapping for the table
            table[511].set(frame.clone(), PRESENT | WRITABLE);
        }
        temporary_page.unmap(active_table);

        InactivePageTable { p4_frame: frame }
    }
}

/// Copy tdata, clear tbss, set TCB self pointer
unsafe fn init_tcb(cpu_id: usize) -> usize {
    extern {
        /// The starting byte of the thread data segment
        static mut __tdata_start: u8;
        /// The ending byte of the thread data segment
        static mut __tdata_end: u8;
        /// The starting byte of the thread BSS segment
        static mut __tbss_start: u8;
        /// The ending byte of the thread BSS segment
        static mut __tbss_end: u8;
    }

    let tcb_offset;
    {
        let size = & __tbss_end as *const _ as usize - & __tdata_start as *const _ as usize;
        let tbss_offset = & __tbss_start as *const _ as usize - & __tdata_start as *const _ as usize;

        let start = KERNEL_PERCPU_OFFSET + KERNEL_PERCPU_SIZE * cpu_id;
        let end = start + size;
        tcb_offset = end - mem::size_of::<usize>();

        // copy data
        ptr::copy(& __tdata_start as *const u8, start as *mut u8, tbss_offset);

        // zero .tbss
        ptr::write_bytes((start + tbss_offset) as *mut u8, 0, size - tbss_offset);

        *(tcb_offset as *mut usize) = end;
    }

    tcb_offset
}

/// Remap the kernel
pub unsafe fn remap_the_kernel<A>(allocator: &mut A, boot_info: &BootInformation) -> ActivePageTable
    where A: FrameAllocator
{
    use core::ops::Range;

    // get the external data segments
    extern {
        /// The starting byte of the text (code) data segment.
        static mut __text_start: u8;
        /// The ending byte of the text (code) data segment.
        static mut __text_end: u8;
        /// The starting byte of the _.rodata_ (read-only data) segment.
        static mut __rodata_start: u8;
        /// The ending byte of the _.rodata_ (read-only data) segment.
        static mut __rodata_end: u8;
        /// The starting byte of the _.data_ segment.
        static mut __data_start: u8;
        /// The ending byte of the _.data_ segment.
        static mut __data_end: u8;
        /// The starting byte of the thread data segment
        static mut __tdata_start: u8;
        /// The ending byte of the thread data segment
        static mut __tdata_end: u8;
        /// The starting byte of the thread BSS segment
        static mut __tbss_start: u8;
        /// The ending byte of the thread BSS segment
        static mut __tbss_end: u8;
        /// The starting byte of the _.bss_ (uninitialized data) segment.
        static mut __bss_start: u8;
        /// The ending byte of the _.bss_ (uninitialized data) segment.
        static mut __bss_end: u8;
    }

    let mut temporary_page = TemporaryPage::new(Page { number: 0xcafebabe }, allocator);

    let mut active_table = unsafe { ActivePageTable::new() };
    let mut new_table = {
        let frame = allocator.allocate_frame().expect("no more frames");
        InactivePageTable::new(frame, &mut active_table, &mut temporary_page)
    };

    // TODO remap the kernel sections individual, set permissions and deal with the tbss section properly

    active_table.with(&mut new_table, &mut temporary_page, |mapper| {
        // Map tdata and tbss
        {
            // get the thread segment size.
            let size = & __tbss_end as *const _ as usize - & __tdata_start as *const _ as usize;

            // TODO add support to multiple CPUs (replace the numeric constant)
            let start = KERNEL_PERCPU_OFFSET + KERNEL_PERCPU_SIZE * 1;
            let end = start + size;

            let start_page = Page::containing_address(start as VirtualAddress);
            let end_page = Page::containing_address((end - 1) as VirtualAddress);
            for page in Page::range_inclusive(start_page, end_page) {
                mapper.map(page, PRESENT | GLOBAL | NO_EXECUTE | WRITABLE, allocator);
            }
        }

        // Function to remap the kernel sections individually. We can no longer use the multiboot
        // sections because of the thread base sections.
        let mut remap = move |start: usize, end: usize, flags: EntryFlags, mapper: &mut Mapper, allocator: &mut A| {
            assert!(start as usize % PAGE_SIZE == 0, "sections need to be page aligned");

            let start_frame = Frame::containing_address(start);
            let end_frame = Frame::containing_address(end - 1);
            for frame in Frame::range_inclusive(start_frame, end_frame) {
                mapper.identity_map(frame, flags, allocator);
            }
        };

        // Remap the kernel with `flags`
        let mut remap_section = move |start: &u8, end: &u8, flags: EntryFlags, mapper: &mut Mapper, allocator: &mut A| {
            remap(start as *const _ as usize, end as *const _ as usize, flags, mapper, allocator);
        };

        // Remap text read-only
        remap_section(& __text_start, & __text_end, PRESENT | GLOBAL, mapper, allocator);
        // Remap rodata read-only, no execute
        remap_section(& __rodata_start, & __rodata_end, PRESENT | GLOBAL | NO_EXECUTE, mapper, allocator);
        // Remap data writable, no execute
        remap_section(& __data_start, & __data_end, PRESENT | GLOBAL | NO_EXECUTE | WRITABLE, mapper, allocator);
        // Remap tdata master writable, no execute
        remap_section(& __tdata_start, & __tdata_end, PRESENT | GLOBAL | NO_EXECUTE, mapper, allocator);
        // Remap bss writable, no execute
        remap_section(& __bss_start, & __bss_end, PRESENT | GLOBAL | NO_EXECUTE | WRITABLE, mapper, allocator);

        // identity map the VGA text buffer
        let vga_buffer_frame = Frame::containing_address(0xb8000);
        mapper.identity_map(vga_buffer_frame, WRITABLE, allocator);

        // identity map the multiboot info structure
        let multiboot_start = Frame::containing_address(boot_info.start_address());
        let multiboot_end = Frame::containing_address(boot_info.end_address() - 1);
        for frame in Frame::range_inclusive(multiboot_start, multiboot_end) {
            mapper.identity_map(frame, PRESENT, allocator);
        }
    });

    // switch context to start using the new page table
    let old_table = active_table.switch(new_table);

    // turn the old p4 page into a guard page
    let old_p4_page = Page::containing_address(old_table.p4_frame.start_address());
    active_table.unmap(old_p4_page, allocator);
    println!("guard page at {:#x}", old_p4_page.start_address());

    // initialize tcb
    // TODO make the CPU id dynamic
    init_tcb(1);

    active_table
}
