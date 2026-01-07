use std::{collections::HashMap, str::Split};

#[derive(Debug)]
pub struct Trie<T> {
    root: Node<T>,
}

impl<T> Default for Trie<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct Node<T> {
    value: Option<T>,
    children: HashMap<String, Node<T>>,
}

impl<T> Default for Node<T> {
    fn default() -> Self {
        Self {
            value: None,
            children: HashMap::new(),
        }
    }
}

impl<T> Trie<T> {
    pub fn new() -> Self {
        Self { root: Node::default() }
    }

    pub fn insert(&mut self, path: &str, value: T) {
        let segments: Split<&str> = path.trim_matches('/').split("/"); // TODO: hardcoded? helper?
        let mut current: &mut Node<T> = &mut self.root;

        for segment in segments {
            if segment.is_empty() {
                continue;
            }

            if !current.children.contains_key(segment) {
                current.children.insert(segment.to_string(), Node::default());
                current = current.children.get_mut(segment).unwrap(); // TODO: valid unwrap
            }
        }

        current.value = Some(value);
    }

    pub fn find(&self, path: &str) -> &Node<T> {
        let segments: Split<&str> = path.trim_matches('/').split("/"); // TODO: hardcoded? helper?
        let mut current: &Node<T> = &self.root;

        for segment in segments {
            if segment.is_empty() {
                continue;
            }

            if let Some(node) = current.children.get(segment) {
                current = node
            }
        }

        current
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_and_find() {
        let mut trie: Trie<i32> = Trie::new();
        trie.insert("/api/users/list", 1);
        assert_eq!(trie.find("/api/users/list").value, Some(1));
    }
}
