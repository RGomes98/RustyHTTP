use std::collections::HashMap;

#[derive(Debug)]
pub enum Segment<'a> {
    Exact(&'a str),
    Param(&'a str),
}

#[derive(Debug)]
pub struct PathMatch<'a, 'b, T> {
    pub value: &'a T,
    pub params: Vec<(&'a str, &'b str)>,
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
    exact_child: HashMap<String, Node<T>>,
    param_child: Option<(String, Box<Node<T>>)>,
}

impl<T> Default for Node<T> {
    fn default() -> Self {
        Self {
            value: None,
            param_child: None,
            exact_child: HashMap::new(),
        }
    }
}

impl<T> PathTree<T> {
    pub fn new() -> Self {
        Self { root: Node::default() }
    }

    pub fn insert<'a, I>(&mut self, segments: I, value: T)
    where
        I: Iterator<Item = Segment<'a>>,
    {
        let mut current: &mut Node<T> = &mut self.root;

        for path in segments {
            match path {
                Segment::Exact(path) => {
                    current = current.exact_child.entry(path.into()).or_default();
                }
                Segment::Param(name) => {
                    current = &mut current
                        .param_child
                        .get_or_insert((name.into(), Box::new(Node::default())))
                        .1;
                }
            }
        }

        current.value = Some(value);
    }

    pub fn find<'a, 'b, I>(&'a self, segments: I) -> Option<PathMatch<'a, 'b, T>>
    where
        I: Iterator<Item = &'b str>,
    {
        let mut params: Vec<(&str, &str)> = Vec::with_capacity(4);
        let mut current: &Node<T> = &self.root;

        for path in segments {
            if let Some(next_node) = current.exact_child.get(path) {
                current = next_node
            } else if let Some((key, next_node)) = &current.param_child {
                params.push((key.as_str(), path));
                current = next_node
            } else {
                return None;
            }
        }

        current.value.as_ref().map(|val: &T| PathMatch { value: val, params })
    }
}
