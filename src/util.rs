//! Utilities used throughout the library.

use std::borrow::{Borrow};
use std::collections::{HashMap, BTreeMap};
use std::hash::{Hash};
use std::net::{SocketAddr, Ipv4Addr, SocketAddrV4};
use std::thread::{self};
use std::sync::mpsc::{self, Receiver, Sender};

use rand;
use sha1::{Sha1};

pub const SHA1_HASH_LEN: usize = 20;

// Iterator over a reference to a single value.
pub mod single_iter {
    pub struct Iter<'a, T: 'a + ?Sized> {
        item: &'a T,
        yielded: bool
    }
    
    impl<'a, T: 'a + ?Sized> Iter<'a, T> {
        pub fn new(item: &'a T) -> Iter<'a, T> {
            Iter{ item: item, yielded: false }
        }
    }
    
    impl<'a, T: 'a + ?Sized> Iterator for Iter<'a, T> {
        type Item = &'a T;
        
        fn next(&mut self) -> Option<&'a T> {
            if !self.yielded {
                self.yielded = true;
                
                Some(self.item)
            } else {
                None
            }
        }
    }
}

// Iterator over a reference to many values.
pub mod multi_iter {
    pub struct Iter<'a, T: 'a> {
        items: &'a [T],
        index: usize
    }
    
    impl<'a, T: 'a> Iter<'a, T> {
        pub fn new(items: &'a [T]) -> Iter<'a, T> {
            Iter{ items: items, index: 0 }
        }
    }
    
    impl<'a, T: 'a> Iterator for Iter<'a, T> {
        type Item = &'a T;
        
        fn next(&mut self) -> Option<&'a T> {
            if self.index < self.items.len() {
                self.index += 1;
                
                Some(&self.items[self.index - 1])
            } else {
                None
            }
        }
    }
}

/// Trait for working with generic map data structures.
pub trait Dictionary<K, V> where K: Borrow<str> {
    /// Convert the dictionary to an unordered list of key/value pairs.
    fn to_list<'a>(&'a self) -> Vec<(&'a K, &'a V)>;

    /// Lookup a value in the dictionary.
    fn lookup<'a>(&'a self, key: &str) -> Option<&'a V>;
    
    /// Lookup a mutable value in the dictionary.
    fn lookup_mut<'a>(&'a mut self, key: &str) -> Option<&'a mut V>;

    /// Insert a key/value pair into the dictionary.
    fn insert(&mut self, key: K, value: V) -> Option<V>;
    
    /// Remove a value from the dictionary and return it.
    fn remove(&mut self, key: &str) -> Option<V>;
}

impl<K, V> Dictionary<K, V> for HashMap<K, V> where K: Hash + Eq + Borrow<str> {
    fn to_list<'a>(&'a self) -> Vec<(&'a K, &'a V)> {
        self.iter().collect()
    }

    fn lookup<'a>(&'a self, key: &str) -> Option<&'a V> {
        self.get(key)
    }
    
    fn lookup_mut<'a>(&'a mut self, key: &str) -> Option<&'a mut V> {
        self.get_mut(key)
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.insert(key, value)
    }
    
    fn remove(&mut self, key: &str) -> Option<V> {
        self.remove(key)
    }
}

impl<K, V> Dictionary<K, V> for BTreeMap<K, V> where K: Ord + Borrow<str> {
    fn to_list<'a>(&'a self) -> Vec<(&'a K, &'a V)> {
        self.iter().collect()
    }

    fn lookup<'a>(&'a self, key: &str) -> Option<&'a V> {
        self.get(key)
    }
    
    fn lookup_mut<'a>(&'a mut self, key: &str) -> Option<&'a mut V> {
        self.get_mut(key)
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.insert(key, value)
    }
    
    fn remove(&mut self, key: &str) -> Option<V> {
        self.remove(key)
    }
}

/// Get the default route IPv4 socket.
pub fn default_route_v4() -> SocketAddr {
    let v4_addr = Ipv4Addr::new(0, 0, 0, 0);
    let v4_sock = SocketAddrV4::new(v4_addr, 0);
    
    SocketAddr::V4(v4_sock)
}

/// Spawn two threads linked together by a channel.
pub fn spawn_threads<S, R, T>(mut send_thread: S, mut recv_thread: R)
    where S: FnMut(Sender<T>) + Send + 'static,
          R: FnMut(Receiver<T>) + Send + 'static, T: Send + 'static {
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || send_thread(tx));
    thread::spawn(move || recv_thread(rx));
}

/// Applies a sha1 hash to the src and outputs it in dst.
pub fn apply_sha1(src: &[u8], dst: &mut [u8]) {
    let mut sha = Sha1::new();
    
    sha.update(src);
    
    sha.output(dst);
}
/// Applies a Fisher-Yates shuffle on the given list.
pub fn fisher_shuffle<T: Copy>(list: &mut [T]) {
    for i in 0..list.len() {
        let swap_index = (rand::random::<usize>() % (list.len() - i)) + i;
        
        let temp = list[i];
        list[i] = list[swap_index];
        list[swap_index] = temp;
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn positive_fisher_shuffle() {
        let mut test_slice = [1, 2, 3, 4];
        
        super::fisher_shuffle(&mut test_slice);
        
        assert!(test_slice.contains(&1));
        assert!(test_slice.contains(&2));
        assert!(test_slice.contains(&3));
        assert!(test_slice.contains(&4));
    }
}