use arch;
use core::sync::atomic::Ordering;

use context::{contexts, Context, Status, CONTEXT_ID};

/// Switch to the next context.
pub unsafe fn switch() -> bool {
    use core::ops::DerefMut;

    // Set the global lock to avoid the unsafe operations below from causing issues
    while arch::context::CONTEXT_SWITCH_LOCK.compare_and_swap(false, true, Ordering::SeqCst) {
        arch::interrupts::pause();
    }

    let from_prt;
    let mut to_ptr = 0 as *mut Context;
    {
        // get the list of context
        let contexts = contexts();

        // get the current context
        {
            let context_lock = contexts.current().expect("context::switch: not inside of context");
            let mut context = context_lock.write();
            from_prt = context.deref_mut() as *mut Context;
        }

        // TODO we must create a mechanism to prevent switch processors from other CPU's

        // find the next context to be executed
        for (pid, context_lock) in contexts.iter() {
            if *pid > (*from_prt).id {
                let mut context = context_lock.write();
                to_ptr = context.deref_mut() as *mut Context;
            }
        }
    }

    // whether there is no contexts to switch to, we remove the lock and return flase
    if to_ptr as usize == 0 {
        arch::context::CONTEXT_SWITCH_LOCK.store(false, Ordering::SeqCst);
        return false;
    }

    // mark the prev context as stopped
    (&mut *from_prt).running = false;

    // mark the next context as running
    (&mut *to_ptr).running = true;

    // store the current context ID
    CONTEXT_ID.store((&mut *to_ptr).id, Ordering::SeqCst);

    // HACK: this is a temporary workaround, as arch is only used the the current CPU
    arch::context::CONTEXT_SWITCH_LOCK.store(false, Ordering::SeqCst);

    // Switch to this new context
    (&mut *from_prt).arch.switch_to(&mut (&mut *to_ptr).arch);

    true
}
