use std::sync::{Arc, Mutex};
use std::thread;

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
    node: Option<Node>,
    index: usize,
}

impl TreeHash {
    fn new(stack: Arc<Mutex<Stack>>) -> Self {
        Self {
            stack,
            index: 0,
            node: None,
        }
    }

    fn initialize(&mut self, index: usize) {
        self.index = 0;
        self.node = None;
    }

    fn lowest_tail_node_height() -> usize {
        todo!();
    }

    /// Executes the treehash alogirthm once.
    /// After the last call the stack contains one node, the desired inner node on height h.
    fn update(&mut self) {
        let stack = Arc::clone(&self.stack);
        thread::spawn(move || {
            let stack = stack.lock();
        });

        todo!();
    }
}

#[derive(Clone, Debug)]
struct Leaf {}

#[derive(Clone, Debug)]
struct Keep;

#[derive(Clone, Debug)]
struct TailNode {
    hash: String,
    height: usize,
    leaf_index: usize,
}
