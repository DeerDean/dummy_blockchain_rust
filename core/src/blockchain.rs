use crate::block;

pub struct BlockChain {
    pub blocks: Vec<block::Block>,
}

impl BlockChain {
    fn genesis_block() -> block::Block {
        block::Block::new("This is genesis block".to_string(), "".to_string())
    }

    pub fn new() -> BlockChain {
        BlockChain { blocks: vec![BlockChain::genesis_block()] }
    }

    pub fn add_block(&mut self, data: String) {
        let pre_hash = self.blocks[self.blocks.len() - 1].hash.clone();
        let new_block = block::Block::new(data, pre_hash);
        self.blocks.push(new_block);
    }
}
