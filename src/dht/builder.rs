use std::collections::{HashSet};
use std::net::{SocketAddr};

use dht::{Dht, Router};
use error::{DhtResult};
use torrent::{Node};
use util::{self};

/// Stores information for initializing a dht.
pub struct DhtBuilder {
    nodes:     HashSet<SocketAddr>,
    routers:   HashSet<Router>,
    read_only: bool,
    src_addr:  SocketAddr,
    ext_addr:  Option<SocketAddr>
}
    
impl DhtBuilder {
    /// Create a new DhtBuilder.
    ///
    /// This should not be used directly, force the user to supply builder with
    /// some initial bootstrap method.
    fn new() -> DhtBuilder {
        DhtBuilder{ nodes: HashSet::new(),
            routers: HashSet::new(),
            read_only: false,
            src_addr: util::default_route_v4(),
            ext_addr: None
        }
    }

    /// Creates a DhtBuilder with initial nodes which will be distributed within]
    /// our routing table.
    pub fn with_nodes<I>(nodes: I) -> DhtBuilder
        where I: Iterator<Item=SocketAddr> {
        let mut dht = DhtBuilder::new();
        
        dht.add_nodes(nodes)
    }
    
    /// Creates a DhtBuilder with an initial router which will let us gather nodes
    /// if our routing table is ever empty.
    ///
    /// Difference between a node and a router is that a router is never put in
    /// our routing table.
    pub fn with_router(router: Router) -> DhtBuilder {
        let mut dht = DhtBuilder::new();
        
        dht.add_router(router)
    }

    /// Add nodes which will be distributed within our routing table.
    pub fn add_nodes<I>(mut self, nodes: I) -> DhtBuilder
        where I: Iterator<Item=SocketAddr> {
        for node in nodes {
            self.nodes.insert(node);
        }
        
        self
    }

    /// Add a router which will let us gather nodes if our routing table is ever empty.
    ///
    /// Difference between a node and a router is that a router is never put in
    /// our routing table.
    pub fn add_router(mut self, router: Router) -> DhtBuilder {
        self.routers.insert(router);
        
        self
    }

    /// Set the read only flag when communicating with other nodes. Indicates
    /// that remote nodes should not add us to their routing table.
    ///
    /// Used when we are behind a restrictive NAT and/or we want to decrease
    /// incoming network traffic.
    pub fn set_read_only(mut self, read_only: bool) -> DhtBuilder {
        self.read_only = read_only;
        
        self
    }
    
    /// Provide the dht with our external address. If this is not supplied we will
    /// have to deduce this information from remote nodes.
    ///
    /// Purpose of the external address is to generate a node id the conforms to
    /// BEP 42 so that nodes can safely store information on our node.
    pub fn set_external_addr(mut self, addr: SocketAddr) -> DhtBuilder {
        self.ext_addr = Some(addr);
        
        self
    }
    
    /// Provide the dht with the source address. If this is not supplied we will
    /// use the OS default route.
    pub fn set_source_addr(mut self, addr: SocketAddr) -> DhtBuilder {
        self.src_addr = addr;
    
        self
    }
    
    /// Start the dht with the current configuration.
    pub fn start(mut self) -> DhtResult<Dht> {
        Dht::with_builder(self)
    }
}