//! Context management

use alloc::boxed::Box;
use spin::{Once, RwLock, RwLockReadGuard, RwLockWriteGuard};

pub use self::context::{Context, Status, ContextId};
pub use self::list::ContextList;

/// Context structure
mod context;

/// Context list
mod list;

/// Memory logic for the Context
mod memory;


/// Limit on number of contexts
pub const CONTEXT_MAX_CONTEXT: usize = usize::max_value() - 1;
/// Contexts list
static CONTEXTS: Once<RwLock<ContextList>> = Once::new();

/// Initialize contexts, called if needed.
fn init_contexts() -> RwLock<ContextList> {
    RwLock::new(ContextList::new())
}

/// Get the global contexts.
fn contexts() -> RwLockReadGuard<'static, ContextList> {
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

    // create the context, mutable
    let mut context = context_lock.write();

    // TODO Alloc some heap space for the initial context.

    context.status = Status::Runnable;
}
