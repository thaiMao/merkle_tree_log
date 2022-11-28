use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Default)]
pub struct Stack {
    nodes: Vec<Box<dyn Node + Send + Sync>>,
}

impl Stack {
    fn pop(&mut self) -> Option<Box<dyn Node + Send + Sync>> {
        self.nodes.pop()
    }

    fn push(&mut self, element: Box<dyn Node + Send + Sync>) {
        self.nodes.push(element);
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
    leaves: Vec<Leaf<'a>>,
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
                    // TODO Pass in correct arguments
                    let hash = [0; 32];
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

    fn j(&self) -> usize {
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
pub struct TreeHash<'a, T>
where
    T: Node,
{
    stack: Arc<Mutex<Stack>>,
    node: Option<T>,
    index: usize,
    leaves: &'a [Leaf<'a>],
}

impl<'a, T> TreeHash<'a, T>
where
    T: Node + Send + Sync,
{
    pub fn new(stack: Arc<Mutex<Stack>>, leaves: &'a [Leaf]) -> Self {
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
    fn update(
        &mut self,
        leaf_index: usize,
        level: usize,
        hash: impl Fn(String) -> String + Send + 'static,
    ) {
        let hashed_leaf = self.hash(self.leaves.get(leaf_index));

        let stack = Arc::clone(&self.stack);
        thread::spawn(move || {
            let mut height = 0;
            let original_leaf = TailNode::new(hashed_leaf, height, leaf_index);

            let mut leaf = original_leaf;
            let mut stack = stack.lock().unwrap();

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
                let mut j = leaf_index;

                for _ in 0..height {
                    j = if is_even(j) { j } else { j - 1 };
                    j /= 2;
                }

                leaf = TailNode::new(create_hash(&prehash), height, j);
            }

            stack.push(Box::new(leaf));
        });

        todo!();
    }

    fn hash<U>(&self, content: U) -> [u8; 32] {
        todo!();
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Leaf<'a> {
    index: usize,
    height: usize,
    content: &'a str,
}

impl<'a> Leaf<'a> {
    pub fn new(content: &'a str, index: usize) -> Self {
        Self {
            index,
            height: 0,
            content,
        }
    }

    fn left_node(&self) -> bool {
        if (self.index as f32 + 1.0) % 2.0 == 0.0 {
            true
        } else {
            false
        }
    }

    fn print(&self) -> String {
        todo!();
    }
}

impl Node for Leaf<'_> {
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
    hash: [u8; 32],
    height: usize,
    leaf_index: usize,
}

impl TailNode {
    fn new(hash: [u8; 32], height: usize, leaf_index: usize) -> Self {
        Self {
            hash,
            height,
            leaf_index,
        }
    }
}

impl<'a> From<Leaf<'a>> for TailNode {
    fn from(leaf: Leaf<'a>) -> Self {
        let hash = create_hash(leaf.content);

        Self {
            hash,
            height: 0,
            leaf_index: leaf.index,
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

fn is_even(index: usize) -> bool {
    todo!();
}

///
///
/// * `leaf` -
/// * `leaves` -
/// * `authentication_path` -
///
fn calculate_root(
    mut index: usize,
    leaves: &[Leaf],
    authentication_path: &[Box<dyn Node>],
) -> [u8; 32] {
    let mut leaf = TailNode::from(leaves[index]);

    for neighbor in authentication_path.iter() {
        let mut prehash = String::new();

        if neighbor.height() == 0 {
            if index < neighbor.j() {
                prehash.push_str(&leaf.hash());
                prehash.push_str(&neighbor.hash());
            } else {
                prehash.push_str(&neighbor.hash());
                prehash.push_str(&leaf.hash());
            }
        } else {
            if leaf.j() < neighbor.j() {
                prehash.push_str(&leaf.hash());
                prehash.push_str(&neighbor.hash());
            } else {
                prehash.push_str(&neighbor.hash());
                prehash.push_str(&leaf.hash());
            }
        }
        index = (leaf.j().max(neighbor.j()) + 1) / 2 - 1;

        leaf = TailNode::new(create_hash(&prehash), index, 0);
    }

    leaf.hash
}

fn create_hash(content: &str) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    let result = hasher.finalize();

    result.into()
}
