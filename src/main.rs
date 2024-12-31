mod classes;
mod util;
mod data_structures;

use std::io;

use chrono::{NaiveDateTime, TimeZone, Utc};
use classes::block::block::Block;
use classes::block::blockchain::Blockchain;
use classes::lamport_signature::key_pair::{KeyPair, initialize_empty_key_blocks};
use classes::transaction::tx::{Tx, TxInput, TxOutput};

use util::disk::{load_branches_from_file, load_keypairs_from_file};

fn main() {

    let keypairs_result: Result<Vec<KeyPair>, ()> = load_keypairs_from_file();
    let keypairs: Vec<KeyPair> = match keypairs_result {
        Ok(val) => {
            println!("Imported Key Pairs...");
            val
        },
        Err(e) => {
            panic!("Failed to import key pairs...");
        } 
    };

    let blockchain_loaded_result: Result<Vec<Blockchain>, ()> = load_branches_from_file();
    let blockchains: Vec<Blockchain> = match blockchain_loaded_result {
        Ok(val) => val,
        Err(()) => {
            println!("Could not load initial blockchain...");
            vec![]
        }
    };

    let mut blockchain: Blockchain = Blockchain::new();
    if blockchains.len() == 0 { // load genesis block
        blockchain.load_genesis_block(&keypairs[0].pub_key);
    } else {
        let mut biggest_chain_height: usize = 0;
        let mut biggest_chain_index: usize = 0;
        for chain_index in 0..blockchains.len() {
            if blockchains[chain_index].blocks.len() > biggest_chain_height {
                biggest_chain_height = blockchains[chain_index].blocks.len();
                biggest_chain_index = chain_index;
            }
        }
    
        blockchain = blockchains[biggest_chain_index].clone();
        blockchain.update_utxo();
    }

    loop {
        let mut choice: String = String::new();
        println!("What can I do for you?\n1. Get Blockchain\n2. Compute Balance\n3. Send Money\n4. Get UTXO\n(Q to Exit)");
        io::stdin().read_line(&mut choice).expect("Failed to read line...");
        choice = choice.trim().to_string();
    
        if choice == "1".to_string() {
            get_blockchain(&blockchain);
        } else if choice == "2".to_string() {
            compute_balance(&blockchain, &keypairs);
        } else if choice == "3".to_string() {
            send_money(&mut blockchain, &keypairs);
        } else if choice == "4".to_string() {
            get_utxo(&blockchain, &keypairs);
        } else if choice == "Q".to_string() || choice == "q".to_string() {
            break;
        }
    }

}

fn compute_balance(blockchain: &Blockchain, keypairs: &Vec<KeyPair>) {
    let utxo: &Vec<Tx> = &blockchain.utxo;
    let mut account_index_str: String = String::new();
    println!("\nAccount Index: ");
    io::stdin().read_line(&mut account_index_str).expect("Failed to read line...");
    let account_index: usize = account_index_str.trim().parse().unwrap();
    let mut computed_balance: u64 = 0;

    let keypair: &KeyPair = &keypairs[account_index];
    for tx in utxo {
        for tx_output in &tx.outputs {
            if tx_output.pub_key.hash_key() == keypair.pub_key.hash_key() {
                computed_balance += tx_output.amount.clone();
            }
        }
    }

    println!("Balance: ${}\n", computed_balance);
}

fn get_blockchain(blockchain: &Blockchain) {
    let mut block_height: usize = 0;
    println!("");
    for block_in_chain in &blockchain.blocks {
        println!("-Block Height: {}-", block_height);
        println!("Block hash: {}", block_in_chain.block_header.hash_block());
        println!("TX Length: {}", block_in_chain.txs.base.len());
        let naive_datetime = NaiveDateTime::from_timestamp(block_in_chain.block_header.timestamp.clone(), 0);
        println!("Timestamp: {}", Utc.from_utc_datetime(&naive_datetime).to_rfc3339());
        println!("");
        block_height += 1;
    }
}

fn send_money(blockchain: &mut Blockchain, keypairs: &Vec<KeyPair>) {
    let utxo: &Vec<Tx> = &blockchain.utxo;
    let mut sender_account_index_str: String = String::new();
    let mut recipient_account_index_str: String = String::new();
    let mut amount_str: String = String::new();

    println!("\nSender Account Index: ");
    io::stdin().read_line(&mut sender_account_index_str).expect("Failed to read line");
    println!("\nRecipient Account Index: ");
    io::stdin().read_line(&mut recipient_account_index_str).expect("Failed to read line");
    println!("\nAmount of Money: ");
    io::stdin().read_line(&mut amount_str).expect("Failed to read line");

    let sender_account_index: usize = sender_account_index_str.trim().parse().unwrap();
    let recipient_account_index: usize = recipient_account_index_str.trim().parse().unwrap();
    let amount: u64 = amount_str.trim().parse().unwrap();

    let mut possible_tx_inputs: Vec<TxInput> = vec![];
    let mut possible_amount: u64 = 0;

    let keypair: &KeyPair = &keypairs[sender_account_index];

    for tx in utxo {
        let mut output_index: usize = 0; 
        for tx_output in &tx.outputs {
            if tx_output.pub_key.hash_key() == keypair.pub_key.hash_key() {
                possible_tx_inputs.push(TxInput::new(initialize_empty_key_blocks(), tx.get_tx_id(), false, output_index));
                possible_amount += tx_output.amount.clone();
            }
            output_index += 1;
        }

        if possible_amount >= amount {
            break;
        }
    }

    let recipient_pubkey = keypairs[recipient_account_index].pub_key.clone();
    let tx_output: TxOutput = TxOutput::new(recipient_pubkey, amount);
    let mut tx_outputs: Vec<TxOutput> = vec![tx_output];

    if possible_amount > amount {
        let self_tx_output: TxOutput = TxOutput::new(keypair.pub_key.clone(), possible_amount-amount);
        tx_outputs.push(self_tx_output);
    }

    let mut transaction = Tx::new(possible_tx_inputs, tx_outputs);
    for tx_input_index in 0..transaction.inputs.len() {
        transaction.inputs[tx_input_index].signature = keypair.create_signature(&transaction);
    }

    let mut block: Block = Block::new(&vec![transaction], blockchain.blocks[blockchain.blocks.len()-1].block_header.hash_block());
    block.mine_block();
    blockchain.accept_new_block(&block);

}

fn get_utxo(blockchain: &Blockchain, keypairs: &Vec<KeyPair>) {
    let mut utxo_length: usize = 0;
    let mut tx_outputs: Vec<TxOutput> = vec![];

    for tx in &blockchain.utxo {
        utxo_length += tx.outputs.len();
        for tx_output in &tx.outputs {
            tx_outputs.push(tx_output.clone());
        }
    }

    println!("\nUTXO Length: {}", utxo_length);
    for output in &tx_outputs {
        let amount = output.amount;
        let mut account_index: usize = 0;
        
        let mut key_pair_index: usize = 0; 
        for keypair in keypairs {
            if output.pub_key.hash_key() == keypair.pub_key.hash_key() {
                account_index = key_pair_index;
                break;
            }
            key_pair_index += 1;
        }


        println!("${0} for Account #{1}", amount, account_index);
    }
    println!("");
}
