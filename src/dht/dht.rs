use dht::{DhtBuilder};
use error::{DhtResult, DhtError};

/// Maintains a distributed routing table.
pub struct Dht {
    table: Arc<RwLock<RoutingTable>>,
    
}

impl Dht {
    pub fn with_builder(builder: DhtBuilder) -> DhtResult<Dht> {
        Dht
    }
}