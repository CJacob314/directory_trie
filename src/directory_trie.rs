use crate::{DirectoryTrieNode, TrieNodeRc};
use std::ffi::OsString;
use std::path::Path;

#[derive(Debug)]
pub struct DirectoryTrie<T> {
    head: Option<TrieNodeRc<T>>,
}

impl<T: Clone> DirectoryTrie<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    #[allow(clippy::assigning_clones)]
    pub fn insert<P: AsRef<Path>>(&mut self, path: P, value: T) {
        let mut cursor_opt = self.head.clone(); // Get head Option<TrieNodeRc>
        let mut next;

        let num_components = path.as_ref().iter().count();
        let skip_num = self.head.as_ref().map(|_| 1).unwrap_or(0);
        if self.head.is_none() {
            // If the head is currently none
            self.head = Some(DirectoryTrieNode::new_intermediate());
            if num_components == 1 {
                // We can't just use the intermediate node, we must also give a value and return
                self.head = Some(DirectoryTrieNode::new(value));
                return;
            }
            cursor_opt.clone_from(&self.head);
        }
        let mut last_cursor_opt = cursor_opt.clone();

        // Now, `self.head`, `last_cursor_opt`, and `cursor_opt` must ALL be referencing THE SAME intermediate trie node
        for (i, component) in path.as_ref().iter().skip(1).enumerate() {
            let component: OsString = component.into();
            // For each component in the path
            match cursor_opt {
                Some(ref cursor) => {
                    // Easy case: If cursor_opt is already present, just call get
                    next = cursor.borrow().get(&component); // Store next cursor_opt in next, to be updated at the end of the loop
                    if next.is_none() {
                        // There is no node here, so we need to inset one.
                        next = Some(if i == num_components - 2 {
                            // Last component, so insert a node with the value
                            DirectoryTrieNode::new(value.clone())
                        } else {
                            // Intermediate component
                            DirectoryTrieNode::new_intermediate()
                        });
                        cursor.borrow_mut().insert(component, next.clone().unwrap());
                        cursor_opt.clone_from(&next);
                    }
                }
                None => {
                    // The harder case. Now we need to create a new trie node.
                    cursor_opt = Some(if i == num_components - 1 - skip_num {
                        // This is the last component, so create the trie node with a value
                        DirectoryTrieNode::new(value.clone())
                    } else {
                        // This is not the last component, so create an intermediate trie node
                        DirectoryTrieNode::new_intermediate()
                    });

                    // Map to this node from the last node by the current path component
                    last_cursor_opt
                        .unwrap()
                        .borrow_mut()
                        .insert(component, cursor_opt.clone().unwrap().clone());
                    next = None;
                }
            }
            last_cursor_opt = cursor_opt;
            cursor_opt = next.clone();
        }
    }

    #[allow(clippy::unnecessary_unwrap)]
    pub fn get<P: AsRef<Path>>(&self, path: P) -> Option<T> {
        if path.as_ref().iter().count() == 1 {
            // The request MUST be for root "/"
            return self.head.as_ref().and_then(|node| node.borrow().value());
        }

        let mut cursor_node_opt = self.head.as_ref().map(|node| node.clone()); // Clone current head

        // Store a head value as most recent one seen.
        let mut last_val_opt = cursor_node_opt
            .as_ref()
            .and_then(|node| node.borrow().value());
        let mut next;

        for component in path.as_ref().iter().skip(1) {
            // Iterate through path components
            if let Some(ref cursor_node) = cursor_node_opt {
                // Cursor node present
                next = cursor_node.borrow().get(component); // Get next node by component
                if next.is_none() {
                    // There's nothing else for us to traverse
                    break;
                } else {
                    // If there is a next node (indexed by the current path component)
                    let val_opt = next.clone().unwrap().borrow().value();
                    if val_opt.is_some() {
                        // If this next node has a value, store it in our last seen value variable
                        last_val_opt = val_opt;
                    }
                }
            } else {
                unreachable!();
            }
            cursor_node_opt = next; // Assign next loop's cursor node to the PRESENT next node found this loop
        }

        // Return the last value we saw
        last_val_opt
    }
}

impl<T: Clone> Default for DirectoryTrie<T> {
    fn default() -> Self {
        Self::new()
    }
}
