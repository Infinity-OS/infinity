//! # Exception handler system

mod idt;

/// This macro creates a wrapper for the handler in order to get the
/// exception stack frame to pass it to the actual exception handler.
macro_rules! handler {
    ($name: ident) => {{
        #[naked]
        extern "C" fn wrapper() -> ! {
            unsafe {
                asm!("mov rdi, rsp
                    sub rsp, 8 // align the stack pointer
                    call $0"
                    :: "i"($name as extern "C" fn(&ExceptionStackFrame) -> !)
                    : "rdi" : "intel");
                ::core::intrinsics::unreachable();
            }
        }
        wrapper
    }}
}

/// This macro creates a wrapper for the handler  to exception that support error codes, in order to
/// get the exception stack frame to pass it to the actual exception handler, and the error code.
macro_rules! handler_with_error_code {
    ($name: ident) => {{
        #[naked]
        extern "C" fn wrapper() -> ! {
            unsafe {
                asm!("pop rsi // pop error code into rsi
                    mov rdi, rsp
                    sub rsp, 8 // align the stack pointer
                    call $0"
                    :: "i"($name as extern "C" fn(&ExceptionStackFrame, u64) -> !)
                    : "rdi" : "intel");
                ::core::intrinsics::unreachable();
            }
        }
        wrapper
    }}
}

// The IDT is allocated statically to ensure that this stays in memory until the end of the kernel
// execution.
lazy_static! {
    #[derive(Debug)]
    static ref IDT: idt::Idt = {
        // create a new IDT structure
        let mut idt = idt::Idt::new();

        // set the handler for the zero division exception
        idt.set_handler(0, handler!(divide_by_zero_handler));
        idt.set_handler(6, handler!(invalid_opcode_handler));
        idt.set_handler(14, handler_with_error_code!(page_fault_exception));

        idt
    };
}

bitflags! {
    flags PageFaultErrorCode: u64 {
        const PROTECTION_VIOLATION = 1 << 0,
        const CAUSED_BY_WRITE = 1 << 1,
        const USER_MODE = 1 << 2,
        const MALFORMED_TABLE = 1 << 3,
        const INSTRUCTION_FETCH = 1 << 4,
    }
}

#[derive(Debug)]
#[repr(C)]
struct ExceptionStackFrame {
    instruction_pointer: u64,
    code_segment: u64,
    cpu_flags: u64,
    stack_pointer: u64,
    stack_segment: u64,
}

/// Initialize the IDT
pub fn init() {
    // load the IDT table into the CPU
    IDT.load();
}

/// Handler for the division by zero exception
extern "C" fn divide_by_zero_handler(stack_frame: &ExceptionStackFrame) -> ! {
    // print out the error message and the stack_frame
    println!("\nEXCEPTION: DIVIDE BY ZERO\n{:#?}", stack_frame);
    loop {}
}

/// Handler for a invalid opcode exception
extern "C" fn invalid_opcode_handler(stack_frame: &ExceptionStackFrame) -> ! {
    println!("\nEXCEPTION: INVALID OPCODE at {:#x}\n{:#?}\n",
        stack_frame.instruction_pointer, stack_frame);
    loop {}
}

extern "C" fn page_fault_exception(stack_frame: &ExceptionStackFrame, error_code: u64) -> ! {
    use x86::shared::control_regs;

    println!("\nEXCEPTION: PAGE FAULT while accessing {:#x}\
        \nerror code: {:?}\n{:#?}",
        unsafe { control_regs::cr2() },
        PageFaultErrorCode::from_bits(error_code).unwrap(),
        stack_frame);
    loop {}
}
