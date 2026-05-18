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
    pub fn size(&self) -> u32 {
        match self {
            Self::Integer(tree) => tree.size(),
            Self::String(tree) => tree.size(),
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
    /// Create a new [`Node`] with no children.
    ///
    /// # Arguments
    ///
    /// * `value` - The value stored by the new [`Node`].
    ///
    /// # Return
    /// A new [`Node`] with no children.
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

    fn add(&mut self, value: T) {
        match &mut self.0 {
            None => self.0 = Some(Box::new(Node::new(value))),
            Some(node) => match value.cmp(&node.value) {
                Ordering::Less => node.left.add(value),
                Ordering::Equal => {}
                Ordering::Greater => node.right.add(value),
            },
        }
    }

    fn has_left_child(&self) -> bool {
        match &self.0 {
            None => false,
            Some(node) => match &node.left.0 {
                None => false,
                Some(_node) => true,
            },
        }
    }

    fn has_right_child(&self) -> bool {
        match &self.0 {
            None => false,
            Some(node) => match &node.right.0 {
                None => false,
                Some(_node) => true,
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

    fn size(&self) -> u32 {
        match &self.0 {
            None => 0,
            Some(node) => node.left.size() + 1 + node.right.size(),
        }
    }

    fn to_string(&self, depth: u32) -> String {
        match &self.0 {
            None => String::from(""),
            Some(node) => {
                let mut output_string: String = String::from("");
                if depth > 1 {
                    output_string += ""
                }

                if depth > 0 {
                    output_string += "   ".repeat((depth - 1) as usize).as_str();
                    if self.has_right_child() {
                        output_string += "├─ ";
                    
                    } else {
                        output_string += "└─ ";
                    };
                }

                output_string += format!(
                    "{}\n{}{}",
                    node.to_string(),
                    &node.left.to_string(depth + 1),
                    &node.right.to_string(depth + 1)
                )
                .as_str();

                return output_string;
            }
        }
    }
}

impl<T: Ord + ToString> ToString for Subtree<T> {
    fn to_string(&self) -> String {
        match &self.0 {
            None => String::from("(empty)"),
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

    pub fn add(&mut self, value: T) {
        self.root.add(value);
    }

    pub fn has(&self, value: &T) -> bool {
        self.root.has(value)
    }

    pub fn size(&self) -> u32 {
        self.root.size()
    }
}
