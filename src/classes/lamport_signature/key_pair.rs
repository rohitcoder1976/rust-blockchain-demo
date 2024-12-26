use rand::Rng;
use sha2::{Digest, Sha256};
use std::array;

pub struct KeyPair {
    pub priv_key: Key,
    pub pub_key: Key
}   

impl KeyPair {
    pub fn new() -> Self { 
        let mut rng = rand::thread_rng();

        let mut priv_key_zero_blocks: [KeyBlock; 256] = initialize_empty_blocks();
        let mut priv_key_one_blocks: [KeyBlock; 256] = initialize_empty_blocks();

        let mut pub_key_zero_blocks: [KeyBlock; 256] = initialize_empty_blocks();
        let mut pub_key_one_blocks: [KeyBlock; 256] = initialize_empty_blocks();

        for i in 0..256 {
            /* generate block with random u128 values for privKey zero part, and hash them to create a zero block for the public key */
            priv_key_zero_blocks[i] = KeyBlock {
                first_part: rng.gen(),
                second_part: rng.gen()
            };  

            pub_key_zero_blocks[i] = hash_priv_key_block(&priv_key_zero_blocks[i]);

            /* repeat the same process as above but for the one part */
            priv_key_one_blocks[i] = KeyBlock {
                first_part: rng.gen(),
                second_part: rng.gen()
            };

            pub_key_one_blocks[i] = hash_priv_key_block(&priv_key_one_blocks[i]);

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

}

pub struct Key {
    pub zero_blocks: [KeyBlock; 256],
    pub one_blocks: [KeyBlock; 256],  
    pub is_private: bool,
}

/* Each key block, in order to meet the 256 bits length requirement, must be two u128 integers stuck together, rather than a simple primitive type. */
pub struct KeyBlock {
    pub first_part: u128,
    pub second_part: u128,
}

fn hash_priv_key_block(block: &KeyBlock) -> KeyBlock{
    let block_bytes: Vec<u8> = [block.first_part.to_be_bytes(), block.second_part.to_be_bytes()].concat();
    let mut hasher = Sha256::new();
    hasher.update(&block_bytes);
    let block_hashed = hasher.finalize();
    let block_hashed_part1 = u128::from_be_bytes(block_hashed[0..16].try_into().expect("slice with incorrect length"));
    let block_hashed_part2: u128 = u128::from_be_bytes(block_hashed[16..32].try_into().expect("slice with incorrect length"));
    return KeyBlock {
        first_part: block_hashed_part1,
        second_part: block_hashed_part2
    };
}

fn initialize_empty_blocks() -> [KeyBlock; 256] {
    return array::from_fn(|_| KeyBlock {
        first_part: 0,
        second_part: 0,
    });
}