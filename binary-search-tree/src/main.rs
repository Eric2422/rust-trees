mod binary_search_tree;

use std::env;

use binary_search_tree::BinarySearchTree;

fn main() {
    let mut tree: BinarySearchTree<String> = BinarySearchTree::new();

    for arg in env::args().skip(1) {
        tree.insert(arg);
    }

    let len = tree.len();
    println!("Initial tree size: {len}");
    let tree_string = tree.to_string();
    println!("Tree: {tree_string}");
}
