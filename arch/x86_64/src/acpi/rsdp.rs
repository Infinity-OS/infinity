//! Root System Description Pointer

/// RSDP Structure
#[derive(Copy, Clone, Debug)]
#[repr(packed)]
pub struct Rsdp {
    /// "RSD PTR "
    signature: [u8; 8],
    /// Checksum
    checksum: u8,
    /// String that identifies the OEM
    oemid: [u8; 6],
    /// Revision of this structure
    revision: u8,
    /// 32-bit physical address of the RSDT
    rsdt_address: u32,
    /// Length of the table
    length: u32,
    /// 64-bit physical address of the XSDT
    xsdt_address: u64,
    /// Checksum for this entire table
    extended_checksum: u8,
    /// Don't used
    reserved: [u8; 3]
}

impl Rsdp {
    /// Search for the RSDP
    pub fn search(start_address: usize, end_address: usize) -> Option<Rsdp> {
        for i in 0..(end_address + 1 - start_address) / 16 {
            // get the data as a RSDP structure
            let rsdp = unsafe { &*((start_address + i * 16) as *const Rsdp) };

            // Check if the signature matches
            if &rsdp.signature == b"RSD PTR " {
                return Some(*rsdp);
            }
        }

        None
    }
}
