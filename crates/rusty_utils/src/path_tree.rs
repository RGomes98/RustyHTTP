use std::{collections::HashMap, str::Split};
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum PathTreeError {
    #[error("Node already has a value at path: {0}")]
    DuplicatePath(String),
}

#[derive(Debug)]
pub struct PathTree<T> {
    root: Node<T>,
}

impl<T> Default for PathTree<T> {
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

impl<T> PathTree<T> {
    pub fn new() -> Self {
        Self { root: Node::default() }
    }

    pub fn insert(&mut self, path: &str, value: T) -> Result<(), PathTreeError> {
        let segments: Split<&str> = path.trim_matches('/').split("/"); // TODO: hardcoded? helper?
        let mut current: &mut Node<T> = &mut self.root;

        for segment in segments {
            if segment.is_empty() {
                continue;
            }

            current = current.children.entry(segment.to_string()).or_default();
        }

        if current.value.is_some() {
            return Err(PathTreeError::DuplicatePath(path.to_string()));
        };

        current.value = Some(value);
        Ok(())
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
        let mut path_tree: PathTree<u8> = PathTree::new();

        path_tree.insert("/users/list", 1).unwrap();
        path_tree.insert("/users/comments", 2).unwrap();
        path_tree.insert("/users/likes", 3).unwrap();

        assert_eq!(path_tree.find("/users/list").value, Some(1));
        assert_eq!(path_tree.find("/users/comments").value, Some(2));
        assert_eq!(path_tree.find("/users/likes").value, Some(3));
    }

    #[test]
    fn duplicated_path() {
        const PATH: &str = "/users/list";

        let mut path_tree: PathTree<u8> = PathTree::new();
        path_tree.insert(PATH, 1).unwrap();

        let result: Result<(), PathTreeError> = path_tree.insert(PATH, 2);
        assert_eq!(result, Err(PathTreeError::DuplicatePath(PATH.to_string())));
    }
}
