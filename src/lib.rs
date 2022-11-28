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
    leaves: Vec<Leaf>,
    keep: HashSet<Node>,
}

impl<'a, const MERKLE_TREE_HEIGHT: usize> MerkleTree<'a, MERKLE_TREE_HEIGHT> {
    pub fn new(leaves: Vec<Leaf>) -> Self {
        // A collection of nodes representing the current authentication path.
        let mut current_authentication_path: Vec<Node> = vec![];

        let stack = Stack::default();
        let stack = Arc::new(Mutex::new(stack));
        let tree_hash: TreeHash = TreeHash::new(stack, &leaves);

        let second_leaf = match leaves.get(1).take() {
            Some(leaf) => leaf,
            None => unreachable!(),
        };
        current_authentication_path.push(Node::from(second_leaf));

        for level in 1..MERKLE_TREE_HEIGHT {
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
                    current_authentication_path.push(node);
                }
                None => {
                    // TODO
                }
            }
        }

        // Retain holds a single right authentication node at height MERKLE_TREE_HEIGHT - 2.
        let mut retain = None;
        const POSITION: usize = 3; // Index position at height MERKLE_TREE_HEIGHT - 2.
        const TWO: usize = 2;
        let start = POSITION * TWO.pow(MERKLE_TREE_HEIGHT as u32 - 2);
        // Tree hash algorithm must be executed 2^height times.
        let end = start + TWO.pow(MERKLE_TREE_HEIGHT as u32 - 2);

        let stack = Stack::default();
        let stack = Arc::new(Mutex::new(stack));
        let tree_hash: TreeHash = TreeHash::new(stack, &leaves);

        for leaf_index in start..end {
            tree_hash.update(leaf_index, Level(MERKLE_TREE_HEIGHT - 2));
            // TODO Handle None case.
            retain = tree_hash.first();
        }

        let retain = match retain {
            Some(retain) => retain,
            None => unreachable!(),
        };

        Self {
            leaves,
            keep: HashSet::<Node>::new(),
            retain,
            treehashes: vec![],
            current_authentication_path,
        }
    }
    /// Update and output phase of merkle tree traversal.
    /// * `leaf` - The current leaf.
    /// Returns the current authentication path.
    fn update_and_output<'b>(&mut self, leaf: Leaf) -> &'b [Node] {
        let first_parent_left_node_height = Self::get_first_left_node_parent_height(&leaf);

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

    fn get_first_left_node_parent_height(leaf: &Leaf) -> usize {
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
    leaves: &'a [Leaf],
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
            Some(leaf) => leaf,
            None => unreachable!(),
        };
        let hashed_leaf = create_hash(leaf.print());

        let stack = Arc::clone(&self.stack);
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
    }

    fn hash<U>(&self, content: U) -> [u8; 32] {
        todo!();
    }
}

#[derive(Clone, Debug)]
pub struct Leaf {
    index: usize,
    height: usize,
    content: String,
}

impl Leaf {
    pub fn new(content: String, index: usize) -> Self {
        Self {
            index,
            height: 0,
            content,
        }
    }

    fn print(&self) -> &str {
        self.content.as_str()
    }
}

#[derive(Clone, Debug)]
struct Keep;

#[derive(Clone, Debug)]
struct Height(usize);

impl PartialEq<Height> for usize {
    fn eq(&self, other: &Height) -> bool {
        other.0 == *self
    }
}

#[derive(Clone, Debug)]
pub struct Level(pub usize);

///
///
/// * `leaf` -
/// * `leaves` -
/// * `authentication_path` -
///
fn calculate_root(mut index: usize, leaves: &[Leaf], authentication_path: &[Node]) -> String {
    let leaf = match leaves.get(index) {
        Some(leaf) => leaf.clone(),
        None => unreachable!(),
    };
    let mut node = Node::from(leaf);

    for neighbor in authentication_path.iter() {
        let mut prehash = String::new();

        if neighbor.height() == 0 {
            if index < neighbor.j() {
                prehash.push_str(&node.hash());
                prehash.push_str(&neighbor.hash());
            } else {
                prehash.push_str(&neighbor.hash());
                prehash.push_str(&node.hash());
            }
        } else {
            if node.j() < neighbor.j() {
                prehash.push_str(&node.hash());
                prehash.push_str(&neighbor.hash());
            } else {
                prehash.push_str(&neighbor.hash());
                prehash.push_str(&node.hash());
            }
        }
        index = (node.j().max(neighbor.j()) + 1) / 2 - 1;

        node = Node::new(create_hash(&prehash), index, Height(0));
    }

    node.hash
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

// TODO Delete this when retain is implemented
impl Default for Node {
    fn default() -> Self {
        Self {
            hash: String::from(""),
            height: Height(0),
            index: 42,
        }
    }
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

impl From<Leaf> for Node {
    fn from(leaf: Leaf) -> Self {
        let hash = create_hash(&leaf.content);

        Self {
            hash,
            height: Height(0),
            index: leaf.index,
        }
    }
}

impl From<&Leaf> for Node {
    fn from(leaf: &Leaf) -> Self {
        let hash = create_hash(&leaf.content);

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

#[cfg(test)]
mod merkle_tree_tests {
    use super::{Leaf, MerkleTree, Node};

    #[test]
    fn test_initial_authentication_path() {
        const MERKLE_TREE_HEIGHT: usize = 4;
        let number_of_leaves = MERKLE_TREE_HEIGHT.pow(2);
        let mut leaves = Vec::with_capacity(number_of_leaves);

        // Create leaves
        for index in 0..number_of_leaves {
            let content = String::from("Hello, world");
            leaves.push(Leaf::new(content, index));
        }

        let merkle_tree = MerkleTree::<MERKLE_TREE_HEIGHT>::new(leaves);

        let init_auth_path = merkle_tree
            .current_authentication_path
            .iter()
            .map(|node| (node.index, node.height.0))
            .collect::<Vec<(usize, usize)>>();

        assert_eq!(vec![(1, 0), (1, 1), (1, 2), (1, 3)], init_auth_path);
    }

    #[test]
    fn test_retain_node_value() {
        const MERKLE_TREE_HEIGHT: usize = 4;
        let number_of_leaves = MERKLE_TREE_HEIGHT.pow(2);
        let mut leaves = Vec::with_capacity(number_of_leaves);

        // Create leaves
        for index in 0..number_of_leaves {
            let content = String::from("Hello, world");
            leaves.push(Leaf::new(content, index));
        }

        let merkle_tree = MerkleTree::<MERKLE_TREE_HEIGHT>::new(leaves);

        assert_eq!(3, merkle_tree.retain.index);
        assert_eq!(2, merkle_tree.retain.height);
    }
}
