use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Default)]
pub struct Stack {
    nodes: Vec<Node>,
}

impl Stack {
    fn pop(&mut self) -> Option<Node> {
        self.nodes.pop()
    }

    fn push(&mut self, element: Node) {
        self.nodes.push(element);
    }
}

/// * `retain` - Holds a single right authentication node at height MERKLE_TREE_HEIGHT - 2.
/// * `current_authentication_path` - A list of nodes representing the current authentication path.
/// * `keep` - A list of nodes stored for efficient computation of left nodes.
pub struct MerkleTree<'a, const MERKLE_TREE_HEIGHT: usize> {
    retain: Node,
    treehashes: Vec<TreeHash<'a>>,
    current_authentication_path: Vec<Node>,
    leaves: Vec<Leaf<'a>>,
    keep: HashSet<usize, Node>,
}

impl<'a, const MERKLE_TREE_HEIGHT: usize> MerkleTree<'a, MERKLE_TREE_HEIGHT> {
    /// Update and output phase of merkle tree traversal.
    /// * `leaf` - The current leaf.
    /// Returns the current authentication path.
    fn update_and_output<'b>(&mut self, leaf: Leaf) -> &'b [Node] {
        let first_parent_left_node_height = Self::get_first_left_node_parent_height(leaf);

        // Check if the parent leaf at height first_parent_left_node_height + 1 is a left node.
        let is_left_node = (first_parent_left_node_height + 1).pow(2) % 2 == 0
            && first_parent_left_node_height + 1 != MERKLE_TREE_HEIGHT;

        // If it is a left node then authentication at height firstParentLeftNodeHeight is a right node and should be stored in `Keep`.
        if is_left_node && first_parent_left_node_height < MERKLE_TREE_HEIGHT - 1 {
            //self.keep.insert(first_parent_left_node_height);
        }

        if Node::from(leaf).left_node() {
            if first_parent_left_node_height == 0 {
                if let Some(node) = self.current_authentication_path.get_mut(0) {
                    // TODO Pass in correct arguments
                    let hash = String::from("");
                    let height = 0;
                    let leaf_index = 0;
                    *node = Node::new(hash, leaf_index, Height(height));
                };
            }
        }
        todo!()
    }

    fn get_first_left_node_parent_height(leaf: Leaf) -> usize {
        let leaf = Node::from(leaf);
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

#[derive(Clone)]
pub struct TreeHash<'a> {
    stack: Arc<Mutex<Stack>>,
    node: Option<Node>,
    index: usize,
    leaves: &'a [Leaf<'a>],
}

impl<'a> TreeHash<'a> {
    pub fn new(stack: Arc<Mutex<Stack>>, leaves: &'a [Leaf]) -> Self {
        Self {
            stack,
            index: 0,
            node: None,
            leaves,
        }
    }

    /// Return a copy of the first node from the stack.
    pub fn first(&self) -> Option<Node> {
        let stack = Arc::clone(&self.stack);
        let stack = stack.lock().unwrap();
        stack.nodes.get(0).cloned()
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
    pub fn update(&self, leaf_index: usize, level: Level) {
        let leaf = match self.leaves.get(leaf_index) {
            Some(leaf) => *leaf,
            None => unreachable!(),
        };
        let hashed_leaf = create_hash(leaf.print());

        let stack = Arc::clone(&self.stack);
        thread::spawn(move || {
            let mut height = 0;
            let original_leaf = Node::new(hashed_leaf, leaf_index, Height(height));

            let mut node = original_leaf;
            let mut stack = stack.lock().unwrap();

            while stack.nodes.len() != 0 && stack.nodes.last().unwrap().height() == node.height() {
                let top_node = stack.pop().unwrap();
                let mut prehash = String::new();

                if top_node.j() < node.j() {
                    prehash.push_str(&top_node.hash());
                    prehash.push_str(&node.hash());
                } else {
                    prehash.push_str(&node.hash());
                    prehash.push_str(&top_node.hash());
                };

                height = node.height() + 1;
                let mut j = leaf_index;

                for _ in 0..height {
                    j = if is_even(j) { j } else { j - 1 };
                    j /= 2;
                }

                node = Node::new(create_hash(&prehash), j, Height(height));
            }

            stack.push(node);
        });
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

    fn print(&self) -> &'a str {
        self.content
    }
}

#[derive(Clone, Debug)]
struct Keep;

#[derive(Clone, Debug)]
struct Height(usize);

#[derive(Clone, Debug)]
pub struct Level(pub usize);

///
///
/// * `leaf` -
/// * `leaves` -
/// * `authentication_path` -
///
fn calculate_root(mut index: usize, leaves: &[Leaf], authentication_path: &[Node]) -> String {
    let mut leaf = Node::from(leaves[index]);

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

        leaf = Node::new(create_hash(&prehash), index, Height(0));
    }

    leaf.hash
}

fn create_hash(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());

    format!("{:X}", hasher.finalize())
}

#[derive(Clone, Debug)]
pub struct Node {
    hash: String,
    height: Height,
    index: usize,
}

impl Node {
    fn new(hash: String, index: usize, height: Height) -> Self {
        Self {
            hash,
            height,
            index,
        }
    }

    fn left_node(&self) -> bool {
        if (self.index as f32 + 1.0) % 2.0 == 0.0 {
            true
        } else {
            false
        }
    }

    fn height(&self) -> usize {
        self.height.0
    }

    fn hash(&self) -> &str {
        self.hash.as_str()
    }

    fn even(&self) -> bool {
        todo!();
    }

    fn j(&self) -> usize {
        self.index
    }
}

impl<'a> From<Leaf<'a>> for Node {
    fn from(leaf: Leaf<'a>) -> Self {
        let hash = create_hash(leaf.content);

        Self {
            hash,
            height: Height(0),
            index: leaf.index,
        }
    }
}

fn is_even(index: usize) -> bool {
    if index % 2 == 0 {
        true
    } else {
        false
    }
}
