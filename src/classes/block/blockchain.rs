use crate::classes::block::block::Block;

pub struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new(blocks: Vec<Block>) -> Blockchain {
        return Blockchain {
            blocks
        };
    }
}