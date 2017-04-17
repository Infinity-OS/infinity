//! # Schemes
//!
//! This was inspired by the Redox OS.

use alloc::arc::Arc;
use alloc::boxed::Box;
use collections::BTreeMap;
use ::core::sync::atomic::AtomicUsize;
use spin::{Once, RwLock, RwLockReadGuard, RwLockWriteGuard};

use syscall::error::*;
use syscall::scheme::Scheme;

use self::inifs::InitFsScheme;

/// `initfs`: a readonly filesystem used for initializing the system
pub mod inifs;

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

        // initialize schemes
        list.new_root();

        list
    }

    /// Initialize schemes.
    fn new_root(&mut self) {
        // Create the root scheme namespace
        let ns = SchemeNamespace(self.next_ns);
        self.next_ns += 1;
        self.names.insert(ns, BTreeMap::new());

        // TODO initialize all available schemes

        // The following namespace must be only available on te root namespace.
        self.insert(ns, Box::new(*b"initfs"), |scheme_id| Arc::new(Box::new(InitFsScheme::new()))).unwrap();
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

    /// Create a new scheme.
    pub fn insert<F>(&mut self, ns: SchemeNamespace, name: Box<[u8]>, scheme_fn: F) -> Result<SchemeId>
        where F: Fn(SchemeId) -> Arc<Box<Scheme + Send + Sync>>
    {
        // if the scheme already exists, return an error.
        if self.names[&ns].contains_key(&name) {
            return Err(Error::new(EEXIST));
        }

        // TODO if we get to the max of schemes set the next_id to 1
        // TODO or make the list of schemes grow if necessary

        // TODO this is necessary?
        while self.map.contains_key(&SchemeId(self.next_id)) {
            self.next_id += 1;
        }

        // create a new scheme id
        let id = SchemeId(self.next_id);
        self.next_id += 1;

        // create a new scheme
        let scheme = scheme_fn(id);

        // store it on the List of available schemes
        assert!(self.map.insert(id, scheme).is_none());

        // try associate the scheme to the namespace
        if let Some(ref mut names) = self.names.get_mut(&ns) {
            assert!(names.insert(name, id).is_none());
        } else {
            panic!("scheme namespace not found");
        }

        Ok(id)
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
