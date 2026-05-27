use std::cmp::Ordering;

/// Enum to distinguish between different generic types for
/// [`BinarySearchTree`].
pub enum BinarySearchTreeType {
    Integer(BinarySearchTree<i32>),
    String(BinarySearchTree<String>),
}

impl ToString for BinarySearchTreeType {
    /// Return a [`String`] that represents the elements of the tree.
    fn to_string(&self) -> String {
        match self {
            Self::Integer(tree) => tree.to_string(),
            Self::String(tree) => tree.to_string(),
        }
    }
}

impl BinarySearchTreeType {
    /// Return the size of the tree (i.e., how many elements are in it).
    ///
    /// # Return
    ///
    /// The size of the tree (i.e., how many elements are in it).
    pub fn size(&self) -> u32 {
        match self {
            Self::Integer(tree) => tree.size(),
            Self::String(tree) => tree.size(),
        }
    }
}

/// A node to be used in [`BinarySearchTree`].
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
    ///
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
/// Serves as a wrapper for possibly null [`Node`]s.
struct Subtree<T: Ord + ToString>(Option<Box<Node<T>>>);

impl<T: Ord + ToString> ToString for Subtree<T> {
    fn to_string(&self) -> String {
        match &self.0 {
            None => String::from("(empty)"),
            Some(_node) => self.to_string(0, false),
        }
    }
}

impl<T: Ord + ToString> Subtree<T> {
    /// Create an empty subtree (i.e, a null [`Node`]).
    fn new() -> Self {
        Self(None)
    }

    /// 获取最小值。
    ///
    /// # 返回
    ///
    /// 最小值。
    fn get_min(self) -> T {
        match self.0 {
            None => self.0.unwrap().value,
            Some(node) => node.left.get_min(),
        }
    }

    /// 获取最大值。
    ///
    /// # 返回
    ///
    /// 最大值。
    fn get_max(self) -> T {
        match self.0 {
            None => self.0.unwrap().value,
            Some(node) => node.right.get_max(),
        }
    }

    /// Add a new [`Node`] to the [`Subtree`], following the rules of binary
    /// search trees. Duplicate values are ignored.
    ///
    /// # Arguments
    ///
    /// * `value` - The value that the new [`Node`] should contain.
    ///
    /// # Return
    ///
    /// Whether a new node was added or not. Adding a new node fails when the
    /// value is duplicate. Otherwise, it succeeds.
    fn add(&mut self, value: T) -> bool {
        match &mut self.0 {
            None => {
                self.0 = Some(Box::new(Node::new(value)));
                true
            }
            Some(node) => match value.cmp(&node.value) {
                Ordering::Less => node.left.add(value),
                Ordering::Equal => false,
                Ordering::Greater => node.right.add(value),
            },
        }
    }

    /// Return whether this [`Subtree`] has a left child.
    ///
    /// # Return
    ///
    /// Whether this [`Subtree`] has a left child.
    fn has_left_child(&self) -> bool {
        match &self.0 {
            None => false,
            Some(node) => match &node.left.0 {
                None => false,
                Some(_node) => true,
            },
        }
    }

    /// Return whether this [`Subtree`] has a right child.
    ///
    /// # Return
    ///
    /// Whether this [`Subtree`] has a right child.
    fn has_right_child(&self) -> bool {
        match &self.0 {
            None => false,
            Some(node) => match &node.right.0 {
                None => false,
                Some(_node) => true,
            },
        }
    }

    /// Return whether a given value is contained within this [`Subtree`].
    /// Recursively searches using the rules of [`BinarySearchTree`]s.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to search for in the tree.
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

    /// Return the size of the tree (i.e., how many elements are in it).
    ///
    /// # Return
    ///
    /// The size of the tree (i.e., how many elements are in it).
    fn size(&self) -> u32 {
        match &self.0 {
            None => 0,
            Some(node) => node.left.size() + 1 + node.right.size(),
        }
    }

    /// Recursively generate a [`String`] to represent the elements in this
    /// [`Subtree`].
    ///
    /// # Arguments
    ///
    /// * `depth` -       The depth of the current [`Subtree`] in the
    ///   [`BinarySearchTree`].
    /// * `make_branch` - Controls whether "├─ " or "└─ " will be printed before
    ///   the element.
    ///
    /// # Return
    ///
    /// The [`String`] representing this [`Subtree`], including all descendants.
    fn to_string(&self, depth: u32, make_branch: bool) -> String {
        match &self.0 {
            None => String::from(""),
            Some(node) => {
                let mut output_string: String = String::from("");

                for _i in 1..depth {
                    output_string += "   ";
                }

                if depth > 0 {
                    output_string += if make_branch { "├─ " } else { "└─ " };
                }

                let child_branches = self.has_left_child() && self.has_right_child();

                output_string += format!(
                    "{}\n{}{}",
                    node.to_string(),
                    &node.left.to_string(depth + 1, child_branches),
                    &node.right.to_string(depth + 1, false)
                )
                .as_str();

                return output_string;
            }
        }
    }
}

/// A binary search tree that does not accept duplicate values. Makes no attempt
/// to balance the tree.
pub struct BinarySearchTree<T: Ord + ToString> {
    root: Subtree<T>,
}

impl<T: Ord + ToString> ToString for BinarySearchTree<T> {
    fn to_string(&self) -> String {
        self.root.to_string(0, false)
    }
}

impl<T: Ord + ToString> BinarySearchTree<T> {
    /// Create an empty [`BinarySearchTree`], one with no root [`Node`].
    pub fn new() -> Self {
        Self {
            root: Subtree::new(),
        }
    }

    /// Add a new [`Node`] to the [`Subtree`], following the rules of binary
    /// search trees.
    ///
    /// # Arguments
    ///
    /// * `value` - The value that the new [`Node`] should contain.
    ///
    /// # Return
    ///
    /// Whether the value was successfully added.
    pub fn add(&mut self, value: T) -> bool {
        self.root.add(value)
    }

    /// Return whether a given value is contained within this [`Subtree`].
    /// Recursively searches using the rules of [`BinarySearchTree`]s.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to search for in the tree.
    pub fn has(&self, value: &T) -> bool {
        self.root.has(value)
    }

    /// Return the size of the tree (i.e., how many elements are in it).
    ///
    /// # Return
    ///
    /// The size of the tree (i.e., how many elements are in it).
    pub fn size(&self) -> u32 {
        self.root.size()
    }
}
