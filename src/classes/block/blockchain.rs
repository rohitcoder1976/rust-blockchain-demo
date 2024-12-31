use std::{collections::HashMap, vec};
use serde::{Deserialize, Serialize};
use crate::{classes::{block::block::Block, lamport_signature::key_pair::{initialize_empty_key_blocks, Key}, transaction::tx::{Tx, TxInput, TxOutput}}, data_structures::merkle_tree::MerkleTree, util::disk::{load_branches_from_file, save_chain_branches_to_file}};

use super::block_header::BlockHeader;

#[derive(Serialize, Deserialize, Clone)]
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

    pub fn accept_new_block(&mut self, block: &Block) {
        let prev_block_hash = &block.block_header.prev_block_hash;

        if self.blocks.len() == 0 { // if genesis block
            self.blocks.push(block.clone());
            match save_chain_branches_to_file(&vec![self.clone()]) {
                Ok(()) => {
                    println!("Saved branches to disk...");
                },
                Err(()) => {}
            };
            return;
        }

        let mut found_prev_block: bool = false;

        // try to find the new block in the valid chain
        for block_in_chain in &self.blocks {
            if &block_in_chain.block_header.hash_block() == prev_block_hash {
                found_prev_block = true;
            }
        }

        // if not in the valid chain, check in the chains stored in disk
        if !found_prev_block {
            let loaded_branch_chains_result: Result<Vec<Blockchain>, ()> = load_branches_from_file();
            let mut loaded_branch_chains: Vec<Blockchain> = match loaded_branch_chains_result {
                Ok(val) => val,
                Err(()) => vec![],
            };

            if loaded_branch_chains.len() == 0 {
                println!("Could not load branch chains from disk...");
                return;
            }

            let mut branch_index: usize = 0;
            for branch_chain in loaded_branch_chains.clone() {
                for block_in_branch in &branch_chain.blocks {
                    if &block_in_branch.block_header.hash_block() == prev_block_hash {
                        found_prev_block = true;
                        loaded_branch_chains[branch_index].blocks.push(block_in_branch.clone());
                    }
                }
                branch_index += 1;
            }

            // if still not found, the block is invalid as it doesn't point to anything
            if !found_prev_block {
                println!("New block does not point to any existing block...");
                return;
            }
        }

        let block_verified: bool = true;

        /* if the new block is verified within a disk stored branch, then push all of the blocks in all chains, except for the last one in the branch
            that the new block exists in, of the branch to the valid chain manually. then call add_new_block to add the last block, while also computing the branches within the chain and picking the new valid chain */ 
        if block_verified {
            self.insert_disk_blocks();
            self.blocks.push(block.clone());
            self.choose_valid_chain_and_update_utxo();
        }

    }

    pub fn choose_valid_chain_and_update_utxo(&mut self) {
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

        // println!(">> DEBUG:");

        if branches_block_hashes.len() > 1 {
            // println!("LOG: Uh oh! Branch!! Number of branches: {}", branches_block_hashes.len());
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

            let mut branches: Vec<Blockchain> = vec![];
            for branch_block_hashes in &branches_block_hashes {
                let mut branch: Blockchain = Blockchain::new();
                for branch_block_hash in branch_block_hashes {
                    for block in &self.blocks {
                        if branch_block_hash == &block.block_header.hash_block() {
                            branch.blocks.push(block.clone());
                        }
                    }
                }
                branches.push(branch);
            }

            match save_chain_branches_to_file(&branches) {
                Ok(()) => {
                    println!("Saved branches to disk...");
                },
                Err(()) => {}
            };

            self.blocks = branches[biggest_branch_block_hashes_index].blocks.clone();
        } else {
            match save_chain_branches_to_file(&vec![self.clone()]) {
                Ok(()) => {
                    println!("Saved branches to disk...");
                },
                Err(()) => {}
            };
        }

        self.update_utxo();
    }

    pub fn update_utxo(&mut self){
        let mut new_utxo: Vec<Tx> = vec![];

        for block in &self.blocks {
            // go through each transaction in the new block
            for tx in &block.txs.base {
                // if no previous transaction to point to (coinbase transaction)
                if tx.inputs[0].is_coinbase { 
                    new_utxo.push(tx.clone());
                    continue;
                }

                // iterate through each input for every transaction in the block
                for new_tx_input in &tx.inputs {
                    let prev_tx_id: String = new_tx_input.prev_tx_id.clone();
                    let prev_tx_index: usize = new_tx_input.index;

                    let mut utxo_tx_index: usize = 0;
                    let mut found_matching_output= false;
                    for utxo_tx in new_utxo.clone() {
                        let utxo_tx_id: String = utxo_tx.get_tx_id();
                        if prev_tx_id == utxo_tx_id { // if found the match, remove the output from the consumed utxo transaction
                            found_matching_output = true;
                            new_utxo[utxo_tx_index].outputs.remove(prev_tx_index);

                            // if there are no outputs left in the consumed utxo transaction, delete the utxo transaction
                            if new_utxo[utxo_tx_index].outputs.len() == 0 {
                                new_utxo.remove(utxo_tx_index.clone());
                                utxo_tx_index += 1;
                            }
                        } 
                        utxo_tx_index += 1;
                    }

                    if !found_matching_output {
                        println!("Could not find the matching output for a new transaction.");
                        return;
                    }
                }

                new_utxo.push(tx.clone());
            }
        }

        self.utxo = new_utxo;
    }

    pub fn load_genesis_block(&mut self, pub_key: &Key){
        let tx_inputs: Vec<TxInput> = vec![TxInput::new(initialize_empty_key_blocks(), "".to_string(), true, 0)];
        let tx_outputs: Vec<TxOutput> = vec![TxOutput::new(pub_key.clone(), 100)];
        let tx: Tx = Tx::new(tx_inputs, tx_outputs);
        let tx_merkle_tree: MerkleTree = MerkleTree::new(&vec![tx]);

        let block: Block = Block {
            block_header: BlockHeader {
                prev_block_hash: "".to_string(),
                target: 4,
                merkle_root: tx_merkle_tree.merkle_root.clone(),
                nonce: 56974,
                timestamp: 1735577085,
            },
            txs: tx_merkle_tree
        };

        self.blocks.push(block);

        match save_chain_branches_to_file(&vec![self.clone()]) {
            Ok(()) => {},
            Err(()) => {println!("Could not save genesis block to disk...")}
        };

        self.update_utxo();
    }

    fn insert_disk_blocks(&mut self) {
        let loaded_branch_chains_result: Result<Vec<Blockchain>, ()> = load_branches_from_file();
        let loaded_branch_chains: Vec<Blockchain> = match loaded_branch_chains_result {
            Ok(val) => val,
            Err(()) => vec![],
        };

        if loaded_branch_chains.len() == 0 {
            return;
        }

        let mut checked_block_hashes: HashMap<String, bool> = HashMap::new();
        for block_in_chain in &self.blocks {
            checked_block_hashes.insert(block_in_chain.block_header.hash_block(), true);
        }
        for branch_chain in &loaded_branch_chains {
            for block_in_branch_chain in &branch_chain.blocks {
                if checked_block_hashes.contains_key(&block_in_branch_chain.block_header.hash_block()) {
                    continue;
                } else {
                    checked_block_hashes.insert(block_in_branch_chain.block_header.hash_block(), true);
                    self.blocks.push(block_in_branch_chain.clone());
                }
            }
        }
    }
}