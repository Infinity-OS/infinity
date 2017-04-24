///! Process syscalls

use alloc::arc::Arc;
use alloc::boxed::Box;
use collections::Vec;
use core::{intrinsics, mem, str};
use spin::Mutex;

use context;
use elf;
use elf::program_header;
use arch::memory::MemoryController;
use arch::memory::paging::{Page, VirtualAddress, entry};
use scheme::{self, FileHandle};
use syscall;
use syscall::data::{Stat, Packet};
use syscall::error::*;

// TODO implement Drop to close the file descriptor
/// Represents a executable file
struct ExecFile(FileHandle);

/// Replaces the current process image with a new process image.
///
/// ## Parameters
/// - `path`: name of a file that is to be executed.
/// - `arg_ptrs`: list of arguments.
///
/// ## Returns
/// Only returns if an error has occurred.
pub fn exec(path: &[u8], arg_ptrs: &[[usize; 2]]) -> Result<usize> {
    let entry;
    let mut sp = ::USER_STACK_OFFSET + ::USER_STACK_SIZE - 256;

    {
        // TODO: handle the arguments

        // get uid, gid and the canonical path to the exec
        let (uid, gid, mut canonical) = {
            let contexts = context::contexts();
            let context_lock = contexts.current().ok_or(Error::new(ESRCH))?;
            let context = context_lock.read();
            (context.euid, context.egid, context.canonicalize(path))
        };

        let mut stat: Stat;
        let mut data: Vec<u8>;

        // open the executable file
        let file = ExecFile(syscall::open(&canonical, syscall::flag::O_RDONLY)?);

        // get the file stats
        stat = Stat::default();
        syscall::file_open_mut_slice(syscall::number::SYS_FSTAT, file.0, &mut stat)?;

        // TODO check permissions when we implement it

        // get the file content
        data = vec![0; stat.st_size as usize];
        syscall::file_open_mut_slice(syscall::number::SYS_READ, file.0, &mut data)?;
        // TODO drop file when implemented

        // TODO add support for a shebang

        // Read ELF sections
        match elf::Elf::from(&data) {
            Ok (elf) => {
                // read ELF sections
                entry = elf.entry();

                // TODO drop path

                // get all contexts
                let contexts = context::contexts();
                let ppid = {
                    // get current context, mutable
                    let context_lock = contexts.current().ok_or(Error::new(ESRCH))?;
                    let mut context = context_lock.write();

                    // Set the new name
                    context.name = Arc::new(Mutex::new(canonical));

                    // TODO clear context

                    // TODO set context uid and egid

                    // Map and copy new segments
                    for segment in elf.segments() {
                        // TODO add support for TLS sections
                        if segment.p_type == program_header::PT_LOAD {
                            let mut memory = context::memory::Memory::new(
                                segment.p_vaddr as VirtualAddress,
                                segment.p_memsz as usize,
                                entry::NO_EXECUTE | entry::WRITABLE,
                                true
                            );

                            unsafe {
                                // Copy file data
                                intrinsics::copy((elf.data.as_ptr() as usize + segment.p_offset as usize) as *const u8,
                                                segment.p_vaddr as *mut u8,
                                                segment.p_filesz as usize);

                                let mut flags = entry::NO_EXECUTE | entry::USER_ACCESSIBLE;

                                if segment.p_flags & program_header::PF_R == program_header::PF_R {
                                    flags.insert(entry::PRESENT);
                                }

                                // W ^ X. If it is executable, do not allow it to be writable, even if requested
                                if segment.p_flags & program_header::PF_X == program_header::PF_X {
                                    flags.remove(entry::NO_EXECUTE);
                                } else if segment.p_flags & program_header::PF_W == program_header::PF_W {
                                    flags.insert(entry::WRITABLE);
                                }

                                memory.remap(flags);

                                context.image.push(memory.to_shared());
                            }
                        }
                    }

                    // Map heap
                    context.heap = Some(context::memory::Memory::new(
                        ::USER_HEAP_OFFSET as VirtualAddress,
                        0,
                        entry::NO_EXECUTE | entry::WRITABLE | entry::USER_ACCESSIBLE,
                        true
                    ).to_shared());

                    // Map stack
                    context.stack = Some(context::memory::Memory::new(
                        ::USER_STACK_OFFSET as VirtualAddress,
                        ::USER_STACK_SIZE,
                        entry::NO_EXECUTE | entry::WRITABLE | entry::USER_ACCESSIBLE,
                        true
                    ));

                    // TODO map TLS

                    // TODO push arguments to stack

                    context.ppid
                };
            },
            Err(err) => {
                println!("failed to execute {}: {}", unsafe { str::from_utf8_unchecked(path) }, err);
                return Err(Error::new(ENOEXEC));
            }
        }
    }

    // TODO go to userland
    // unsafe { arch::usermode(entry, sp); }
    Err(Error::new(ENOEXEC))
}
