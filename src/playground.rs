
#[warn(dead_code)]
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