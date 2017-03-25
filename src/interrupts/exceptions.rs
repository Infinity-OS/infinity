use x86_64::structures::idt::{ExceptionStackFrame, PageFaultErrorCode};

/// Handler for the division by zero exception
pub extern "x86-interrupt" fn divide_by_zero(stack_frame: &mut ExceptionStackFrame) {
    // print out the error message and the stack_frame
    println!("\nDivide by zero fault at {:>02x}:{:>016x}", stack_frame.code_segment, stack_frame.instruction_pointer);
    loop {}
}

/// Handler for a debug exception
pub extern "x86-interrupt" fn debug(stack_frame: &mut ExceptionStackFrame) {
    println!("\nDebug trap at {:>02x}:{:>016x}", stack_frame.code_segment, stack_frame.instruction_pointer);
    loop {}
}

/// Handler for a non-maskable interrupt exception
pub extern "x86-interrupt" fn non_maskable(stack_frame: &mut ExceptionStackFrame) {
    println!("\nNon-maskable interrupt at {:>02x}:{:>016x}", stack_frame.code_segment, stack_frame.instruction_pointer);
    loop {}
}

/// Handler to catch breakpoint exceptions
pub extern "x86-interrupt" fn breakpoint(stack_frame: &mut ExceptionStackFrame) {
    println!("\nBreakpoint trap at {:>02x}:{:>016x}", stack_frame.code_segment, stack_frame.instruction_pointer);
}

/// Handler for a overflow exception
pub extern "x86-interrupt" fn overflow(stack_frame: &mut ExceptionStackFrame) {
    println!("\nOverflow trap at {:>02x}:{:>016x}", stack_frame.code_segment, stack_frame.instruction_pointer);
    loop {}
}

/// Handler for a bound range exceeded exception
pub extern "x86-interrupt" fn bound_range_exceeded(stack_frame: &mut ExceptionStackFrame) {
    println!("\nBound range exceeded fault at {:>02x}:{:>016x}", stack_frame.code_segment, stack_frame.instruction_pointer);
    loop {}
}

/// Handler for a invalid opcode exception
pub extern "x86-interrupt" fn invalid_opcode(stack_frame: &mut ExceptionStackFrame) {
    println!("\nInvalid opcode fault at {:>02x}:{:>016x}", stack_frame.code_segment, stack_frame.instruction_pointer);
    loop {}
}

/// Handler for a device not found exception
pub extern "x86-interrupt" fn device_not_available(stack_frame: &mut ExceptionStackFrame) {
    println!("\nDevice not available fault at {:>02x}:{:>016x}", stack_frame.code_segment, stack_frame.instruction_pointer);
    loop {}
}

/// Handler to catch double faults
pub extern "x86-interrupt" fn double_fault(stack_frame: &mut ExceptionStackFrame, error_code: u64) {
    println!("\nDouble fault: {:x} at {:>02x}:{:>016x}", error_code, stack_frame.code_segment,
             stack_frame.instruction_pointer);
    loop {}
}

/// Handler for a invalid tss exception
pub extern "x86-interrupt" fn invalid_tss(stack_frame: &mut ExceptionStackFrame, error_code: u64) {
    println!("\nInvalid TSS fault: {:x} at {:>02x}:{:>016x}", error_code, stack_frame.code_segment,
             stack_frame.instruction_pointer);
    loop {}
}

/// Handler for a segment not present exception
pub extern "x86-interrupt" fn segment_not_present(stack_frame: &mut ExceptionStackFrame, error_code: u64) {
    println!("\nSegment not present fault: {:x} at {:>02x}:{:>016x}", error_code, stack_frame.code_segment,
             stack_frame.instruction_pointer);
    loop {}
}

/// Handler for a stack segment fault
pub extern "x86-interrupt" fn stack_segment_fault(stack_frame: &mut ExceptionStackFrame, error_code: u64) {
    println!("\nStack segment fault: {:x} at {:>02x}:{:>016x}", error_code, stack_frame.code_segment,
             stack_frame.instruction_pointer);
    loop {}
}

/// Handler for a general protection fault
pub extern "x86-interrupt" fn general_protection_fault(stack_frame: &mut ExceptionStackFrame, error_code: u64) {
    println!("\nProtection fault: {:x} at {:>02x}:{:>016x}", error_code, stack_frame.code_segment,
             stack_frame.instruction_pointer);
    loop {}
}

/// Handler for page faults
pub extern "x86-interrupt" fn page_fault(stack_frame: &mut ExceptionStackFrame, error_code: PageFaultErrorCode) {
    use x86_64::registers::control_regs;

    println!("\nPage fault while accessing {:>015x}\nerror code: {:?} at {:>02x}:{:>016x}",
             control_regs::cr2(),
             error_code,
             stack_frame.code_segment,
             stack_frame.instruction_pointer);
    loop {}
}

/// Handler for a FPU floating point fault
pub extern "x86-interrupt" fn x87_floating_point(stack_frame: &mut ExceptionStackFrame) {
    println!("\nFPU floating point fault at {:>02x}:{:>016x}", stack_frame.code_segment,
             stack_frame.instruction_pointer);
    loop {}
}

/// Handler for alignment check exception
pub extern "x86-interrupt" fn alignment_check(stack_frame: &mut ExceptionStackFrame, error_code: u64) {
    println!("\nAlignment check fault: {:x} at {:>02x}:{:>016x}", error_code, stack_frame.code_segment,
             stack_frame.instruction_pointer);
    loop {}
}

/// Handler for a machine check exception
pub extern "x86-interrupt" fn machine_check(stack_frame: &mut ExceptionStackFrame) {
    println!("\nMachine check fault at {:>02x}:{:>016x}", stack_frame.code_segment,
             stack_frame.instruction_pointer);
    loop {}
}

/// Handler for a SIMD floating point exception
pub extern "x86-interrupt" fn simd_floating_point(stack_frame: &mut ExceptionStackFrame) {
    println!("\nSIMD floating point fault at {:>02x}:{:>016x}", stack_frame.code_segment,
             stack_frame.instruction_pointer);
    loop {}
}

/// Handler for a virtualization exception
pub extern "x86-interrupt" fn virtualization(stack_frame: &mut ExceptionStackFrame) {
    println!("\nVirtualization fault at {:>02x}:{:>016x}", stack_frame.code_segment,
             stack_frame.instruction_pointer);
    loop {}
}

/// Handler for a security exception
pub extern "x86-interrupt" fn security_exception(stack_frame: &mut ExceptionStackFrame, error_code: u64) {
    println!("\nSecurity exception: {:x} at {:>02x}:{:>016x}", error_code, stack_frame.code_segment,
             stack_frame.instruction_pointer);
    loop {}
}
