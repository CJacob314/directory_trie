use std::cell::RefCell;
use std::ffi::{OsStr, OsString};
use std::rc::Rc;

use hashbrown::hash_map::HashMap;

pub(crate) type TrieNodeRc<T> = Rc<RefCell<DirectoryTrieNode<T>>>;

#[derive(Debug)]
pub(crate) struct DirectoryTrieNode<T> {
    pub(crate) children: HashMap<OsString, TrieNodeRc<T>>,
    pub(crate) value: Option<T>,
}

impl<T: Clone> DirectoryTrieNode<T> {
    pub(crate) fn new(value: T) -> TrieNodeRc<T> {
        Rc::new(RefCell::new(DirectoryTrieNode {
            children: HashMap::new(),
            value: Some(value),
        }))
    }

    pub(crate) fn new_intermediate() -> TrieNodeRc<T> {
        Rc::new(RefCell::new(DirectoryTrieNode {
            children: HashMap::new(),
            value: None,
        }))
    }

    pub(crate) fn get(&self, path_component: &OsStr) -> Option<TrieNodeRc<T>> {
        self.children.get(path_component).cloned()
    }

    pub(crate) fn insert(&mut self, path_component: OsString, node: TrieNodeRc<T>) {
        self.children.insert(path_component, node);
    }

    pub(crate) fn value(&self) -> Option<T> {
        self.value.clone()
    }
}
