use crate::classes::{block::block::Block, transaction::tx::Tx};

pub struct Blockchain {
    blocks: Vec<Block>,
    last_checked_height: u128,
    pub utxo: Vec<Tx>
}

impl Blockchain {
    pub fn new() -> Blockchain {
        return Blockchain {
            blocks: vec![],
            last_checked_height: 0,
            utxo: vec![]
        };
    }

    pub fn add_new_block(&mut self, block: &Block) {
        self.blocks.push(block.clone());
        self.update_utxo();
    }

    fn update_utxo(&mut self){
        let new_block_txs: &Vec<Tx> = &self.blocks[self.blocks.len() - 1].txs.base;

        // go through each transaction in the new block
        for new_tx in new_block_txs {
            // if no previous transaction to point to (coinbase transaction)
            if new_tx.inputs[0].is_coinbase { 
                self.utxo.push(new_tx.clone());
                continue;
            }

            // iterate through each input for every transaction in the block
            for new_tx_input in &new_tx.inputs {
                let prev_tx_id: String = new_tx_input.prev_tx_id.clone();
                let prev_tx_index: usize = new_tx_input.index;

                let mut k: usize = 0; 
                // iterate through the utxo to find the prev_tx_id that the new transaction input points to
                for utxo_tx_index in 0..self.utxo.len() {
                    let utxo_tx_id: String = self.utxo[utxo_tx_index].get_tx_id();
                    if prev_tx_id == utxo_tx_id { // if found the match, remove the output from the consumed utxo transaction
                        self.utxo[utxo_tx_index].outputs.remove(prev_tx_index);

                        // if there are no outputs left in the consumed utxo transaction, delete the utxo transaction
                        if self.utxo[utxo_tx_index].outputs.len() == 0 {
                            self.utxo.remove(k);
                        }
                    } else {
                        print!("Error! Not found a matching input for a new transaction.");
                        return;
                    }
                    k += 1;
                }   
            }

            self.utxo.push(new_tx.clone());
        }

        self.last_checked_height = self.blocks.len() as u128;
    }
}