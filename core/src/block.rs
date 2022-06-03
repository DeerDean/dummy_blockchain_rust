use chrono::prelude::*;

pub struct BlockHeader {
    pub time: i64,
    pub tx_hash: String,
    pub pre_hash: String,
}

pub struct Block {
    pub header: BlockHeader,
    pub hash: String,
    pub data: String,
}


impl Block {
    fn set_hash(&mut self) {
        self.header.time = Utc::now().timestamp();
    }
}