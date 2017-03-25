use x86_64::structures::idt::ExceptionStackFrame;

use device::local_apic::LOCAL_APIC;

/// Handler for a Inter-Process Interrupt (IPI)
pub extern "x86-interrupt" fn ipi(stack_frame: &mut ExceptionStackFrame) {
    unsafe { LOCAL_APIC.end_of_interrupt(); }
}
