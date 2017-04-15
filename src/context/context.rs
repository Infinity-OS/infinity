//! This file contains the implementation of the context concept.

use ::core::sync::atomic::AtomicUsize;
use alloc::boxed::Box;
use alloc::arc::Arc;
use collections::Vec;
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
    pub name: Arc<Mutex<Vec<u8>>>
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
            status: Status::Blocked,
            running: false,
            arch: ::arch::context::Context::new(),
            kstack: None,
            heap: None,
            stack: None,
            name: Arc::new(Mutex::new(Vec::new()))
        }
    }
}
