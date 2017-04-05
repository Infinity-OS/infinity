use alloc::arc::Arc;
use alloc::boxed::Box;
use collections::BTreeMap;
use core::mem;
use spin::RwLock;

use super::context::{Context, ContextId};

/// Context list type
pub struct ContextList {
    map: BTreeMap<ContextId, Arc<RwLock<Context>>>,
    next_id: usize
}

impl ContextList {
    pub fn new() -> Self {
        ContextList {
            map: BTreeMap::new(),
            next_id: 1
        }
    }

    /// Create a new context.
    ///
    /// ## Returns
    /// A Result with a reference counter for the created Context.
    pub fn new_context(&mut self) -> Result<&Arc<RwLock<Context>>, &str> {
        // TODO check if we are exceeding the max PID number

        // find the next not used ID
        while self.map.contains_key(&ContextId::from(self.next_id)) {
            self.next_id += 1;
        }

        // TODO we can exceed the max number with the operation above

        // save the new id
        let id = ContextId::from(self.next_id);

        // update the next id
        self.next_id += 1;

        // insert the new context into the list
        assert!(self.map.insert(id, Arc::new(RwLock::new(Context::new(id)))).is_none());

        // return the new created context
        Ok(self.map.get(&id).expect("Failed to insert new context. ID is out of bounds."))
    }

    /// Spawn a context from a function.
    pub fn spawn(&mut self, func: extern fn()) -> Result<&Arc<RwLock<Context>>, &str> {
        // lock the context
        let context_lock = self.new_context()?;

        {
            // request a mutable reference
            let mut context = context_lock.write();

            // allocate enough space to store the FX registers
            let mut fx = unsafe { Box::from_raw(::alloc::heap::allocate(512, 16) as *mut [u8; 512]) };

            // zero the allocated memory zone
            for b in fx.iter_mut() {
                *b = 0;
            }

            // allocate a vector of 32 KB
            let mut stack = vec![0; 32_768].into_boxed_slice();

            // Put the function address on the first stack entry
            let offset = stack.len() - mem::size_of::<usize>();
            unsafe {
                let func_ptr = stack.as_mut_ptr().offset(offset as isize);
                *(func_ptr as *mut usize) = func as usize;
            }

            // set the required field of the context structure
            context.arch.set_page_table(unsafe { ::arch::memory::paging::ActivePageTable::new().address() });
            context.arch.set_fx(fx.as_ptr() as usize);
            context.arch.set_stack(stack.as_ptr() as usize + offset);
            context.kstack = Some(stack);
        }

        Ok(context_lock)
    }

    /// Remove a context from the list.
    ///
    /// ## Parameters
    /// - `id`: Id from the context to be removed.
    ///
    /// ## Returns
    /// An Option with a reference counter for the removed Context.
    pub fn remove(&mut self, id: ContextId) -> Option<Arc<RwLock<Context>>> {
        self.map.remove(&id)
    }
}