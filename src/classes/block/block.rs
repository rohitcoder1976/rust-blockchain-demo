mod block_header;
use block_header::BlockHeader;

use crate::classes::transaction::tx::Tx;

pub struct Block {
    block_header: BlockHeader,
    txs: Vec<Tx>,
}

impl Block {
    pub fn mine_block(&self) {
    }
}