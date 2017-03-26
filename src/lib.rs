#![feature(abi_x86_interrupt)]
#![feature(asm)]
#![feature(const_fn, unique)]
#![feature(lang_items)]
#![feature(naked_functions)]
#![no_std]
#![feature(alloc, collections)]
#![feature(core_intrinsics)]

extern crate bit_field;
#[macro_use]
extern crate bitflags;
extern crate raw_cpuid;
#[macro_use]
extern crate lazy_static;
extern crate multiboot2;
extern crate rlibc;
extern crate spin;
extern crate volatile;
extern crate x86_64;

extern crate bump_allocator;
extern crate alloc;
#[macro_use]
extern crate collections;

#[macro_use]
extern crate once;

#[macro_use]
/// Console handling
pub mod vga_buffer;

/// Devices management
pub mod device;

/// Memory management
pub mod memory;

/// Interrupt instructions
pub mod interrupts;

/// Initialization and start function
pub mod start;

/// Timer functions
pub mod time;

#[cfg(not(test))]
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[cfg(not(test))]
#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str, line: u32) -> ! {
    println!("\n\nPANIC in {} at line {}:", file, line);
    println!("    {}", fmt);
    loop {}
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    loop {}
}
