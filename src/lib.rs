//! The Infinity OS Kernel is a µkernel with focus on performance, multi-core system, security, and
//! no legacy components.

#![feature(alloc, collections)]
#![feature(const_fn)]
#![feature(drop_types_in_const)]
#![feature(heap_api)]
#![feature(thread_local)]
#![no_std]

/// Architecture specific items (x86_64)
#[cfg(all(not(test), target_arch = "x86_64"))]
#[macro_use]
extern crate arch_x86_64 as arch;

extern crate alloc;
#[macro_use]
extern crate collections;
extern crate spin;

use arch::memory::MemoryController;
use arch::interrupts;
use core::sync::atomic::{AtomicUsize, ATOMIC_USIZE_INIT, Ordering};
use spin::Mutex;

#[macro_use]
pub mod common;

pub mod context;

/// Architecture memory controller.
static MEMORY_CONTROLLER: Mutex<Option<&'static mut MemoryController>> = Mutex::new(None);

/// A unique number that identifies the current CPU - used for scheduling.
static CPU_ID: AtomicUsize = ATOMIC_USIZE_INIT;

/// Get the current CPU's scheduling ID.
pub fn cpu_id() -> usize {
    CPU_ID.load(Ordering::Relaxed)
}

pub extern fn userspace_init() {
    println!("Hello from a context!");
    loop {}
}

/// This is the kernel entry point for the primary CPU. The arch crate is responsible for calling
/// this.
#[no_mangle]
pub extern fn kmain(memory_controller: &'static mut MemoryController) -> ! {
    // save the memory controller
    *MEMORY_CONTROLLER.lock() = Some(memory_controller);

    // set the current CPU id
    CPU_ID.store(0, Ordering::SeqCst);

    // initialize the context sub-system
    context::init();

    // Spawn a context
    match context::contexts_mut().spawn(userspace_init) {
        Ok(context_lock) => {
            let mut context = context_lock.write();
            context.status = context::Status::Runnable;
        },
        Err(error) => {
            panic!("failed to spawn userspace_init: {}", error);
        }
    }

    loop {
        unsafe {
            // disable interrupts in order to perform the switch without interruptions.
            interrupts::disable();

            // swicth to the next context.
            if context::switch() {
                interrupts::enable_and_nop();
            } else {
                // enable interrupt, then halt CPU (to save power) until the next interrupt is fired.
                interrupts::enable_and_halt();
            }
        }
    }
}
