use std::fmt::{ self, Debug, Formatter };
use super::*;

pub struct Block {
    pub index: u32,
    pub timestamp: u128,  // unix time
    pub hash: BlockHash,
    pub prev_block_hash: BlockHash,
    pub nonce: u64,
    pub payload: String,  // later becomes transaction 
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f, "Block[{}]: {} at: {} with {}", 
            &self.index,
            &hex::encode(&self.hash),
            &self.timestamp,
            &self.payload
        )
    }
}

impl Block {
    pub fn new(index: u32, timestamp: u128, prev_block_hash: BlockHash, 
        nonce: u64, payload: String) -> Self {
            return Block {
                index, 
                timestamp, 
                hash: vec![0; 32], 
                prev_block_hash, 
                nonce, 
                payload
            }
    }
}