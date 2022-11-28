use merkle_tree_log::{Leaf, Level, MerkleTree, Node, Stack, TreeHash};
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
    let mut auth_path: Vec<Node> = vec![];

    let stack = Stack::default();
    let stack = Arc::new(Mutex::new(stack));
    let tree_hash: TreeHash = TreeHash::new(stack, &leaves);

    let second_leaf = match leaves.get(1).take() {
        Some(leaf) => leaf,
        None => unreachable!(),
    };
    auth_path.push(Node::from(*second_leaf));

    for level in 1..merkle_tree_height {
        const TWO: usize = 2;
        let start = TWO.pow(level as u32);
        let end = start + TWO.pow(level as u32);
        let mut auth_node = None;
        for leaf_index in start..end {
            tree_hash.update(leaf_index, Level(level));
            // TODO Handle None case.
            auth_node = tree_hash.first();
        }

        match auth_node {
            Some(node) => {
                auth_path.push(node);
            }
            None => {
                // TODO
            }
        }
    }
}
