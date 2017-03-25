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
    fn store(&self, reg: u32) -> u32 {
        unsafe {
            volatile_load((self.base + reg as usize) as *const u32);
        }
    }
}
