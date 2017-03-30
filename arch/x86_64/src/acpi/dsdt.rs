//! Differentiated System Description Table (DSDT)
//!
//! The Differentiated System Description Table (DSDT) is part of the system fixed description. The
//! DSDT is comprised of a system description table header followed by data in Definition Block
//! format. This Definition Block is like all other Definition Blocks, with the exception that it
//! cannot be unloaded.

use core::slice;

use super::sdt::Sdt;

#[derive(Debug)]
pub struct Dsdt(&'static Sdt);

impl Dsdt {
    /// Cast the SDT to a DSDT
    pub fn new(sdt: &'static Sdt) -> Option<Dsdt> {
        if &sdt.signature == b"DSDT" {
            Some(Dsdt(sdt))
        } else {
            None
        }
    }

    pub fn data(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self.0.data_address() as *const u8, self.0.data_len())
        }
    }
}
