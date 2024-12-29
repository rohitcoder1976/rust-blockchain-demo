use serde::{Deserialize, Serialize};

use crate::classes::block::block_header:: BlockHeader;
use crate::classes::transaction::tx::Tx;
use crate::data_structures::merkle_tree::MerkleTree;

#[derive(Clone, Serialize, Deserialize)]
pub struct Block {
    pub block_header: BlockHeader,
    pub txs: MerkleTree,
}

impl Block {
    pub fn new(txs: &Vec<Tx>, prev_block_hash: String) -> Block {
        let tx_merkle_tree = MerkleTree::new(txs);
        return Block {
            block_header: BlockHeader::new(tx_merkle_tree.merkle_root.clone(), prev_block_hash),
            txs: tx_merkle_tree,
        };
    }
    
    pub fn mine_block(&mut self) {
        let target: u8 = self.block_header.target;
        let mut attempts: u128 = 0;
        loop {
            let mut leading_zeros: u8 = 0;
            let block_hash: String = self.block_header.hash_block(); // compute the hash of the block

            // check to see if the block hash has enough leading zeros
            for char in block_hash.chars() {
                if char == '0' {
                    leading_zeros += 1;
                } else {
                    break;
                }
            }

            // if so, break from the loop and announce the answer (TODO: add a block propagation to announce to other nodes)
            if leading_zeros >= target {
                break;
            }

            // update the nonce if still not mined. if at the max, reset back to zero
            if self.block_header.nonce == u128::MAX {
                self.block_header.nonce = 0;
            } else {
                self.block_header.nonce += 1;
            }

            attempts += 1;
        }

        println!("\n--- Mined the block! ---\nNonce: {0}\nBlock hash: {1}\nAttempts: {2}", self.block_header.nonce, self.block_header.hash_block(), attempts);
    }
}