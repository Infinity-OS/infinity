//! ACPI manager
//!
//! References:
//! - [ACPI 5](http://www.acpi.info/DOWNLOADS/ACPI_5_Errata%20A.pdf)

use spin::Mutex;

use memory::{ActivePageTable, MemoryController, Frame};
use memory::paging::Page;
use memory::paging::{VirtualAddress, PhysicalAddress};
use memory::paging::entry;
use self::fadt::Fadt;
use self::rsdp::Rsdp;
use self::rsdt::Rsdt;
use self::sdt::Sdt;
use self::xsdt::Xsdt;

mod fadt;
mod rsdp;
mod rsdt;
mod sdt;
mod xsdt;

/// Get the SDT structure
fn get_sdt(sdt_address: usize, memory_controller: &mut MemoryController) -> &'static Sdt {
    let &mut MemoryController {
        ref mut active_table,
        ref mut frame_allocator,
        .. } = memory_controller;

    // If the SDT isn't already mapped into the memory, do it now.
    {
        let page = Page::containing_address(sdt_address as VirtualAddress);
        if active_table.translate_page(page).is_none() {
            let frame = Frame::containing_address(page.start_address() as PhysicalAddress);
            active_table.map_to(page, frame, entry::PRESENT | entry::NO_EXECUTE, frame_allocator);

            // flush TLB
            active_table.flush_all();
        }
    }

    let sdt = unsafe { &*(sdt_address as *const Sdt) };

    // Map extra SDT frames if required
    {
        let start_page = Page::containing_address(sdt_address + 4096 as VirtualAddress);
        let end_page = Page::containing_address(sdt_address + sdt.length as VirtualAddress);

        for page in Page::range_inclusive(start_page, end_page) {
            if active_table.translate_page(page).is_none() {
                let frame = Frame::containing_address(page.start_address() as PhysicalAddress);
                active_table.map_to(page, frame, entry::PRESENT | entry::NO_EXECUTE, frame_allocator);

                // flush TLB
                active_table.flush_all();
            }
        }
    }

    sdt
}

/// Parse a SDT
fn parse_sdt(sdt: &'static Sdt, memory_controller: &mut MemoryController) {
    print!("\t");

    // first, we print out the signature
    for &chr in sdt.signature.iter() {
        print!("{}", chr as char);
    }

    if let Some(fadt) = Fadt::new(sdt) {
        // Print the DSDT address
        println!(": {:x}", fadt.dsdt);

        // parse the DSDT
        let dsdt = get_sdt(fadt.dsdt as usize, memory_controller);
        parse_sdt(dsdt, memory_controller);

        // save the FADT reference
        ACPI_TABLE.lock().fadt = Some(fadt)
    } else {
        println!(": Unknown");
    }
}

/// Initialize ACPI.
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
        // map the (R|X)SDT into virtual memory
        let rxsdt = get_sdt(rsdp.sdt_address(), memory_controller);

        // print out the signature
        for &chr in rxsdt.signature.iter() {
            print!("{}", chr as char);
        }
        println!(":");

        // Check if is a RSDT or a XSDT table
        if let Some(rsdt) = Rsdt::new(rxsdt) {
            // Iterate each entry and parse it
            for sdt_address in rsdt.iter() {
                let sdt = get_sdt(sdt_address, memory_controller);
                parse_sdt(sdt, memory_controller);
            }
        } else if let Some(xsdt) = Xsdt::new(rxsdt) {
            // Iterate each entry and parse it
            for sdt_address in xsdt.iter() {
                let sdt = get_sdt(sdt_address, memory_controller);
                parse_sdt(sdt, memory_controller);
            }
        } else {
            println!("Unknown RSDT or XSDT signature");
        }
    } else {
        println!("ACPI: no RSDP found");
    }

    // TODO Clean the allocated memory after looking for RSDP
}

/// ACPI manager structure
pub struct Acpi {
    pub fadt: Option<Fadt>
}

/// Static ACPI instance
pub static ACPI_TABLE: Mutex<Acpi> = Mutex::new(Acpi { fadt: None });
