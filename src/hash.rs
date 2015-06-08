use std::ops::{BitXor};

use util::multi_iter::{Iter};

/// Length in bytes of an info hash.
pub const SHA_HASH_LEN: usize = 20;

/// Sha1 hash that provides convenience methods for manipulating it.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ShaHash {
    hash: [u8; SHA_HASH_LEN]
}

impl ShaHash {
    /// Create a ShaHash from the given bytes.
    ///
    /// Returns None if the length of the bytes is not equal to SHA_HASH_LEN.
    pub fn from_bytes(bytes: &[u8]) -> Option<ShaHash> {
        let mut buffer = [0u8; SHA_HASH_LEN];
        
        if bytes.len() != SHA_HASH_LEN {
            None
        } else {
            for (src, dst) in bytes.iter().zip(buffer.iter_mut()) {
                *dst = *src;
            }
            
            Some(ShaHash{ hash: buffer })
        }
    }
    
    /// Iterator over the bits of the ShaHash starting from the most significant
    /// bit of the first byte.
    pub fn bits<'a>(&'a self) -> Bits<'a> {
        Bits::new(&self.hash[..])
    }
}

impl From<[u8; SHA_HASH_LEN]> for ShaHash {
    fn from(sha_hash: [u8; SHA_HASH_LEN]) -> ShaHash {
        ShaHash{ hash: sha_hash }
    }
}

impl BitXor<ShaHash> for ShaHash {
    type Output = ShaHash;
    
    fn bitxor(mut self, rhs: ShaHash) -> ShaHash {
        for (src, dst) in rhs.hash.iter().zip(self.hash.iter_mut()) {
            *dst = *src ^ *dst;
        }
        
        self
    }
}

/// Enumerates the result from comparing the xor result of two bits.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Bit {
    /// Indicates a bit that is set.
    Set,
    /// Indicates a bit that is not set.
    Unset
}

/// Iterator over the bits of a value starting at the most significant bit.
pub struct Bits<'a> {
    bytes:   &'a [u8],
    bit_pos: usize
}

impl<'a> Bits<'a> {
    fn new(bytes: &'a [u8]) -> Bits<'a> {
        Bits{ bytes: bytes, bit_pos: 0 }
    }
}

impl<'a> Iterator for Bits<'a> {
    type Item = Bit;
    
    fn next(&mut self) -> Option<Bit> {
        if self.bit_pos < self.bytes.len() * 8 {
            let byte_index = self.bit_pos / 8;
            let bit_offset = 7 - (self.bit_pos % 8);
            let bit_value = self.bytes[byte_index] >> bit_offset;
            
            self.bit_pos += 1;
            
            Some(bit_value).map(|x| if x == 1 { Bit::Set } else { Bit::Unset })
        } else {
            None
        }
    }
}