#![feature(asm)]
#![feature(const_fn, unique)]
#![feature(lang_items)]
#![no_std]
#![feature(alloc, collections)]


extern crate bit_field;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate lazy_static;
extern crate multiboot2;
extern crate rlibc;
extern crate spin;
extern crate volatile;
extern crate x86;

extern crate bump_allocator;
extern crate alloc;
#[macro_use]
extern crate collections;

#[macro_use]
extern crate once;

#[macro_use]
mod vga_buffer;
mod memory;
mod interrupts;

/// Enable the NXE bit to allow NO_EXECUTE pages.
fn enable_nxe_bit() {
    use x86::shared::msr::{IA32_EFER, rdmsr, wrmsr};

    let nxe_bit = 1 << 11;
    unsafe {
        let efer = rdmsr(IA32_EFER);
        wrmsr(IA32_EFER, efer | nxe_bit);
    }
}

/// This enables the write protect bit in order to enable write protection on kernel mode.
fn enable_write_protect_bit() {
    use x86::shared::control_regs::{cr0, cr0_write, CR0_WRITE_PROTECT};

    unsafe { cr0_write(cr0() | CR0_WRITE_PROTECT) };
}

#[no_mangle]
/// Entry point for the Rust code
pub extern "C" fn rust_main(multiboot_information_address: usize) {

    // clear the console screen
    vga_buffer::clear_screen();

    // print out a welcome message
    println!("kernel: botting");

    let boot_info = unsafe { multiboot2::load(multiboot_information_address) };

    // enable NXE bit, to allow define none executable pages.
    enable_nxe_bit();

    // set write protect bit in order to enable write protection on kernel mode.
    enable_write_protect_bit();

    // set up guard page and map the heap pages
    let mut memory_controller = memory::init(boot_info, multiboot_information_address);

    // WIP: Initialize IDT system
    interrupts::init();

    fn divide_by_zero() {
        unsafe {
            asm!("mov dx, 0; div dx" ::: "ax", "dx" : "volatile", "intel")
        }
    }

    // provoke a divide-by-zero fault
    divide_by_zero();


    use alloc::boxed::Box;
    let heap_test = Box::new(42);

    println!("It did not crash!");

    loop {}
}

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
