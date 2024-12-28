use std::collections::HashMap;

use crate::classes::{block::block::Block, transaction::tx::Tx};

pub struct Blockchain {
    pub blocks: Vec<Block>,
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

        let mut branches_block_hashes: Vec<Vec<String>> = vec![];
        
        let reversed_blocks: &Vec<Block> = &self.blocks.iter().rev().cloned().collect();
        let mut checked_blocks: HashMap<String, bool> = HashMap::new();

        /*
            Identify all branches an store them. 
            Every branch will have a unique final block in the chain.
            Check for those unique final blocks and trace back to the first block to identify the individual branches
            Store the already checked blocks so that you won't trace back in the same branch but in a shorter (in height of the branch) block
        */
        for block_in_chain in reversed_blocks {
            let mut branch_block_hashes: Vec<String> = vec![];
            let mut prev_branch_block_hash: &String = &block_in_chain.block_header.prev_block_hash;

            // if the current block has already been checked, it cannot be a unique final block of a branch, so skip to the next block
            if checked_blocks.contains_key(&block_in_chain.block_header.hash_block()) {
                continue;
            }

            branch_block_hashes.push(block_in_chain.block_header.hash_block());
            checked_blocks.insert(block_in_chain.block_header.hash_block(), true);

            // go back through the reversed blockchain, and trace through the blocks to construct the branch
            for block_in_chain2 in reversed_blocks {
                if prev_branch_block_hash == &block_in_chain2.block_header.hash_block() {
                    branch_block_hashes.push(block_in_chain2.block_header.hash_block());
                    checked_blocks.insert(block_in_chain2.block_header.hash_block(), true);
                    prev_branch_block_hash = &block_in_chain2.block_header.prev_block_hash;
                } 
            }

            // reverse the block hashes to be in chronological order once again
            branch_block_hashes.reverse();
            branches_block_hashes.push(branch_block_hashes);
        }

        println!(">> DEBUG:");

        if branches_block_hashes.len() > 1 {
            println!("LOG: Uh oh! Branch!! Number of branches: {}", branches_block_hashes.len());
            let mut i: usize = 0;
            let mut biggest_branch_block_hashes_index: usize = 0;
            let mut biggest_branch_block_hashes_num: usize = 0;

            for branch_block_hashes in &branches_block_hashes {
                if branch_block_hashes.len() > biggest_branch_block_hashes_num {
                    biggest_branch_block_hashes_num = branch_block_hashes.len();
                    biggest_branch_block_hashes_index = i;
                }
                i += 1;
            }

            println!("Biggest Branch In Blockchain:");
            for branch_block_hash in &branches_block_hashes[biggest_branch_block_hashes_index] {
                println!("Block hash: {}", branch_block_hash);
            }

        } else {
            println!("LOG: No branches in blockchain");
        }

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

        println!("UTXO Len: {}", self.utxo.len());
        self.last_checked_height = self.blocks.len() as u128;
    }
}