use core::sync::atomic::{AtomicUsize, ATOMIC_USIZE_INIT, Ordering};

use acpi;
use device;
use interrupts;
use memory;
use memory::MemoryController;
use multiboot2;
use vga_buffer;
use kernel_messaging;
use spin::Mutex;

/// This is used to count the number of CPUs on the system
pub static CPU_COUNT: AtomicUsize = ATOMIC_USIZE_INIT;

///  Get the number of CPUs currently active
pub fn cpu_count() -> usize {
    CPU_COUNT.load(Ordering::Relaxed)
}

/// Enable the NXE bit to allow NO_EXECUTE pages.
fn enable_nxe_bit() {
    use x86_64::registers::msr::{IA32_EFER, rdmsr, wrmsr};

    let nxe_bit = 1 << 11;
    unsafe {
        let efer = rdmsr(IA32_EFER);
        wrmsr(IA32_EFER, efer | nxe_bit);
    }
}

/// This enables the write protect bit in order to enable write protection on kernel mode.
fn enable_write_protect_bit() {
    use x86_64::registers::control_regs::{cr0, cr0_write, Cr0};

    unsafe { cr0_write(cr0() | Cr0::WRITE_PROTECT) };
}

extern {
    /// Kernel main function
    fn kmain(memory_controller: &mut MemoryController) -> !;
}

/// Entry point for the Rust code.
#[no_mangle]
pub extern "C" fn rust_main(multiboot_information_address: usize) -> ! {
    // Initialize all the arch components in a different scope than the kernel's main function call.

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
    let (mut memory_controller, tcb_offset) = memory::init(0, boot_info);

    // Initialize IDT and GDT
    interrupts::init(&mut memory_controller, tcb_offset);

    // Initialize devices
    device::init(&mut memory_controller);

    // Read ACPI tables, starts APs
    acpi::init(&mut memory_controller);

    // Initialize all the non-core devices
    device::init_non_core();

    // reset CPU count
    CPU_COUNT.store(1, Ordering::SeqCst);

    // Call the kernel main function
    unsafe { kmain(&mut memory_controller); }
}
