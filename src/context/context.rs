//! This file contains the implementation of the context concept.

use ::core::sync::atomic::AtomicUsize;
use alloc::boxed::Box;

/// Unique identifier for a context
int_like!(ContextId, AtomicContextId, usize, AtomicUsize);

/// The status of a context (used for scheduling)
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Status {
    Runnable,
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
    /// The architecture specific context.
    pub arch: ::arch::context::Context,
    /// Stores the kernel stack.
    pub kstack: Option<Box<[u8]>>,
    // TODO User heap.
    // pub heap:
    // TODO User stack.
    // pub stack:
    // TODO A string identifier for the current context.
    // pub name: Arc
}

impl Context {}
