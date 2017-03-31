use alloc::arc::Arc;
use collections::BTreeMap;
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
}
