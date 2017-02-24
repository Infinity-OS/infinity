#![feature(lang_items)]
#![no_std]
#![feature(const_fn)]
#![feature(unique)]
#![feature(asm)]
#![feature(naked_functions)]

extern crate rlibc;
extern crate volatile;
extern crate spin;

use rlibc::memset;

mod initium;
#[macro_use]
mod vga_buffer;
mod version;

/// Magic number passed to the entry point of a Initium kernel.
static INITIUM_MAGIC: u32 = 0xb007cafe;

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
    println!("Infinity OS {}", version::PULSAR_VER_STRING);

    // print out the Initium Magic number
    println!("Bootloder magic flag: 0x{0:x}", magic);

    // check if we are been booted up from a valid bootloader
    if magic != INITIUM_MAGIC {
        panic!("Invalid magic flag!");
    }

    // load boot information
    let boot_info = unsafe { initium::load(initium_info_addr) };
    let memory_map_tag = boot_info.memory_map();

    println!("memory areas:");
    for entry in memory_map_tag {
        println!("    start: 0x{:x}, length: 0x{:x}",
        entry.base_address(), entry.length());
    }

    // Makes a infinity loop to avoid the kernel returns
    loop {}
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
pub extern "C" fn panic_fmt() -> ! {
    loop {}
}
