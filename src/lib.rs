use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Clone, Debug)]
struct Stack {}

/// * `retain` - Holds a single right authentication node at height MERKLE_TREE_HEIGHT - 2.
/// * `current_authentication_path` - A list of nodes representing the current authentication path.
/// * `keep` - A list of nodes stored for efficient computation of left nodes.
pub struct MerkleTree<const MERKLE_TREE_HEIGHT: usize, T>
where
    T: Node,
{
    retain: InternalNode,
    treehashes: Vec<TreeHash<T>>,
    current_authentication_path: Vec<Box<dyn Node>>,
    leaves: Vec<Leaf>,
    keep: HashSet<usize, Box<dyn Node>>,
}

impl<const MERKLE_TREE_HEIGHT: usize, T> MerkleTree<MERKLE_TREE_HEIGHT, T>
where
    T: Node + Clone + Debug,
{
    /// Update and output phase of merkle tree traversal.
    /// * `leaf` - The current leaf.
    /// Returns the current authentication path.
    fn update_and_output<'a>(&mut self, leaf: Leaf) -> &'a [Box<dyn Node>] {
        let first_parent_left_node_height = Self::get_first_left_node_parent_height(leaf);

        // Check if the parent leaf at height first_parent_left_node_height + 1 is a left node.
        let is_left_node = (first_parent_left_node_height + 1).pow(2) % 2 == 0
            && first_parent_left_node_height + 1 != MERKLE_TREE_HEIGHT;

        // If it is a left node then authentication at height firstParentLeftNodeHeight is a right node and should be stored in `Keep`.
        if is_left_node && first_parent_left_node_height < MERKLE_TREE_HEIGHT - 1 {
            //self.keep.insert(first_parent_left_node_height);
        }

        if leaf.left_node() {
            if first_parent_left_node_height == 0 {
                if let Some(node) = self.current_authentication_path.get_mut(0) {
                    let hash = String::from("");
                    let height = 0;
                    let leaf_index = 0;
                    *node = Box::new(TailNode::new(hash, height, leaf_index));
                };
            }
        }
        todo!()
    }

    fn get_first_left_node_parent_height(leaf: Leaf) -> usize {
        let mut height = 0;

        if leaf.left_node() {
            height
        } else {
            let mut node = leaf;

            while !node.left_node() {
                height += 1;

                if node.even() {
                    node.index = (node.index + 2) / (2 - 1);
                } else {
                    node.index = (node.index + 1) / (2 - 1);
                }
            }

            height
        }
    }
}

pub trait Node {
    fn even(&self) -> bool {
        todo!();
    }
}

impl InternalNode {
    fn even(&self) -> bool {
        todo!()
    }
}

#[derive(Clone, Debug)]
struct TreeHash<T>
where
    T: Node,
{
    stack: Arc<Mutex<Stack>>,
    node: Option<T>,
    index: usize,
}

impl<T> TreeHash<T>
where
    T: Node,
{
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

#[derive(Clone, Copy, Debug)]
struct Leaf {
    index: usize,
}

impl Leaf {
    fn left_node(&self) -> bool {
        if (self.index as f32 + 1.0) % 2.0 == 0.0 {
            true
        } else {
            false
        }
    }
}

impl Node for Leaf {}

#[derive(Clone, Debug)]
struct Keep;

#[derive(Clone, Debug)]
struct TailNode {
    hash: String,
    height: usize,
    leaf_index: usize,
}

impl TailNode {
    fn new(hash: String, height: usize, leaf_index: usize) -> Self {
        Self {
            hash,
            height,
            leaf_index,
        }
    }
}

impl Node for TailNode {}

#[derive(Clone, Debug)]
struct InternalNode;
