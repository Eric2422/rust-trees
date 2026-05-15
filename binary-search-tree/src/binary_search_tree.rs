use std::cmp::Ordering;

pub enum BinarySearchTreeType {
    Integer(BinarySearchTree<i32>),
    String(BinarySearchTree<String>),
}

impl ToString for BinarySearchTreeType {
    fn to_string(&self) -> String {
        match self {
            Self::Integer(tree) => tree.to_string(),
            Self::String(tree) => tree.to_string(),
        }
    }
}

impl BinarySearchTreeType {
    pub fn len(&self) -> u32 {
        match self {
            Self::Integer(tree) => tree.len(),
            Self::String(tree) => tree.len(),
        }
    }
}

/// A node to be used in [`BinarySearchTree`].
#[derive(Debug)]
struct Node<T: Ord + ToString> {
    value: T,
    left: Subtree<T>,
    right: Subtree<T>,
}

impl<T: Ord + ToString> ToString for Node<T> {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}

impl<T: Ord + ToString> Node<T> {
    /// Create a new node with no children.
    ///
    /// # Arguments
    ///
    /// * `value` - The value stored by the new `Node`.
    pub fn new(value: T) -> Self {
        Self {
            value,
            left: Subtree::new(),
            right: Subtree::new(),
        }
    }
}

/// A subtree in [`BinarySearchTree`].
#[derive(Debug)]
struct Subtree<T: Ord + ToString>(Option<Box<Node<T>>>);

impl<T: Ord + ToString> Subtree<T> {
    /// Create an empty subtree (i.e, a null node).
    pub fn new() -> Self {
        Self(None)
    }

    fn insert(&mut self, value: T) {
        match &mut self.0 {
            None => self.0 = Some(Box::new(Node::new(value))),
            Some(node) => match value.cmp(&node.value) {
                Ordering::Less => node.left.insert(value),
                Ordering::Equal => {}
                Ordering::Greater => node.right.insert(value),
            },
        }
    }

    fn has(&self, value: &T) -> bool {
        match &self.0 {
            None => false,
            Some(node) => match value.cmp(&node.value) {
                Ordering::Less => node.left.has(value),
                Ordering::Equal => true,
                Ordering::Greater => node.right.has(value),
            },
        }
    }

    fn len(&self) -> u32 {
        match &self.0 {
            None => 0,
            Some(node) => node.left.len() + 1 + node.right.len(),
        }
    }

    fn to_string(&self, depth: u32) -> String {
        match &self.0 {
            None => String::from(""),
            Some(node) => format!(
                "{}{}: {depth}\n{}{}",
                "\t".repeat(depth as usize),
                node.to_string(),
                &node.left.to_string(depth + 1),
                &node.right.to_string(depth + 1)
            ),
        }
    }
}

impl<T: Ord + ToString> ToString for Subtree<T> {
    fn to_string(&self) -> String {
        match &self.0 {
            None => String::from(""),
            Some(_node) => self.to_string(0),
        }
    }
}

/// A binary search tree that does not accept duplicate values.
#[derive(Debug)]
pub struct BinarySearchTree<T: Ord + ToString> {
    root: Subtree<T>,
}

impl<T: Ord + ToString> ToString for BinarySearchTree<T> {
    fn to_string(&self) -> String {
        self.root.to_string(0)
    }
}

impl<T: Ord + ToString> BinarySearchTree<T> {
    pub fn new() -> Self {
        Self {
            root: Subtree::new(),
        }
    }

    pub fn insert(&mut self, value: T) {
        self.root.insert(value);
    }

    pub fn has(&self, value: &T) -> bool {
        self.root.has(value)
    }

    pub fn len(&self) -> u32 {
        self.root.len()
    }
}
