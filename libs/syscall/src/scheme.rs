//! Traits for the Scheme definition.

use core::{mem, slice};

use super::*;

pub trait Scheme {
    fn handle(&self, packet: &mut Packet) {
        packet.a = Error::mux(match packet.a {
            SYS_OPEN => self.open(unsafe { slice::from_raw_parts(packet.b as *const u8, packet.c) }, packet.d, packet.uid, packet.gid),

            SYS_READ => self.read(packet.b, unsafe { slice::from_raw_parts_mut(packet.c as *mut u8, packet.d) }),
            SYS_FSTAT => if packet.d >= mem::size_of::<Stat>() { self.fstat(packet.b, unsafe { &mut *(packet.c as *mut Stat) } ) } else { Err(Error::new(EFAULT)) },
           _ => Err(Error::new(ENOSYS))
        });
    }

    fn open(&self, path: &[u8], flags: usize, uid: u32, gid: u32) -> Result<usize> {
        Err(Error::new(ENOENT))
    }

    /// This function returns information about a file.
    #[allow(unused_variables)]
    fn fstat(&self, id: usize, stat: &mut Stat) -> Result<usize> {
        Err(Error::new(EBADF))
    }

    /// This function close a file descriptor.
    #[allow(unused_variables)]
    fn close(&self, id: usize) -> Result<usize> {
        Err(Error::new(EBADF))
    }

    /// This function read from a file descriptor.
    #[allow(unused_variables)]
    fn read(&self, id: usize, buffer: &mut [u8]) -> Result<usize> {
        Err(Error::new(EBADF))
    }
}
