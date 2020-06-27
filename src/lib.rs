//! Very simple trie implementation to have fast implementations of:
//!  - `common_prefix_search`: give every item in the trie that is
//!  a prefix of the query
//!  - `search`: give every item in the trie that would follow the query
//!  In order for search to be fast, an index has to be built, which can
//!  become very large if you intend to store many items in the trie.
//!  PR welcomes to store partial indexes for large tries.
//!
//! ```
//! use trie_rs::TrieBuilder;
//!
//! let build_index = true;
//! let mut builder = TrieBuilder::new(build_index);
//! builder.push(&vec!['A', 'l', 'a', 'b', 'a', 'm', 'a']);
//! builder.push(&vec!['A', 'l', 'a', 's', 'k', 'a']);
//! builder.push(&vec!['A', 'l', 'a', 's']);
//! let trie = builder.build();
//! assert_eq!(trie.search(&vec!['A', 'l', 'a', 's']).unwrap(),
//! &vec![
//!     vec!['A', 'l', 'a', 's'],
//!     vec!['A', 'l', 'a', 's', 'k', 'a'],
//! ]);
//! assert_eq!(trie.common_prefix_search(&vec!['A', 'l', 'a', 's', 'k', 'a']),
//! vec![
//!     vec!['A', 'l', 'a', 's'],
//!     vec!['A', 'l', 'a', 's', 'k', 'a'],
//! ]);
//! ```
//!
//! The item stored in the Trie needs eq + Hash as under the hood we use
//! a hashmap for fast query. We also need copy because most of the time
//! we will use very small items as trie elements, like `char` or `u8` for
//! strings, or ints.
//!
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Clone)]
pub struct TrieBuilder<Label> {
    build_search_index: bool,
    trie: Trie<Label>,
}

impl<Label: Eq + Hash + Copy> TrieBuilder<Label> {
    pub fn new(build_search_index: bool) -> Self {
        Self {
            build_search_index,
            trie: Trie::<Label>::default(),
        }
    }

    pub fn push(&mut self, element: &[Label]) {
        self.trie.push(element);
    }

    pub fn build(mut self) -> Trie<Label> {
        if self.build_search_index {
            self.trie.build_index();
        }
        self.trie
    }
}

#[derive(Debug, Clone)]
pub enum TrieError {
    /// Attempt to use search on a trie, that did not build the index.
    /// Run trie.build_index() first.
    IndexNotBuilt,
    /// Your trie does not have any result for this search.
    NoResultFound,
}

#[derive(Debug, Clone)]
pub struct Trie<Label> {
    has_search_index: bool,
    root: Node<Label>,
}

fn _build_index<Label: Eq + Hash + Copy>(
    node: &mut Node<Label>,
    current_words: &mut Vec<Vec<Label>>,
    prefix: &mut Vec<Label>,
) {
    if node.is_leaf {
        current_words.push(prefix.to_vec());
    }
    for (label, mut child) in node.children.iter_mut() {
        prefix.push(*label);
        let mut new_words = vec![];
        _build_index(&mut child, &mut new_words, prefix);
        current_words.extend(new_words);
        prefix.pop();
    }
    node.subwords = current_words.clone();
}

impl<Label: Eq + Hash + Copy> Trie<Label> {
    /// Does a search within the trie in constant time (index is built ahead of time).
    /// ```
    /// use trie_rs::TrieBuilder;
    ///
    /// let build_index = true;
    /// let mut builder = TrieBuilder::new(build_index);
    /// builder.push(&vec!['A', 'l', 'a', 'b', 'a', 'm', 'a']);
    /// builder.push(&vec!['A', 'l', 'a', 's', 'k', 'a']);
    /// builder.push(&vec!['A', 'l', 'a', 's']);
    /// let trie = builder.build();
    /// assert_eq!(trie.search(&vec!['A', 'l', 'a', 's']).unwrap(),
    /// &vec![
    ///     vec!['A', 'l', 'a', 's'],
    ///     vec!['A', 'l', 'a', 's', 'k', 'a'],
    /// ]);
    /// ```
    pub fn search(&self, element: &[Label]) -> Result<&Vec<Vec<Label>>, TrieError> {
        if !self.has_search_index {
            return Err(TrieError::IndexNotBuilt);
        }
        let mut node = &self.root;
        for label in element.iter() {
            let child_opt = node.children.get(label);
            if let Some(child) = child_opt {
                node = child;
            } else {
                return Err(TrieError::NoResultFound);
            }
        }
        Ok(&node.subwords)
    }

    pub fn build_index(&mut self) {
        // let node = &mut self.root;
        let mut current_words = vec![];
        let mut prefix = vec![];
        _build_index(&mut self.root, &mut current_words, &mut prefix);

        self.has_search_index = true;
    }

    pub fn push(&mut self, element: &[Label]) {
        let mut node = &mut self.root;
        for label in element.iter() {
            node = node.children.entry(*label).or_insert_with(Node::default);
        }
        node.is_leaf = true;
    }

    /// Does a common prefix search in O(n) n being the number of labels in the query
    /// ```
    /// use trie_rs::TrieBuilder;
    ///
    /// let build_index = false;
    /// let mut builder = TrieBuilder::new(build_index);
    /// builder.push(&vec!['A', 'l', 'a', 'b', 'a', 'm', 'a']);
    /// builder.push(&vec!['A', 'l', 'a', 's', 'k', 'a']);
    /// builder.push(&vec!['A', 'l', 'a', 's']);
    /// let trie = builder.build();
    /// assert_eq!(trie.common_prefix_search(&vec!['A', 'l', 'a', 's', 'k', 'a']),
    /// vec![
    ///     vec!['A', 'l', 'a', 's'],
    ///     vec!['A', 'l', 'a', 's', 'k', 'a'],
    /// ]);
    /// ```
    pub fn common_prefix_search(&self, element: &[Label]) -> Vec<Vec<Label>> {
        let mut node = &self.root;
        let mut results = vec![];
        let mut prefix = vec![];
        for label in element.iter() {
            prefix.push(*label);
            let child_opt = node.children.get(label);
            if let Some(child) = child_opt {
                node = child;
                if node.is_leaf {
                    results.push(prefix.clone());
                }
            } else {
                return results;
            }
        }
        results
    }
}

impl<Label> Default for Trie<Label> {
    fn default() -> Self {
        Trie {
            has_search_index: false,
            root: Node::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Node<Label> {
    is_leaf: bool,
    subwords: Vec<Vec<Label>>,
    children: HashMap<Label, Node<Label>>,
}

impl<Label> Default for Node<Label> {
    fn default() -> Self {
        Node {
            is_leaf: false,
            subwords: vec![],
            children: HashMap::new(),
        }
    }
}
