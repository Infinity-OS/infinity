//! Traits for the Scheme definition.

use super::*;

pub trait Scheme {
    fn handle(&self, packet: &mut Packet) {
        packet.a = Error::mux(match packet.a {
           _ => Err(Error::new(ENOSYS))
        });
    }

    fn open(&self, path: &[u8], flags: usize, uid: u32, gid: u32) -> Result<usize> {
        Err(Error::new(ENOENT))
    }
}
