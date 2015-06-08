use dht::{NodeId};
use dht::bucket::{Bucket};
use util::{ShaHash, Bits, Bit};

struct RoutingTable {
    buckets: Vec<Bucket>,
    node_id: NodeId
}

impl RoutingTable {
    
}

// Number of leading bits that are identical between the local node id and
// the remote node id.
fn leading_bit_count(local_node_id: NodeId, remote_node_id: NodeId) -> usize {
    let diff_id = local_node_id ^ remote_node_id;
    
    // An unset bit, 0, after a xor indicates the two bits were different.
    diff_id.bits().take_while(|x| x == Bit::Unset).count()
}

// Take the number of leading bits that are the same between our node
// and the remote node and calculate a bucket index for that node id.
//
// The value for num_buckets should be greater than 0.
fn bucket_placement(num_same_bits: usize, num_buckets: usize) -> usize {
    // The index that the node should be placed in *eventually*, meaning
    // when we create enough buckets for that bucket to appear.
    let ideal_index = num_same_bits * 2;
    
    if ideal_index > num_buckets {
        num_buckets - 1
    } else {
        ideal_index
    }
}