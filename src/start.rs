use vga_buffer;
use memory;
use multiboot2;
use interrupts;
use device;

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
    let mut memory_controller = memory::init(boot_info);

    // Initialize IDT
    interrupts::init(&mut memory_controller);

    // Initialize devices
    device::init(&mut memory_controller);

    println!("It did not crash!");

    loop {}
}
