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

#[macro_use]
mod vga_buffer;

/// This is the entry point for the Kernel, all the things must be initialized
#[no_mangle]
pub extern "C" fn start() -> ! {
    vga_buffer::clear_screen();
    println!("Hello from Infinity OS{}", "!");


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
    println!("Infinity OS!");

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
