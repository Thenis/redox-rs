use std::net::{SocketAddr};

use hash::{ShaHash};

// Since NodeId's behave idential to InfoHash's, we will use this as
// an alias to distinguish what type of id we are working with.
pub type NodeId = ShaHash;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum NodeStatus {
    Good,
    Questionable,
    Bad
}

/// Remote node participating in the dht.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Node {
    id:     NodeId,
    info:   SocketAddr,
    status: NodeStatus,
}

impl Node {
    /// Creates a new node with their status set to good.
    pub fn new(id: NodeId, info: SocketAddr) -> Node {
        Node{ id: id, info: info, status: NodeStatus::Good }
    }
}