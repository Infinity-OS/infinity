//! # Exception handler system

use memory::MemoryController;
use x86_64::PrivilegeLevel;
use x86_64::structures::tss::TaskStateSegment;
use x86_64::structures::idt::Idt;
use spin::Once;

mod gdt;
mod ipi;
mod irq;
mod exceptions;

const DOUBLE_FAULT_IST_INDEX: usize = 0;

// The IDT is allocated statically to ensure that this stays in memory until the end of the kernel
// execution.
lazy_static! {
    static ref IDT: Idt = {
        // create a new IDT structure
        let mut idt = Idt::new();

        // Set up exceptions
        // Set up exceptions
        idt.divide_by_zero.set_handler_fn(exceptions::divide_by_zero);
        idt.debug.set_handler_fn(exceptions::debug);
        idt.non_maskable_interrupt.set_handler_fn(exceptions::non_maskable);
        idt.overflow.set_handler_fn(exceptions::overflow);
        idt.breakpoint.set_handler_fn(exceptions::breakpoint);
        idt.bound_range_exceeded.set_handler_fn(exceptions::bound_range_exceeded);
        idt.invalid_opcode.set_handler_fn(exceptions::invalid_opcode);
        idt.device_not_available.set_handler_fn(exceptions::device_not_available);
        unsafe {
            idt.double_fault.set_handler_fn(exceptions::double_fault)
                .set_stack_index(DOUBLE_FAULT_IST_INDEX as u16);
        }
        // 9 the coprocessor_segment_overrun is a discontinued exception
        idt.invalid_tss.set_handler_fn(exceptions::invalid_tss);
        idt.segment_not_present.set_handler_fn(exceptions::segment_not_present);
        idt.stack_segment_fault.set_handler_fn(exceptions::stack_segment_fault);
        idt.general_protection_fault.set_handler_fn(exceptions::general_protection_fault);
        idt.page_fault.set_handler_fn(exceptions::page_fault);
        // 15 reserved
        idt.x87_floating_point.set_handler_fn(exceptions::x87_floating_point);
        idt.alignment_check.set_handler_fn(exceptions::alignment_check);
        idt.machine_check.set_handler_fn(exceptions::machine_check);
        idt.simd_floating_point.set_handler_fn(exceptions::simd_floating_point);
        idt.virtualization.set_handler_fn(exceptions::virtualization);
        // 21 through 29 reserved
        idt.security_exception.set_handler_fn(exceptions::security_exception);
        // 31 reserved

        // set timer interrupt
        idt.interrupts[0x40].set_handler_fn(irq::timer).set_privilege_level(PrivilegeLevel::Ring3);

        // set IPI handler
        // TODO implement this properly. For this this is just a null interrupt.
        idt.interrupts[64].set_handler_fn(ipi::ipi);

        idt
    };
}

static TSS: Once<TaskStateSegment> = Once::new();
static GDT: Once<gdt::Gdt> = Once::new();

// TODO this must be adapted to add support to multi thread systems
/// Initialize the IDT
pub fn init(memory_controller: &mut MemoryController, tcb_offset: usize) {
    use x86_64::instructions::segmentation;
    use x86_64::instructions::tables::load_tss;
    use x86_64::PrivilegeLevel;
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
    let mut data_selector = SegmentSelector(0);
    let mut tls_selector = SegmentSelector(0);
    let mut tss_selector = SegmentSelector(0);
    let gdt = GDT.call_once(|| {
        let mut gdt = gdt::Gdt::new();
        // 1. setup the kernel code segment
        code_selector = gdt.add_entry(gdt::Descriptor::kernel_code_segment());

        // 2. setup the kernel data segment
        data_selector = gdt.add_entry(gdt::Descriptor::kernel_data_segment());

        // 3. setup the thread local segment
        tls_selector = gdt.add_entry(gdt::Descriptor::thread_local_segment(tcb_offset));

        // 4. User code
        gdt.add_entry_user(gdt::Descriptor::user_code_segment());

        // 5. User data
        gdt.add_entry_user(gdt::Descriptor::user_data_segment());

        // 6. User TLS
        gdt.add_entry_user(gdt::Descriptor::user_thread_local_segment(::USER_TCB_OFFSET));

        // 7/8. setup the TSS segment
        tss_selector = gdt.add_entry(gdt::Descriptor::tss_segment(&tss));

        gdt
    });
    gdt.load();

    unsafe {
        // reload segment registers
        segmentation::set_cs(code_selector);
        segmentation::load_ds(data_selector);
        segmentation::load_es(SegmentSelector::new(2, PrivilegeLevel::Ring0));
        segmentation::load_fs(tls_selector);
        segmentation::load_gs(SegmentSelector::new(2, PrivilegeLevel::Ring0));

        // load TSS
        load_tss(tss_selector);
    }

    // load the IDT table into the CPU
    IDT.load();
}

/// Clear interrupts.
#[inline(always)]
pub unsafe fn disable() {
    asm!("cli" : : : : "intel", "volatile");
}

/// Set interrupts.
#[inline(always)]
pub unsafe fn enable() {
    asm!("sti" : : : : "intel", "volatile");
}

/// Set interrupt and halt.
///
/// This will wait for the next interrupt.
#[inline(always)]
pub unsafe fn enable_and_halt() {
    asm!("sti
        hlt" : : : : "intel", "volatile");
}

/// Set interrupts and nop.
///
/// This will enable interrupts and allow the IF flag to be processed.
/// Simply enabling interrupts does not guarantee that they will trigger, use this instead!
#[inline(always)]
pub unsafe fn enable_and_nop() {
    asm!("sti
        nop" : : : : "intel" "volatile");
}

/// Pause instruction
#[inline(always)]
pub fn pause() {
    unsafe { asm!("pause" : : : : "intel", "volatile"); }
}
