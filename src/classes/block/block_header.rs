use rand::Rng;
use serde::Deserialize;
use sha2::{Sha256, Digest};
use hex;
use chrono::Utc; 
use bincode;

#[derive(serde::Serialize, Deserialize, Clone)]
pub struct BlockHeader {
    pub prev_block_hash: String,
    merkle_root: String,
    pub nonce: u128,
    pub target: u8,
    pub timestamp: i64,
}

impl BlockHeader {
    pub fn new(merkle_root: String, prev_block_hash: String) -> BlockHeader {
        let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
        return BlockHeader {
            prev_block_hash,
            merkle_root,
            nonce: rng.gen(),
            // TODO: Target is an arbitrary number right now. develop a mechanism to dynamically change it based on network congestion
            target: 4,
            timestamp: Utc::now().timestamp(),
        }
    }

    pub fn hash_block(&self) -> String {
        let bytes: Vec<u8> = match bincode::serialize(self) {
            Ok(value) => value,
            Err(e) => {
                println!("Error! Could not hash block");
                vec![]
            }
        };

        let mut hasher = Sha256::new();
        hasher.update(bytes);
        let result = hasher.finalize();
        let hex_result: String = hex::encode(result);

        return hex_result;
    }
}