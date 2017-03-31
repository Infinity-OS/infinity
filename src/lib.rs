//! The Infinity OS Kernel is a µkernel with focus on performance, multi-core system, security, and
//! no legacy components.

#![feature(alloc, collections)]
#![feature(const_fn)]
#![feature(drop_types_in_const)]
#![no_std]

/// Architecture specific items (x86_64)
#[cfg(all(not(test), target_arch = "x86_64"))]
#[macro_use]
extern crate arch_x86_64 as arch;

extern crate alloc;
extern crate collections;
extern crate spin;

#[macro_use]
pub mod common;

pub mod context;

/// This is the kernel entry point for the primary CPU. The arch crate is responsible for calling
/// this.
#[no_mangle]
pub extern fn kmain() -> ! {
    // initialize the context sub-system
    context::init();

    println!("It did not crash!");

    loop { }
}
