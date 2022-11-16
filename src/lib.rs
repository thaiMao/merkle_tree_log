use std::collections::HashSet;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use std::thread;

struct Stack {
    nodes: Vec<Box<dyn Node + Send + Sync>>,
}

impl Stack {
    fn pop(&mut self) -> Option<Box<dyn Node + Send + Sync>> {
        self.nodes.pop()
    }
}

/// * `retain` - Holds a single right authentication node at height MERKLE_TREE_HEIGHT - 2.
/// * `current_authentication_path` - A list of nodes representing the current authentication path.
/// * `keep` - A list of nodes stored for efficient computation of left nodes.
pub struct MerkleTree<'a, const MERKLE_TREE_HEIGHT: usize, T>
where
    T: Node,
{
    retain: InternalNode,
    treehashes: Vec<TreeHash<'a, T>>,
    current_authentication_path: Vec<Box<dyn Node>>,
    leaves: Vec<Leaf>,
    keep: HashSet<usize, Box<dyn Node>>,
}

impl<'a, const MERKLE_TREE_HEIGHT: usize, T> MerkleTree<'a, MERKLE_TREE_HEIGHT, T>
where
    T: Node + Clone + Debug,
{
    /// Update and output phase of merkle tree traversal.
    /// * `leaf` - The current leaf.
    /// Returns the current authentication path.
    fn update_and_output<'b>(&mut self, leaf: Leaf) -> &'b [Box<dyn Node>] {
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

    fn height(&self) -> usize;

    fn hash(&self) -> &str;
}

impl InternalNode {
    fn even(&self) -> bool {
        todo!()
    }
}

#[derive(Clone)]
struct TreeHash<'a, T>
where
    T: Node,
{
    stack: Arc<Mutex<Stack>>,
    node: Option<T>,
    index: usize,
    leaves: &'a [Leaf],
}

impl<'a, T> TreeHash<'a, T>
where
    T: Node + Send + Sync,
{
    fn new(stack: Arc<Mutex<Stack>>, leaves: &'a [Leaf]) -> Self {
        Self {
            stack,
            index: 0,
            node: None,
            leaves,
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
    fn update(&mut self, leaf_index: usize, level: usize) {
        let hashed_leaf = self.hash(self.leaves.get(leaf_index));
        let mut height = 0;
        let original_leaf = TailNode::new(hashed_leaf, height, leaf_index);
        let leaf = original_leaf;

        let stack = Arc::clone(&self.stack);
        thread::spawn(move || {
            let stack = stack.lock().unwrap();

            while stack.nodes.len() != 0 && stack.nodes.last().unwrap().height() == leaf.height {
                let top_node = stack.pop().unwrap();
                let mut prehash = String::new();

                if top_node.j() < leaf.j() {
                    prehash.push_str(top_node.hash());
                    prehash.push_str(leaf.hash());
                } else {
                    prehash.push_str(leaf.hash());
                    prehash.push_str(top_node.hash());
                };

                height = leaf.height() + 1;
                let j = leaf_index;
            }
        });

        todo!();
    }

    fn hash<U>(&self, content: U) -> String {
        todo!();
    }
}

#[derive(Clone, Copy, Debug)]
struct Leaf {
    index: usize,
    height: usize,
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

impl Node for Leaf {
    fn height(&self) -> usize {
        self.height
    }

    fn hash(&self) -> &str {
        todo!()
    }
}

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

impl Node for TailNode {
    fn height(&self) -> usize {
        self.height
    }

    fn hash(&self) -> &str {
        todo!()
    }
}

#[derive(Clone, Debug)]
struct InternalNode;
