//! Extended System Description Table

use core::mem;

use super::sdt::Sdt;

/// XSDT structure
#[derive(Debug)]
pub struct Xsdt(&'static Sdt);

impl Xsdt {
    /// Cast SDT to XSDT if signature matches.
    pub fn new(sdt: &'static Sdt) -> Option<Self> {
        if &sdt.signature == b"XSDT" {
            Some(Xsdt(sdt))
        } else {
            None
        }
    }

    /// Get a iterator for the table entries.
    pub fn iter(&self) -> XsdtIter {
        XsdtIter {
            sdt: self.0,
            index: 0
        }
    }
}

/// XSDT as an array of 64-bit physical addresses that point to other DESCRIPTION_HEADERs. So we use
/// an iterator to walk through it.
pub struct XsdtIter {
    sdt: &'static Sdt,
    index: usize
}

impl Iterator for XsdtIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.sdt.data_len() / mem::size_of::<u64>() {
            // get the item
            let item = unsafe { *(self.sdt.data_address() as *const u64).offset(self.index as isize) };

            // increment the index
            self.index += 1;

            // return the found entry
            return Some(item as usize);
        }

        // When there is no more elements return a None value
        None
    }
}
