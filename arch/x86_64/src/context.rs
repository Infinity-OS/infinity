//! Architecture context implementation.

/// Architecture context structure
#[derive(Clone, Debug)]
pub struct Context {
    /// FX registers location
    fx_regs: usize,
    /// Page table pointer
    cr3: usize,
    /// RFLAGS register
    rflags: usize,
    /// R12 register
    r12: usize,
    /// R13 register
    r13: usize,
    /// R14 register
    r14: usize,
    /// R15 register
    r15: usize,
    /// Base pointer
    rbp: usize,
    /// Stack pointer
    rsp: usize
}

impl Context {
    /// Create a new Context instance
    pub fn new() -> Context {
        Context {
            fx_regs: 0,
            cr3: 0,
            rflags: 0,
            r12: 0,
            r13: 0,
            r14: 0,
            r15: 0,
            rbp: 0,
            rsp: 0
        }
    }

    pub fn set_fx(&mut self, address: usize) {
        self.fx_regs = address;
    }

    /// Set the page table address.
    pub fn set_page_table(&mut self, address: usize) {
        self.cr3 = address;
    }

    /// Set the stack address.
    pub fn set_stack(&mut self, address: usize) {
        self.rsp = address;
    }
}
