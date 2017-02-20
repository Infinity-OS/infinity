#![feature(lang_items)]
#![no_std]

extern crate rlibc;
use rlibc::memset;

/// This is the entry point for the Kernel, all the things must be initialized
#[no_mangle]
pub extern "C" fn start() -> ! {
    extern {
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

    // Makes a infinity loop to avoid the kernel returns
    loop { }
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}
