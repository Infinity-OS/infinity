mod idt;

// The IDT is allocated statically to ensure that this stays in memory until the end of the kernel
// execution.
lazy_static! {
    #[derive(Debug)]
    static ref IDT: idt::Idt = {
        // create a new IDT structure
        let mut idt = idt::Idt::new();

        // set the handler for the zero division exception
        idt.set_handler(0, divide_by_zero_handler);

        idt
    };
}

/// Initialize the IDT
pub fn init() {
    // load the IDT table into the CPU
    IDT.load();
}

/// Handler for the division by zero exception
extern "C" fn divide_by_zero_handler() -> ! {
    println!("EXCEPTION: DIVIDE BY ZERO");
    loop {}
}
