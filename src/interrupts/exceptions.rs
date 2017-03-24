use x86_64::structures::idt::{Idt, ExceptionStackFrame, PageFaultErrorCode};

/// Handler for the division by zero exception
pub extern "x86-interrupt" fn divide_by_zero_handler(stack_frame: &mut ExceptionStackFrame) {
    // print out the error message and the stack_frame
    println!("\nEXCEPTION: DIVIDE BY ZERO\n{:#?}", stack_frame);
    loop {}
}

/// Handler for a invalid opcode exception
pub extern "x86-interrupt" fn invalid_opcode_handler(stack_frame: &mut ExceptionStackFrame) {
    println!("\nEXCEPTION: INVALID OPCODE at {:#x}\n{:#?}\n",
             stack_frame.instruction_pointer, stack_frame);
    loop {}
}

/// Handler for page faults
pub extern "x86-interrupt" fn page_fault_exception(stack_frame: &mut ExceptionStackFrame, error_code: PageFaultErrorCode) {
    use x86_64::registers::control_regs;

    println!("\nEXCEPTION: PAGE FAULT while accessing {:#x}\nerror code: {:?}\n{:#?}",
             control_regs::cr2(),
             error_code,
             stack_frame);
    loop {}
}

/// Handler to catch breakpoint exceptions
pub extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut ExceptionStackFrame) {
    println!("\nEXCEPTION: BREAKPOINT at {:#x}\n{:#?}",
             stack_frame.instruction_pointer, stack_frame);
}

/// Handler to catch double faults
pub extern "x86-interrupt" fn double_fault_handler(stack_frame: &mut ExceptionStackFrame, error_code: u64) {
    println!("\nEXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
    loop {}
}
