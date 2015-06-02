use std::fmt::{self, Display, Formatter};
use std::net::{SocketAddr};

use torrent::{Node};

const UTORRENT_DHT:     &'static str = "router.utorrent.com:6881";
const BITCOMET_DHT:     &'static str = "router.bitcomet.com:6881";
const TRANSMISSION_DHT: &'static str = "dht.transmissionbt.com:6881";

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Router {
    /// Bootstrap server under the uTorrent domain name.
    uTorrent,
    /// Bootstrap server under the BitComet domain name.
    BitComet,
    /// Bootstrap server under the TransmissionBT domain name.
    Transmission,
    /// Custom bootstrap server.
    Custom(SocketAddr)
}

impl Display for Router {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match *self {
            Router::uTorrent     => f.write_str(UTORRENT_DHT),
            Router::BitComet     => f.write_str(BITCOMET_DHT),
            Router::Transmission => f.write_str(TRANSMISSION_DHT),
            Router::Custom(n)    => Display::fmt(&n, f)
        }
    }
}

pub struct DhtBuilder;
    
impl DhtBuilder {
    /// Create a new DhtBuilder.
    pub fn new() -> DhtBuilder {
        DhtBuilder
    }

    /// Add nodes which will be pinged and distributed within our routing table.
    pub fn add_nodes<'a, I>(self, nodes: I) -> DhtBuilder
        where I: Iterator<Item=Node<'a>> {
        unimplemented!()
    }

    /// Add a router which will be used to gather nodes in case our routing table
    /// is ever empty.
    ///
    /// The key difference between a node and a router is that routers will only be
    /// used to gather nodes and will never be inserted into our routing table.
    pub fn add_router(self, router: Router) -> DhtBuilder {
        unimplemented!()
    }

    /// Set the read only flag when communicating with other nodes. This indicates that
    /// this dht node will not respond to pings or queries and so remote nodes should
    /// not add us to their routing table.
    ///
    /// This is used when we are behind a restrictive NAT and/or we want to decrease
    /// incoming network traffic.
    pub fn set_read_only(self, read_only: bool) -> DhtBuilder {
        unimplemented!()
    }
    
    /// Allows us to provide our dht with our external address that is supposedly
    /// from a trusted source. If this is not supplied we will have to deduce this
    /// information from remote nodes.
    ///
    /// The purpose of the external address is to choose a node id that conforms
    /// to the dht security extension so that other nodes can store information
    /// on our dht node.
    pub fn set_external_addr(self, addr: SocketAddr) -> DhtBuilder {
        unimplemented!()
    }
    
    /// Start the dht with the current configuration.
    pub fn start(self) -> Dht {
        unimplemented!()
    }
}

pub struct Dht;