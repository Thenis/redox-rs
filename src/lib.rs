//! # Rust Bittorrent Library

#![feature(collections)]

extern crate rand;
extern crate sha1;

pub mod bencode;
pub mod dht;
pub mod error;
pub mod torrent;

mod hash;
mod util;

pub use self::hash::{ShaHash, SHA_HASH_LEN, Bits, BitCmp};