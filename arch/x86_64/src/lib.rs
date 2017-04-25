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

extern crate hole_list_allocator;
extern crate alloc;
extern crate collections;

#[macro_use]
extern crate once;

// Make constants public
pub use consts::*;

#[macro_use]
/// Console handling
pub mod vga_buffer;

/// Kernel message writer
pub mod kernel_messaging;

/// ACPI manager
pub mod acpi;

/// Architecture constants
pub mod consts;

/// Architecture context
pub mod context;

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

/// Enter in usermode.
///
/// This functions never returns.
pub unsafe fn usermode(ip: usize, sp: usize) -> ! {
    asm!("
        mov ds, ax
        mov es, ax
        mov fs, bx
        mov gs, ax

        push rax
        push rcx
        push rdx
        push rsi
        push rdi

        iretq"
        :
        :   "{rax}"(5 << 3 | 3)         // Data segment
            "{rbx}"(6 << 3 | 3)         // TLS segment
            "{rcx}"(sp)                 // Stack pointer
            "{rdx}"(3 << 12 | 1 << 9)   // Flags - Set IOPL and interrupt enable flag
            "{rsi}"(4 << 3 | 3)         // Code segment
            "{rdi}"(ip)                 // Instruction Pointer
        :
        : "intel", "volatile"
    );
    unreachable!();
}
