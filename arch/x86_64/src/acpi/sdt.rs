//! System Description Table

use core::mem;

#[derive(Copy, Clone, Debug)]
#[repr(packed)]
pub struct Sdt {
    /// ASCII string representing the table identifier.
    pub signature: [u8; 4],
    /// Length of the table.
    pub length: u32,
    /// Revision of the structure.
    pub revision: u8,
    /// Checksum for the entire table.
    pub checksum: u8,
    /// OEM identifier.
    pub oem_id: [u8; 6],
    /// OEM table identifier.
    pub oem_table_id: [u8; 8],
    /// OEM structure revision.
    pub oem_revision: u32,
    /// Vendor ID (ID for the ASL Compiler).
    pub creator_id: u32,
    /// Revision of utility that created the table (Revision of the ASL Compiler).
    pub creator_revision: u32
}

impl Sdt {
    /// Get the address of this table data.
    pub fn data_address(&'static self) -> usize {
        self as *const _ as usize + mem::size_of::<Self>()
    }

    /// Get the length of this table data.
    pub fn data_len(&'static self) -> usize {
        let total_size = self.length as usize;
        let header_size = mem::size_of::<Self>();

        if total_size >= header_size {
            total_size - header_size
        } else {
            0
        }
    }
}
