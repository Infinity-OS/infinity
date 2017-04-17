//! Filesystem related syscalls.

use context;
use syscall;
use syscall::data::{Stat, Packet};
use syscall::error::*;
use syscall::flag::*;
use scheme::{self, FileHandle};

pub fn file_open(a: usize, fd: FileHandle, c: usize, d: usize) -> Result<usize> {
    // get the file, the pid of the current context, as well the user id and group id.
    let (file, pid, uid, gid) = {
        // get the current context
        let contents = context::contexts();
        let context_lock = contents.current().ok_or(Error::new(ESRCH))?;
        let context = context_lock.read();

        // get the required file by file descriptor
        let file = context.get_file(fd).ok_or(Error::new(EBADF))?;

        (file, context.id, context.euid, context.egid)
    };

    // get the correspondent scheme
    let scheme = {
        let schemes = scheme::schemes();
        let scheme = schemes.get(file.scheme).ok_or(Error::new(EBADF))?;
        scheme.clone()
    };

    // create a new package to contain the file info
    let mut packet = Packet {
        id: 0,
        pid: pid.into(),
        uid: uid,
        gid: gid,
        a: a,
        b: file.number,
        c: c,
        d: d
    };

    scheme.handle(&mut packet);

    Error::demux(packet.a)
}

pub fn file_open_mut_slice(a: usize, fd: FileHandle, slice: &mut [u8]) -> Result<usize> {
    file_open(a, fd, slice.as_mut_ptr() as usize, slice.len())
}

/// Change the current work directory
///
/// ## Parameters
/// - `path`: location to change to.
///
/// ## Returns
/// A `Result` with a success message or an error.
pub fn chdir(path: &[u8]) -> Result<usize> {
    // open the requests file
    let fd = open(path, syscall::flag::O_RDONLY | syscall::flag::O_DIRECTORY)?;

    // check the file state
    let mut stat = Stat::default();
    let stat_res = file_open_mut_slice(syscall::number::SYS_FSTAT, fd, &mut stat);

    // TODO close the file descriptor

    // handle the response status
    stat_res?;

    if stat.st_mode & (MODE_FILE | MODE_DIR) == MODE_DIR {
        // TODO change the cwd of the context
        Ok(0)
    } else {
        Err(Error::new(ENOTDIR))
    }
}

/// Open system call.
///
/// ## Parameters
/// - `path`: file that must be opened.
/// - `flags`: define how the file must be opened.
pub fn open(path: &[u8], flags: usize) -> Result<FileHandle> {
    // TODO canonicalize the path (make a relative path absolute)
    let (uid, gid, scheme_ns) = {
        // get the correspondent process
        let contexts = context::contexts();
        let context_lock = contexts.current().ok_or(Error::new(ESRCH))?;
        let context = context_lock.read();
        (context.euid, context.egid, context.ens)
    };

    // split the path into two. The first part is the schema and the second part is the reference.
    let mut parts = path.splitn(2, |&b| b == b':');
    let scheme_name_opt = parts.next();
    let reference_opt = parts.next();

    // get the scheme id and the file id
    let (scheme_id, file_id) = {
        let scheme_name = scheme_name_opt.ok_or(Error::new(ENODEV))?;

        // Get the scheme id and the scheme object
        let (scheme_id, scheme) = {
            let schemes = scheme::schemes();
            let (scheme_id, scheme) = schemes.get_name(scheme_ns, scheme_name).ok_or(Error::new(ENODEV))?;
            (scheme_id, scheme.clone())
        };

        // Open a new file
        let file_id = scheme.open(reference_opt.unwrap_or(b""), flags, uid, gid)?;
        (scheme_id, file_id)
    };

    // get the current context
    let contexts = context::contexts();
    let context_lock = contexts.current().ok_or(Error::new(ESRCH))?;
    let context = context_lock.read();

    // add the file to the context
    context.add_file(::context::File {
        scheme: scheme_id,
        number: file_id,
        event: None
    }).ok_or(Error::new(EMFILE))
}
