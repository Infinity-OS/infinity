//! # Exception handler system

use memory::MemoryController;
use x86_64::structures::tss::TaskStateSegment;
use x86_64::structures::idt::{Idt, ExceptionStackFrame, PageFaultErrorCode};
use spin::Once;
use self::exceptions::*;

mod gdt;
mod exceptions;

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

static TSS: Once<TaskStateSegment> = Once::new();
static GDT: Once<gdt::Gdt> = Once::new();

/// Initialize the IDT
pub fn init(memory_controller: &mut MemoryController) {
    use x86_64::instructions::segmentation::set_cs;
    use x86_64::instructions::tables::load_tss;
    use x86_64::VirtualAddress;
    use x86_64::structures::gdt::SegmentSelector;

    // allocate a double fault stack
    let double_fault_stack = memory_controller.alloc_stack(1).expect("could not allocate double fault stack");

    // configure the task state segment
    let tss = TSS.call_once(|| {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX] = VirtualAddress(double_fault_stack.top());
        tss
    });

    // configure GDT
    let mut code_selector = SegmentSelector(0);
    let mut tss_selector = SegmentSelector(0);
    let gdt = GDT.call_once(|| {
        let mut gdt = gdt::Gdt::new();
        code_selector = gdt.add_entry(gdt::Descriptor::kernel_code_segment());
        tss_selector = gdt.add_entry(gdt::Descriptor::tss_segment(&tss));
        gdt
    });
    gdt.load();

    unsafe {
        // reload code segment register
        set_cs(code_selector);

        // load TSS
        load_tss(tss_selector);
    }

    // load the IDT table into the CPU
    IDT.load();
}
