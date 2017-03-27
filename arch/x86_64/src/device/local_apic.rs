use core::intrinsics::{volatile_load, volatile_store};
use raw_cpuid::CpuId;
use x86_64::registers::msr::*;

use memory::{ActivePageTable, MemoryController, Frame};
use memory::paging::Page;
use memory::paging::{VirtualAddress, PhysicalAddress};
use memory::paging::entry;

/// Bind containing an instance of the LocalApic struct
pub static mut LOCAL_APIC: LocalApic = LocalApic {
    base: 0,
    x2_support: false
};

/// Initialize the Local APIC system
pub unsafe fn init(memory_controller: &mut MemoryController) {
    LOCAL_APIC.init(memory_controller);
}

/// End of interrupt register
const APIC_REG_EOI: u32 = 0xb0;
/// Interrupt Control Register (low)
const APIC_REG_ICR_LOW: u32 = 0x300;
/// Interrupt Control Register (higher)
const APIC_REG_ICR_HIGH: u32 = 0x310;

/// Local APIC
pub struct LocalApic {
    base: usize,
    x2_support: bool
}

impl LocalApic {
    /// Initialize the Local APIC system
    pub fn init(&mut self, memory_controller: &mut MemoryController) {
        // get the Local APIC base address
        self.base = rdmsr(IA32_APIC_BASE) as usize & 0xFFFF0000;

        // check if the x2APIC is supported
        self.x2_support = CpuId::new().get_feature_info().unwrap().has_x2apic();

        if ! self.x2_support {
            let page = Page::containing_address(self.base as VirtualAddress);
            let frame = Frame::containing_address(self.base as PhysicalAddress);
            memory_controller.map_to(page, frame, entry::PRESENT | entry::WRITABLE | entry::NO_EXECUTE);

            // flush TLB
            memory_controller.flush_all();
        }

        self.init_ap();

        println!("APIC: Initialized!\n\tBase address: 0x{:>016x}\n\tx2APIC support: {:#?}", self.base, self.x2_support);
    }

    /// Enable LAPIC.
    ///
    /// Whether the X2APIC is supported we enable it too.
    fn init_ap(&mut self) {
        unsafe {
            if self.x2_support {
                wrmsr(IA32_APIC_BASE, rdmsr(IA32_APIC_BASE) | 1 << 10);
                wrmsr(IA32_X2APIC_SIVR, 0x100);
            } else {
                self.write(0xf0, 0x100);
            }
        }
    }

    /// Change the value of a LAPIC register.
    ///
    /// ## Parameters
    /// - `reg`: register offset
    /// - `value`: register value
    fn write(&self, reg: u32, value: u32) {
        unsafe {
            volatile_store((self.base + reg as usize) as *mut u32, value);
        }
    }

    /// Read a LAPIC register.
    ///
    /// ## Parameters
    /// - `reg`: register offset
    ///
    /// ## Returns
    /// The register value.
    fn read(&self, reg: u32) -> u32 {
        unsafe {
            volatile_load((self.base + reg as usize) as *const u32)
        }
    }

    /// Read the Interrupt Command Register (ICR).
    ///
    /// ## Returns
    /// The value of the ICR register.
    pub fn icr(&self) -> u64 {
        if self.x2_support {
            unsafe { rdmsr(IA32_X2APIC_ICR) }
        } else {
            unsafe {
                (self.read(APIC_REG_ICR_HIGH) as u64) << 32 | self.read(APIC_REG_ICR_LOW) as u64
            }
        }
    }

    /// Set the value for the Interrupt Command Register (ICR).
    ///
    /// ## Parameters
    /// - `value`: new value for the register.
    pub fn set_icr(&self, value: u64) {
        if self.x2_support {
            unsafe { wrmsr(IA32_X2APIC_ICR, value); }
        } else {
            unsafe {
                while self.read(APIC_REG_ICR_LOW) & 1 << 12 == 1 << 12 {}
                self.write(APIC_REG_ICR_HIGH, (value >> 32) as u32);
                self.write(APIC_REG_ICR_LOW, value as u32);
                while self.read(APIC_REG_ICR_LOW) & 1 << 12 == 1 << 12 { }
            }
        }
    }

    /// Throw an Inter-Processor Interrupt.
    ///
    /// ## Parameters
    /// - `apic_id`: LAPIC's ID of destination.
    pub fn inter_processor_interrupt(&mut self, apic_id: usize) {
        let mut icr = 0x4040;

        // Set the destination
        if self.x2_support {
            // bits 63:32
            icr |= (apic_id as u64) << 32;
        } else {
            // bits 63:56
            icr |= (apic_id as u64) << 56;
        }

        // set the ICR register
        self.set_icr(icr);
    }

    /// Specific End of Interrupt
    pub fn end_of_interrupt(&mut self) {
        unsafe {
            if self.x2_support {
                wrmsr(IA32_X2APIC_EOI, 0);
            } else {
                self.write(APIC_REG_EOI, 0)
            }
        }
    }
}
