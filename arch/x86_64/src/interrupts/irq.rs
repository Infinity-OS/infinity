use x86_64::structures::idt::ExceptionStackFrame;

use time;
use device::local_apic;

pub extern "x86-interrupt" fn timer(stack_frame: &mut ExceptionStackFrame) {
    const UPDATE_RATE: u64 = 0x10000;

    let mut offset = time::OFFSET.lock();
    let sum = offset.1 + UPDATE_RATE;
    offset.1 = sum % 1000000000;
    offset.0 += sum / 1000000000;

    unsafe {
        local_apic::LOCAL_APIC.end_of_interrupt();
    }

}
