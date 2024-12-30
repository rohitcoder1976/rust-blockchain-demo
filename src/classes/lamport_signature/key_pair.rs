use rand::Rng;
use sha2::{Digest, Sha256};
use std::{array, vec};

use crate::Tx;
use crate::util::conversions::hex_string_to_bit_vector;
use bincode;
use serde::{Serialize, Deserialize};
use serde_big_array::big_array;

big_array! { BigArray; 256 }

#[derive(Serialize, Deserialize, Clone)]
pub struct KeyPair {
    pub priv_key: Key,
    pub pub_key: Key
}   

impl KeyPair {
    pub fn new() -> Self { 
        let mut rng = rand::thread_rng();

        let mut priv_key_zero_blocks: [KeyBlock; 256] = initialize_empty_key_blocks();
        let mut priv_key_one_blocks: [KeyBlock; 256] = initialize_empty_key_blocks();

        let mut pub_key_zero_blocks: [KeyBlock; 256] = initialize_empty_key_blocks();
        let mut pub_key_one_blocks: [KeyBlock; 256] = initialize_empty_key_blocks();

        for i in 0..256 {
            /* generate block with random u128 values for privKey zero part, and hash them to create a zero block for the public key */
            priv_key_zero_blocks[i] = KeyBlock {
                first_part: rng.gen(),
                second_part: rng.gen()
            };  

            pub_key_zero_blocks[i] = priv_key_zero_blocks[i].hash_priv_key_block();

            /* repeat the same process as above but for the one part */
            priv_key_one_blocks[i] = KeyBlock {
                first_part: rng.gen(),
                second_part: rng.gen()
            };

            pub_key_one_blocks[i] = priv_key_one_blocks[i].hash_priv_key_block();
        }

        return KeyPair {
            priv_key: Key {
                zero_blocks: priv_key_zero_blocks,
                one_blocks: priv_key_one_blocks,
                is_private: true,
            },
            pub_key: Key {
                zero_blocks: pub_key_zero_blocks,
                one_blocks: pub_key_one_blocks,
                is_private: false
            }
        };
    
    }

    pub fn create_signature(&self, tx: &Tx) -> [KeyBlock; 256]{
        // hash the message
        let msg_hash_bits: Vec<u8> = hex_string_to_bit_vector(tx.get_tx_hash());

        let mut signature_priv_blocks: [KeyBlock; 256] = initialize_empty_key_blocks();

        // assign blocks to the signature blocks, zero or one blocks based on each bit of the hashed message
        let mut i: usize = 0; 
        for bit in msg_hash_bits {
            if bit == 0 {
                signature_priv_blocks[i] = self.priv_key.zero_blocks[i].clone();
            } else {
                signature_priv_blocks[i] = self.priv_key.one_blocks[i].clone();
            }
            i += 1;
        }

        // return the signature blocks and the public key
        return signature_priv_blocks;
    }

}

#[derive(Clone, Serialize, Deserialize)]
pub struct Key {
    #[serde(with = "BigArray")]
    pub zero_blocks: [KeyBlock; 256],
    #[serde(with = "BigArray")]
    pub one_blocks: [KeyBlock; 256],  
    pub is_private: bool,
}

impl Key {
    pub fn convert_key_to_bytes(&self) -> Vec<u8> {
        let bytes: Vec<u8> = match bincode::serialize(self) {
            Ok(val) => val,
            Err(e) => {
                println!("Error! Could not convert key to bytes");
                vec![]
            }
        };
        
        return bytes;
    }

    pub fn hash_key(&self) -> String {
        let bytes: Vec<u8> = match bincode::serialize(self) {
            Ok(val) => val,
            Err(e) => {
                println!("Error! Could not convert key to bytes");
                vec![]
            }
        };

        let mut hasher = Sha256::new();
        hasher.update(&bytes);
        let result = hasher.finalize();
        let hex_string: String = hex::encode(result);
        hex_string
    }
}

/* Each key block, in order to meet the 256 bits length requirement, must be two u128 integers stuck together, rather than a simple primitive type. */
#[derive(Clone, Serialize, Deserialize, Copy)]
pub struct KeyBlock {
    pub first_part: u128,
    pub second_part: u128,
}

impl KeyBlock {
    pub fn hash_priv_key_block(&self) -> KeyBlock{
        let block_bytes: Vec<u8> = [self.first_part.to_be_bytes(), self.second_part.to_be_bytes()].concat();
        let mut hasher = Sha256::new();
        hasher.update(&block_bytes);
        let block_hashed = hasher.finalize();
        let block_hashed_part1: u128 = u128::from_be_bytes(block_hashed[0..16].try_into().expect("slice with incorrect length"));
        let block_hashed_part2: u128 = u128::from_be_bytes(block_hashed[16..32].try_into().expect("slice with incorrect length"));
        return KeyBlock {
            first_part: block_hashed_part1,
            second_part: block_hashed_part2
        };
    }
}

// necessary for serde deserialize and serialize derivations
impl Default for KeyBlock {
    fn default() -> Self {
        Self {
            first_part: 0,
            second_part: 0
        }
    }
}

pub fn initialize_empty_key_blocks() -> [KeyBlock; 256] {
    return array::from_fn(|_| KeyBlock {
        first_part: 0,
        second_part: 0,
    });
}