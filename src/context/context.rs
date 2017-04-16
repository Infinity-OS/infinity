//! This file contains the implementation of the context concept.

use ::core::sync::atomic::AtomicUsize;
use alloc::boxed::Box;
use alloc::arc::Arc;
use collections::Vec;
use super::File;
use scheme::{SchemeNamespace, FileHandle};
use spin::Mutex;

use arch::memory::MemoryController;
use super::memory::{Memory, SharedMemory};

/// Unique identifier for a context
int_like!(ContextId, AtomicContextId, usize, AtomicUsize);

/// The status of a context (used for scheduling)
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Status {
    Runnable,
    Blocked,
    Exited(usize)
}

/// A context, witch identifier either a process or a thread.
pub struct Context {
    /// This is used to uniquely identify the context. Its value is always increments when there is
    /// a creation of another object. This can be seen as the PID.
    pub id: ContextId,
    /// This is the ID from the owner process that spawn this object.
    pub parentId: ContextId,
    /// The real user id
    pub ruid: u32,
    /// The real group id
    pub rgid: u32,
    /// The real namespace id
    pub rns: SchemeNamespace,
    /// The effective user id
    pub euid: u32,
    /// The effective group id
    pub egid: u32,
    /// The effective namespace id
    pub ens: SchemeNamespace,
    /// This status is used to store the current structure state.
    pub status: Status,
    /// Is just a fast way to check if the context is currently running.
    pub running: bool,
    /// CPU ID, if locked
    pub cpu_id: Option<usize>,
    /// The architecture specific context.
    pub arch: ::arch::context::Context,
    /// Used to hold the Box to store the FX registers
    pub kfx: Option<Box<[u8]>>,
    /// Stores the kernel stack.
    pub kstack: Option<Box<[u8]>>,
    /// User heap.
    pub heap: Option<SharedMemory>,
    /// User stack.
    pub stack: Option<Memory>,
    /// A string identifier for the current context.
    pub name: Arc<Mutex<Vec<u8>>>,
    /// The open files in the scheme
    pub files: Arc<Mutex<Vec<Option<File>>>>
}

impl Context {
    /// Create a new context instance.
    ///
    /// ## Parameters
    /// - `id`: The unique identifier for this context.
    pub fn new(id: ContextId) -> Context {
        Context {
            id: id,
            parentId: ContextId::from(0),
            ruid: 0,
            rgid: 0,
            rns: SchemeNamespace::from(0),
            euid: 0,
            egid: 0,
            ens: SchemeNamespace::from(0),
            status: Status::Blocked,
            running: false,
            cpu_id: None,
            arch: ::arch::context::Context::new(),
            kfx: None,
            kstack: None,
            heap: None,
            stack: None,
            name: Arc::new(Mutex::new(Vec::new())),
            files: Arc::new(Mutex::new(Vec::new()))
        }
    }

    /// Add a file to the lowest available slot.
    ///
    /// ## Returns
    /// The file descriptor number or None if no slot was found.
    pub fn add_file(&self, file: File) -> Option<FileHandle> {
        // Get the lock for the list of files.
        let mut files = self.files.lock();

        for (i, mut file_option) in files.iter_mut().enumerate() {
            if file_option.is_none() {
                *file_option = Some(file);
                return Some(FileHandle::from(i))
            }
        }

        let len = files.len();
        if len < super::CONTEXT_MAX_FILES {
            files.push(Some(file));
            Some(FileHandle::from(len))
        } else {
            None
        }
    }

    /// Get a file.
    ///
    /// ## Parameters
    /// - `fd`: File descriptor of the file to get.
    ///
    /// ## Returns
    /// The file structure if found. Otherwise a `None`.
    pub fn get_file(&self, fd: FileHandle) -> Option<File> {
        let files = self.files.lock();
        if fd.into() < files.len() {
            files[fd.into()]
        } else {
            None
        }
    }
}
