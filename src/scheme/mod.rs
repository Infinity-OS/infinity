//! # Schemes
//!
//! This was inspired by the Redox OS.

use syscall::scheme::Scheme;

use alloc::arc::Arc;
use alloc::boxed::Box;
use collections::BTreeMap;
use ::core::sync::atomic::AtomicUsize;
use spin::{Once, RwLock, RwLockReadGuard, RwLockWriteGuard};

/// Unique identifier for a file descriptor.
int_like!(FileHandle, AtomicFileHandle, usize, AtomicUsize);

/// Unique identifier for a scheme.
int_like!(SchemeId, AtomicSchemeId, usize, AtomicUsize);

/// Unique identifier for a scheme namespace.
int_like!(SchemeNamespace, AtomicSchemeNamespace, usize, AtomicUsize);

pub struct SchemeList {
    map: BTreeMap<SchemeId, Arc<Box<Scheme + Send + Sync>>>,
    names: BTreeMap<SchemeNamespace, BTreeMap<Box<[u8]>, SchemeId>>,
    next_ns: usize,
    next_id: usize
}

impl SchemeList {
    /// Create a new scheme list.
    pub fn new() -> Self {
        let mut list = SchemeList {
            map: BTreeMap::new(),
            names: BTreeMap::new(),
            next_ns: 0,
            next_id: 1
        };

        // TODO initialize some root namespace

        list
    }

    /// Get an iterator.
    pub fn iter(&self) -> ::collections::btree_map::Iter<SchemeId, Arc<Box<Scheme + Send + Sync>>> {
        self.map.iter()
    }

    /// Get an iterator for a given namespace.
    pub fn iter_name(&self, ns: SchemeNamespace) -> ::collections::btree_map::Iter<Box<[u8]>, SchemeId> {
        self.names[&ns].iter()
    }

    /// get the nth scheme.
    pub fn get(&self, id: SchemeId) -> Option<&Arc<Box<Scheme + Send + Sync>>> {
        self.map.get(&id)
    }

    /// Get a scheme by name.
    pub fn get_name(&self, ns: SchemeNamespace, name: &[u8]) -> Option<(SchemeId, &Arc<Box<Scheme + Send + Sync>>)> {
        if let Some(&id) = self.names[&ns].get(name) {
            self.get(id).map(|scheme| (id, scheme))
        } else {
            None
        }
    }
}

/// Schemes list
static SCHEMES: Once<RwLock<SchemeList>> = Once::new();

/// Initialize schemes. This is called when needed
fn init_schemes() -> RwLock<SchemeList> {
    RwLock::new(SchemeList::new())
}

/// Get the global schemes list, const
pub fn schemes() -> RwLockReadGuard<'static, SchemeList> {
    SCHEMES.call_once(init_schemes).read()
}
