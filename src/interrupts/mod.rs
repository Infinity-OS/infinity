//! # Exception handler system

mod idt;

/// This macro saves all scratch registers before calling an exception.
macro_rules! save_scratch_registers {
    () => {
        asm!("push rax
              push rcx
              push rdx
              push rsi
              push rdi
              push r8
              push r9
              push r10
              push r11
        " :::: "intel", "volatile");
    }
}

/// This macro restores all scratch registers before calling an exception.
macro_rules! restore_scratch_registers {
    () => {
        asm!("pop r11
              pop r10
              pop r9
              pop r8
              pop rdi
              pop rsi
              pop rdx
              pop rcx
              pop rax
            " :::: "intel", "volatile");
    }
}

/// This macro creates a wrapper for the handler in order to get the
/// exception stack frame to pass it to the actual exception handler.
macro_rules! handler {
    ($name: ident) => {{
        #[naked]
        extern "C" fn wrapper() -> ! {
            unsafe {
                // save all scratch registers
                save_scratch_registers!();

                // align the stack, and call the handler passing the exception stack frame as a
                // argument.
                asm!("mov rdi, rsp
                    add rdi, 9*8 // calculate exception stack frame pointer
                    call $0"
                    :: "i"($name as extern "C" fn(&ExceptionStackFrame))
                    : "rdi" : "intel");

                // restore scratch registerns
                restore_scratch_registers!();

                // this allow us to return from an exception
                asm!("iretq"
                    :::: "intel", "volatile");
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
                // save all scratch registers
                save_scratch_registers!();

                // align the stack, and call the handler passing the exception stack frame and the
                // error code as a arguments.
                asm!("mov rsi, [rsp + 9*8] // load error code into rsi
                    mov rdi, rsp
                    add rsi, 10*8 // calculate exception stack frame pointer
                    sub rsp, 8 // align the stack pointer
                    call $0
                    add rsp, 8 // under stack pointer alignment"
                    :: "i"($name as extern "C" fn(&ExceptionStackFrame, u64))
                    : "rdi" : "intel");

                // restore scratch registerns
                restore_scratch_registers!();

                // this allow us to return from an exception
                asm!("add rsp, 8 // pop error code
                    iretq"
                    :::: "intel", "volatile");
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
        idt.set_handler(3, handler!(breakpoint_handler));
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
extern "C" fn divide_by_zero_handler(stack_frame: &ExceptionStackFrame) {
    // print out the error message and the stack_frame
    println!("\nEXCEPTION: DIVIDE BY ZERO\n{:#?}", stack_frame);
    loop {}
}

/// Handler for a invalid opcode exception
extern "C" fn invalid_opcode_handler(stack_frame: &ExceptionStackFrame) {
    println!("\nEXCEPTION: INVALID OPCODE at {:#x}\n{:#?}\n",
        stack_frame.instruction_pointer, stack_frame);
    loop {}
}

/// Handler for page faults
extern "C" fn page_fault_exception(stack_frame: &ExceptionStackFrame, error_code: u64) {
    use x86::shared::control_regs;

    println!("\nEXCEPTION: PAGE FAULT while accessing {:#x}\
        \nerror code: {:?}\n{:#?}",
        unsafe { control_regs::cr2() },
        PageFaultErrorCode::from_bits(error_code).unwrap(),
        stack_frame);
    loop {}
}

/// Handler to catch breakpoint exceptions
extern "C" fn breakpoint_handler(stack_frame: &ExceptionStackFrame) {
    println!("\nEXCEPTION: BREAKPOINT at {:#x}\n{:#?}",
        stack_frame.instruction_pointer, stack_frame);
}
