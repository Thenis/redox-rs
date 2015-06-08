use dht::node::{Node};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Bucket {
    nodes:    Vec<Node>,
    max_size: usize
}

impl Bucket {
    pub fn new(max_size: usize) -> Bucket {
        Bucket{ nodes: Vec::new(, max_size: max_size }
    }
    
    pub fn with_capacity(max_size: usize) -> Bucket {
        Bucket{ nodes: Vec::with_capacity(capacity), max_size: max_size }
    }
    
    pub fn add_node(&mut self, node: Node) {
        
    }
}