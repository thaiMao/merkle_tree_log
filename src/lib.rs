use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
struct Stack {}

/// * `retain` - Holds a single right authentication node at height MERKLE_TREE_HEIGHT - 2.
/// * `current_authentication_path` - A list of nodes representing the current authentication path.
/// * `keep` - A list of nodes stored for efficient computation of left nodes.
#[derive(Clone, Debug)]
pub struct MerkleTree<const MERKLE_TREE_HEIGHT: usize> {
    retain: Node,
    treehashes: Vec<TreeHash>,
    current_authentication_path: Vec<Node>,
    leaves: Vec<Leaf>,
    keep: Keep,
}

#[derive(Clone, Debug)]
struct Node {}

#[derive(Clone, Debug)]
struct TreeHash {
    stack: Arc<Mutex<Stack>>,
}

#[derive(Clone, Debug)]
struct Leaf {}

#[derive(Clone, Debug)]
struct Keep;
