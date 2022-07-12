use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct BlockHeader {
    timestamp: i64, 
    prev_hash: String,  // hash of prev block
    nounce: usize,  // for PoW
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Block {
    header: BlockHeader,
    data: String,   // block data
    hash: String,   // hash of this block
}




