//! Context management

use alloc::boxed::Box;
use spin::{Once, RwLock, RwLockReadGuard, RwLockWriteGuard};
use core::sync::atomic::Ordering;

pub use self::context::{Context, Status, ContextId};
pub use self::file::File;
pub use self::list::ContextList;
pub use self::switch::switch;

/// Context structure
mod context;

/// Context file
mod file;

/// Context list
mod list;

/// Memory logic for the Context
pub mod memory;

/// Scheduler function.
mod switch;

/// Limit on number of contexts
pub const CONTEXT_MAX_CONTEXT: usize = usize::max_value() - 1;

/// Maximum context files
pub const CONTEXT_MAX_FILES: usize = 65536;

/// Current context in this thread
#[thread_local]
static CONTEXT_ID: context::AtomicContextId = context::AtomicContextId::default();

/// Contexts list
static CONTEXTS: Once<RwLock<ContextList>> = Once::new();

/// Initialize contexts, called if needed.
fn init_contexts() -> RwLock<ContextList> {
    RwLock::new(ContextList::new())
}

/// Get the global contexts.
pub fn contexts() -> RwLockReadGuard<'static, ContextList> {
    CONTEXTS.call_once(init_contexts).read()
}

/// Get the global contexts, mutable.
pub fn contexts_mut() -> RwLockWriteGuard<'static, ContextList> {
    CONTEXTS.call_once(init_contexts).write()
}

/// Initialize the context sub-system
pub fn init() {
    // get the contexts as mutable
    let mut contexts = contexts_mut();

    // create a new context
    let context_lock = contexts.new_context().expect("Could not initialize first context");

    // Lock the context, mutable
    let mut context = context_lock.write();

    // alloc space to save the FX registers
    let mut fx = unsafe { Box::from_raw(::alloc::heap::allocate(512, 16) as *mut [u8; 512]) };
    for b in fx.iter_mut() {
        *b = 0;
    }

    // set the other required context properties
    context.arch.set_fx(fx.as_ptr() as usize);
    context.status = Status::Runnable;
    context.running = true;
    context.kfx = Some(fx);
    context.cpu_id = Some(::cpu_id());

    // store the current context id
    CONTEXT_ID.store(context.id, Ordering::SeqCst);
}
