#![feature(lang_items)]
#![no_std]
#![feature(const_fn)]
#![feature(unique)]
#![feature(asm)]
#![feature(naked_functions)]

extern crate rlibc;
extern crate volatile;
extern crate spin;
#[macro_use]
extern crate bitflags;
extern crate bit_field;
extern crate x86;
#[macro_use]
extern crate lazy_static;

use rlibc::memset;
use memory::FrameAllocator;

mod initium;
#[macro_use]
mod vga_buffer;
mod version;
mod memory;
mod interrupts;

/// Magic number passed to the entry point of a Initium kernel.
static INITIUM_MAGIC: u32 = 0xb007cafe;

/// Kernel allocation area base
const KERNEL_KMEM_BASE: u32 = 0xFFFFFF8000000000;
/// Kernel allocation area size (510GB)
const KERNEL_KMEM_SIZE: u32 = 0x0000007F80000000;
/// Last address of kmem space
const KERNEL_KMEM_END: u32 = (KERNEL_KMEM_BASE + KERNEL_KMEM_SIZE - 1);

/// This is the entry point for the Kernel, all the things must be initialized
#[no_mangle]
pub extern "C" fn start(magic: u32, initium_info_addr: usize) -> ! {
    extern "C" {
        /// The starting byte of the .bss segment
        static mut __bss_start: u8;
        /// The ending byte of the .bss segment
        static mut __bss_end: u8;
    }

    // Zero BSS, this initializes statics that are set to 0
    unsafe {
        let start_ptr = &mut __bss_start as *mut u8;
        let end_ptr = &mut __bss_end as *const u8 as usize;

        if start_ptr as usize <= end_ptr {
            let size = end_ptr - start_ptr as usize;
            memset(start_ptr, 0, size);
        }
    }

    // clear the screen
    vga_buffer::clear_screen();

    // Print a Welcome message
    println!("kernel: version {} booting...", version::PULSAR_VER_STRING);

    // check if we are been booted up from a valid bootloader
    if magic != INITIUM_MAGIC {
        panic!("Invalid magic flag!");
    }

    // load boot information
    let boot_info = unsafe { initium::load(initium_info_addr) };

    let core_information = boot_info.core_information().expect("Core tag required");
    let initium_start = initium_info_addr;
    let initium_end = initium_info_addr + (core_information.tags_size() as usize);

    let mut frame_allocator = memory::AreaFrameAllocator::new(KERNEL_KMEM_BASE as usize,
                                                              KERNEL_KMEM_END as usize,
                                                              initium_start,
                                                              initium_end,
                                                              boot_info.memory_map());

    loop {}

    // --- OLD

    // WIP: testing page
    // TODO: use the bootloader page tables tag to get the PML4 address
    // memory::test_paging(&mut frame_allocator);

    // WIP: Initialize IDT system
    interrupts::init();

    fn divide_by_zero() {
        unsafe {
            asm!("mov dx, 0; div dx" ::: "ax", "dx" : "volatile", "intel")
        }
    }

    // provoke a divide-by-zero fault
    divide_by_zero();

    println!("It did not crash!");
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str,
                        line: u32) -> !
{
    println!("\n\nPANIC in {} at line {}:", file, line);
    println!("    {}", fmt);



    loop{}
}





