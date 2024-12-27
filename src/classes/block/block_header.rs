use rand::Rng;
use sha2::{Sha256, Digest};
use hex;
use chrono::Utc;

pub struct BlockHeader {
    merkle_root: String,
    pub nonce: u128,
    pub target: u8,
    timestamp: i64,
}

impl BlockHeader {
    pub fn new(merkle_root: String) -> BlockHeader {
        let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
        return BlockHeader {
            merkle_root,
            nonce: rng.gen(),
            // TODO: Target is an arbitrary number right now. develop a mechanism to dynamically change it based on network congestion
            target: 4,
            timestamp: Utc::now().timestamp(),
        }
    }

    pub fn hash_block(&self) -> String {
        let mut bytes: Vec<u8> = vec![];
        for byte in self.merkle_root.as_bytes() {
            bytes.push(byte.clone());
        }
        for byte in self.nonce.to_be_bytes() {
            bytes.push(byte.clone());
        }
        bytes.push(self.target.clone());
        for byte in self.timestamp.to_be_bytes() {
            bytes.push(byte.clone());
        }

        let mut hasher = Sha256::new();
        hasher.update(bytes);
        let result = hasher.finalize();
        let hex_result = hex::encode(result);

        return hex_result;
    }
}