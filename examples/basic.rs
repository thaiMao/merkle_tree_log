use merkle_tree_log::{Leaf, MerkleTree, Node, Stack, TailNode, TreeHash};
use std::boxed::Box;
use std::sync::{Arc, Mutex};

fn main() {
    let merkle_tree_height: usize = 4;
    let number_of_leaves = merkle_tree_height.pow(2);
    let mut leaves = Vec::with_capacity(number_of_leaves);

    // Create leaves
    for index in 0..number_of_leaves {
        let content = "Hello, world";
        leaves.push(Leaf::new(content, index));
    }

    // A collection of nodes representing the current authentication path.
    let mut auth_path: Vec<Box<dyn Node>> = vec![];

    let stack = Stack::default();
    let stack = Arc::new(Mutex::new(stack));
    let tree_hash: TreeHash<TailNode> = TreeHash::new(stack, &leaves);

    let second_leaf = match leaves.get(1).take() {
        Some(leaf) => leaf,
        None => unreachable!(),
    };
    auth_path.push(Box::new(TailNode::from(*second_leaf)));
}
