//! Root System Description Table

use core::mem;

use super::sdt::Sdt;

/// RSDT structure
#[derive(Debug)]
pub struct Rsdt(&'static Sdt);

impl Rsdt {
    /// Cast SDT to RSDT if signature matches.
    pub fn new(sdt: &'static Sdt) -> Option<Rsdt> {
        if &sdt.signature == b"RSDT" {
            Some(Rsdt(sdt))
        } else {
            None
        }
    }

    /// Get a iterator for the table entries.
    pub fn iter(&self) -> RsdtIter {
        RsdtIter {
            sdt: self.0,
            index: 0
        }
    }
}

/// RSDT as an array of 32-bit physical addresses that point to other DESCRIPTION_HEADERs. So we use
/// an iterator to walk through it.
pub struct RsdtIter {
    sdt: &'static Sdt,
    index: usize
}

impl Iterator for RsdtIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.sdt.data_len() / mem::size_of::<u32>() {
            // get the item
            let item = unsafe { *(self.sdt.data_address() as *const u32).offset(self.index as isize) };

            // increment the index
            self.index += 1;

            // return the found entry
            return Some(item as usize);
        }

        // When there is no more elements return a None value
        None
    }
}
