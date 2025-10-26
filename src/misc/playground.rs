use crate::{classes::{block::{block::Block, blockchain::Blockchain}, lamport_signature::key_pair::{initialize_empty_key_blocks, Key}, transaction::tx::{Tx, TxInput, TxOutput}}, util::disk::load_branches_from_file};

pub fn test_blockchain_fork_detection(pub_key: &Key){
    let tx_inputs: Vec<TxInput> = vec![TxInput::new(initialize_empty_key_blocks(), "".to_string(), true, 0)];
    let tx_outputs: Vec<TxOutput> = vec![TxOutput::new(pub_key.clone(), 100)];
    let tx: Tx = Tx::new(tx_inputs, tx_outputs);
    let tx_vec = vec![tx];

    let mut blockchain = Blockchain::new();
    blockchain.load_genesis_block(&pub_key, &"testbranches.bin".to_string());

    let block_1: Block = blockchain.blocks[0].clone();
    let block_1_hash = block_1.block_header.hash_block();

    let mut block_2: Block = Block::new(&tx_vec, block_1_hash.clone());
    block_2.mine_block();
    let block_2_hash: String = block_2.block_header.hash_block();
    blockchain.accept_new_block(&block_2, &"testbranches.bin".to_string());

    let mut block_3: Block = Block::new(&tx_vec, block_1_hash.clone());
    block_3.mine_block();
    let block_3_hash: String = block_3.block_header.hash_block();
    blockchain.accept_new_block(&block_3, &"testbranches.bin".to_string());

    let mut block_4: Block = Block::new(&tx_vec, block_3_hash.clone());
    block_4.mine_block();
    let block_4_hash: String = block_4.block_header.hash_block();
    blockchain.accept_new_block(&block_4, &"testbranches.bin".to_string());

    let mut block_5: Block = Block::new(&tx_vec, block_2_hash.clone());
    block_5.mine_block();
    let block_5_hash: String = block_5.block_header.hash_block();
    blockchain.accept_new_block(&block_5, &"testbranches.bin".to_string());

    let mut block_6: Block = Block::new(&tx_vec, block_4_hash.clone());
    block_6.mine_block();
    let block_6_hash: String = block_6.block_header.hash_block();
    blockchain.accept_new_block(&block_6, &"testbranches.bin".to_string());

    let mut block_7: Block = Block::new(&tx_vec, block_3_hash.clone());
    block_7.mine_block();
    let block_7_hash: String = block_7.block_header.hash_block();
    blockchain.accept_new_block(&block_7, &"testbranches.bin".to_string());

    let mut block_8: Block = Block::new(&tx_vec, block_7_hash.clone());
    block_8.mine_block();
    let block_8_hash: String = block_8.block_header.hash_block();
    blockchain.accept_new_block(&block_8, &"testbranches.bin".to_string());

    println!("\n--- Blockchain ---");
    for block in &blockchain.blocks {
        println!("Block hash: {}", block.block_header.hash_block());
    }

    println!("");

    println!("---- LOADED BLOCKCHAINS ----\n");
    let loaded_chains_result: Result<Vec<Blockchain>, ()> = load_branches_from_file(&"testbranches.bin".to_string());
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