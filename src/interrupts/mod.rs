//! # Exception handler system

use memory::MemoryController;
use x86_64::structures::tss::TaskStateSegment;
use x86_64::structures::idt::{Idt, ExceptionStackFrame, PageFaultErrorCode};

const DOUBLE_FAULT_IST_INDEX: usize = 0;

// The IDT is allocated statically to ensure that this stays in memory until the end of the kernel
// execution.
lazy_static! {
    static ref IDT: Idt = {
        // create a new IDT structure
        let mut idt = Idt::new();

        idt.divide_by_zero.set_handler_fn(divide_by_zero_handler);
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.invalid_opcode.set_handler_fn(invalid_opcode_handler);
        idt.page_fault.set_handler_fn(page_fault_exception);

        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(DOUBLE_FAULT_IST_INDEX as u16);
        }

        idt
    };
}

/// Initialize the IDT
pub fn init(memory_controller: &mut MemoryController) {
    use x86_64::VirtualAddress;

    // allocate a double fault stack
    let double_fault_stack = memory_controller.alloc_stack(1).expect("could not allocate double fault stack");

    // configure the task state segment
    let mut tss = TaskStateSegment::new();
    tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX] = VirtualAddress(double_fault_stack.top());

    // load the IDT table into the CPU
    IDT.load();
}

/// Handler for the division by zero exception
extern "x86-interrupt" fn divide_by_zero_handler(stack_frame: &mut ExceptionStackFrame) {
    // print out the error message and the stack_frame
    println!("\nEXCEPTION: DIVIDE BY ZERO\n{:#?}", stack_frame);
    loop {}
}

/// Handler for a invalid opcode exception
extern "x86-interrupt" fn invalid_opcode_handler(stack_frame: &mut ExceptionStackFrame) {
    println!("\nEXCEPTION: INVALID OPCODE at {:#x}\n{:#?}\n",
        stack_frame.instruction_pointer, stack_frame);
    loop {}
}

/// Handler for page faults
extern "x86-interrupt" fn page_fault_exception(stack_frame: &mut ExceptionStackFrame, error_code: PageFaultErrorCode) {
    use x86_64::registers::control_regs;

    println!("\nEXCEPTION: PAGE FAULT while accessing {:#x}\nerror code: {:?}\n{:#?}",
        control_regs::cr2(),
        error_code,
        stack_frame);
    loop {}
}

/// Handler to catch breakpoint exceptions
extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut ExceptionStackFrame) {
    println!("\nEXCEPTION: BREAKPOINT at {:#x}\n{:#?}",
        stack_frame.instruction_pointer, stack_frame);
}

/// Handler to catch double faults
extern "x86-interrupt" fn double_fault_handler(stack_frame: &mut ExceptionStackFrame, error_code: u64) {
    println!("\nEXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
    loop {}
}
