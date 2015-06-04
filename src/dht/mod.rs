//! Interact with the bittorrent Distributed Hash Table.

use std::fmt::{self, Display, Formatter};
use std::io::{self, Error, ErrorKind};
use std::net::{SocketAddr, ToSocketAddrs};

const UTORRENT_DHT:     &'static str = "router.utorrent.com:6881";
const BITCOMET_DHT:     &'static str = "router.bitcomet.com:6881";
const TRANSMISSION_DHT: &'static str = "dht.transmissionbt.com:6881";

mod builder;
mod dht;

pub use self::builder::{DhtBuilder};
pub use self::dht::{Dht};

/// Enumerates different routers that can be used to bootstrap a dht.
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

impl Router {
    /// SocketAddr corresponding to the given Router variant.
    pub fn socket_addr(&self) -> io::Result<SocketAddr> {
        match *self {
            Router::uTorrent => {
                try!(UTORRENT_DHT.to_socket_addrs()).next().ok_or(
                    Error::new(ErrorKind::Other, "Could Not Parse uTorrent Router")
                )
            },
            Router::BitComet => {
                try!(BITCOMET_DHT.to_socket_addrs()).next().ok_or(
                    Error::new(ErrorKind::Other, "Could Not Parse BitComet Router")
                )
            },
            Router::Transmission => {
                try!(TRANSMISSION_DHT.to_socket_addrs()).next().ok_or(
                    Error::new(ErrorKind::Other, "Could Not Parse TransmissionBT Router")
                )
            },
            Router::Custom(n) => Ok(n)
        }
    }
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