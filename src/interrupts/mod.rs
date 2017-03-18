mod idt;

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

// The IDT is allocated statically to ensure that this stays in memory until the end of the kernel
// execution.
lazy_static! {
    #[derive(Debug)]
    static ref IDT: idt::Idt = {
        // create a new IDT structure
        let mut idt = idt::Idt::new();

        // set the handler for the zero division exception
        idt.set_handler(0, handler!(divide_by_zero_handler));

        idt
    };
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
