//! ACPI manager
//!
//! References:
//! - [ACPI 5](http://www.acpi.info/DOWNLOADS/ACPI_5_Errata%20A.pdf)

use memory::{ActivePageTable, MemoryController, Frame};
use memory::paging::Page;
use memory::paging::{VirtualAddress, PhysicalAddress};
use memory::paging::entry;
use self::rsdp::Rsdp;

mod rsdp;

pub fn init(memory_controller: &mut MemoryController) {
    let start_addr = 0xe0000;
    let end_addr = 0xfffff;

    // Map all of the ACPI Root System Description Pointer (RSDP) space.
    {
        let start_frame = Frame::containing_address(start_addr as VirtualAddress);
        let end_frame = Frame::containing_address(end_addr as VirtualAddress);

        for frame in Frame::range_inclusive(start_frame, end_frame) {
            let page = Page::containing_address(frame.start_address());
            let result = memory_controller.map_to(page, frame, entry::PRESENT | entry::NO_EXECUTE);

            // flush TLB
            // TODO Flushing TLB is really expensive, can we do this after mapping all pages?
            memory_controller.flush_all();
        }
    }

    // Now we need to search for the RSDP in order to get the RSDT or XSDT addresses.
    if let Some(rsdp) = Rsdp::search(start_addr, end_addr) {
        println!("ACPI: RSDP found");
    } else {
        println!("ACPI: no RSDP found");
    }
}
