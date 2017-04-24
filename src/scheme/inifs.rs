// use core::sync::atomic::{AtomicUsize, Ordering};

use collections::BTreeMap;
use core::{cmp, str};
use core::sync::atomic::{AtomicUsize, Ordering};
use spin::RwLock;

use syscall::data::Stat;
use syscall::error::*;
use syscall::scheme::Scheme;
use syscall::flag::{MODE_DIR, MODE_FILE};

// Include the auto-generated file with list of files that are part of Initfs.
include!(concat!(env!("OUT_DIR"), "/gen.rs"));

struct Handle {
    path: &'static [u8],
    flags: usize,
    data: &'static [u8],
    mode: u16,
    seek: usize
}

pub struct InitFsScheme {
    next_id: AtomicUsize,
    files: BTreeMap<&'static [u8], (&'static [u8], bool)>,
    handles: RwLock<BTreeMap<usize, Handle>>
}

impl InitFsScheme {
    /// Create a new instance of `InitFsScheme`
    pub fn new() -> Self {
        InitFsScheme {
            next_id: AtomicUsize::new(0),
            files: gen::gen(),
            handles: RwLock::new(BTreeMap::new())
        }
    }
}

impl Scheme for InitFsScheme {
    fn open(&self, path: &[u8], flags: usize, _uid: u32, _gid: u32) -> Result<usize> {
        // get a str from the path argument
        let path_utf8 = str::from_utf8(path).or(Err(Error::new(ENOENT)))?;

        // trim path from '/'
        let path_trimmed = path_utf8.trim_matches('/');

        for entry in self.files.iter() {
            if entry.0 == &path_trimmed.as_bytes() {
                let id = self.next_id.fetch_add(1, Ordering::SeqCst);
                self.handles.write().insert(id, Handle {
                    path: entry.0,
                    flags: flags,
                    data: (entry.1).0,
                    mode: if (entry.1).1 { MODE_DIR | 0o755 } else { MODE_FILE | 0o744 },
                    seek: 0
                });

                return Ok(id);
            }
        }

        // the requested path doesn't exists
        Err(Error::new(ENOENT))
    }

    fn fstat(&self, id: usize, stat: &mut Stat) -> Result<usize> {
        // get the handler if exists
        let handles = self.handles.read();
        let handle = handles.get(&id).ok_or(Error::new(EBADF))?;

        // fill the stat var
        stat.st_mode = handle.mode;
        stat.st_uid = 0;
        stat.st_gid = 0;
        stat.st_size = handle.data.len() as u64;

        Ok(0)
    }

    fn read(&self, id: usize, buffer: &mut [u8]) -> Result<usize> {
        // get the file descriptor
        let mut handlers = self.handles.write();
        let mut handle = handlers.get_mut(&id).ok_or(Error::new(EBADF))?;

        // Read all the content from the file
        let mut i = 0;
        while i < buffer.len() && handle.seek < handle.data.len() {
            buffer[i] = handle.data[handle.seek];
            i += 1;
            handle.seek += 1;
        }

        // return the size of the date read
        Ok(i)
    }

    fn close(&self, id: usize) -> Result<usize> {
        self.handles.write().remove(&id).ok_or(Error::new(EBADF)).and(Ok(0))
    }
}
