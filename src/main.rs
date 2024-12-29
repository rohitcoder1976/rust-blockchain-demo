mod classes;
mod util;
mod data_structures;

use classes::block::block::Block;
use classes::block::blockchain::Blockchain;
use classes::lamport_signature::key_pair::{KeyPair, KeyBlock, initialize_empty_key_blocks};
use classes::transaction::tx::{Tx, TxInput, TxOutput};
use rand::Rng;
use sha2::{Sha256, Digest};
use util::disk::load_branches_from_file;

fn main() {
    let new_key_pair1: KeyPair = KeyPair::new();
    
    let mut tx_inputs: Vec<TxInput> = Vec::new();
    let prev_tx_id = String::new();
    tx_inputs.push(TxInput::new(initialize_empty_key_blocks(), prev_tx_id, true));

    let mut tx_outputs: Vec<TxOutput> = Vec::new();
    tx_outputs.push(TxOutput::new(new_key_pair1.pub_key.clone(), 100));

    let mut new_tx = Tx::new(tx_inputs, tx_outputs);

    let signature: [KeyBlock; 256] = new_key_pair1.create_signature(&new_tx);
    new_tx.inputs[0].signature = signature;

    println!("Transaction is verified: {}", new_tx.verify_signature(&new_key_pair1.pub_key));
    let mut tx_vec: Vec<Tx> = vec![];
    tx_vec.push(new_tx);

    test_blockchain(tx_vec.clone());

    // let blockchains_loaded_result: Result<Vec<Blockchain>, ()> = load_branches_from_file();
    // let blockchains_loaded = match blockchains_loaded_result {
    //     Ok(val) => val,
    //     Err(()) => {
    //         vec![]
    //     }
    // };
    // let mut i:usize= 0; 
    // for blockchain_loaded in &blockchains_loaded {
    //     println!("Blockchain {0} height: {1}", i+1, blockchain_loaded.blocks.len());
    //     i += 1;
    // }
}

#[warn(dead_code)]
fn generate_test_vec(range: usize) -> Vec<[u8; 32]>{
    let mut vec: Vec<[u8; 32]> = Vec::new();
    for i in 0..range {
        let mut hasher = Sha256::new();
        let mut rng = rand::thread_rng();
        let random_num: u32 = rng.gen_range(1..1000);

        let random_num_bytes: [u8; 4] = random_num.to_be_bytes();
        hasher.update(random_num_bytes);
        let hash_result: [u8; 32] = hasher.finalize().into();
        vec.push(hash_result.clone());
    }
    return vec;
}

fn test_blockchain(tx_vec: Vec<Tx>){
    let mut blockchain = Blockchain::new();
    let mut block_1: Block = Block::new(&tx_vec, "".to_string());
    block_1.mine_block();
    let block_1_hash: String = block_1.block_header.hash_block();
    blockchain.accept_new_block(&block_1);

    let mut block_2: Block = Block::new(&tx_vec, block_1_hash.clone());
    block_2.mine_block();
    let block_2_hash: String = block_2.block_header.hash_block();
    blockchain.accept_new_block(&block_2);

    let mut block_3: Block = Block::new(&tx_vec, block_1_hash.clone());
    block_3.mine_block();
    let block_3_hash: String = block_3.block_header.hash_block();
    blockchain.accept_new_block(&block_3);

    let mut block_4: Block = Block::new(&tx_vec, block_3_hash.clone());
    block_4.mine_block();
    let block_4_hash: String = block_4.block_header.hash_block();
    blockchain.accept_new_block(&block_4);

    let mut block_5: Block = Block::new(&tx_vec, block_2_hash.clone());
    block_5.mine_block();
    let block_5_hash: String = block_5.block_header.hash_block();
    blockchain.accept_new_block(&block_5);

    let mut block_6: Block = Block::new(&tx_vec, block_4_hash.clone());
    block_6.mine_block();
    let block_6_hash: String = block_6.block_header.hash_block();
    blockchain.accept_new_block(&block_6);

    let mut block_7: Block = Block::new(&tx_vec, block_3_hash.clone());
    block_7.mine_block();
    let block_7_hash: String = block_7.block_header.hash_block();
    blockchain.accept_new_block(&block_7);

    let mut block_8: Block = Block::new(&tx_vec, block_7_hash.clone());
    block_8.mine_block();
    let block_8_hash: String = block_8.block_header.hash_block();
    blockchain.accept_new_block(&block_8);

    println!("\n--- Blockchain ---");
    for block in &blockchain.blocks {
        println!("Block hash: {}", block.block_header.hash_block());
    }

    println!("");

    println!("---- LOADED BLOCKCHAINS ----\n");
    let loaded_chains_result: Result<Vec<Blockchain>, ()> = load_branches_from_file();
    let loaded_chains = match loaded_chains_result {
        Ok(val) => val,
        Err(e) => vec![]
    };

    let mut i: usize = 0; 
    for loaded_chain in &loaded_chains {
        println!("-- Blockchain #{} --", i+1);
        for loaded_chain_block in &loaded_chain.blocks {
            println!("Block hash: {}", loaded_chain_block.block_header.hash_block());
        }
        println!("");
        i += 1;
    }
}