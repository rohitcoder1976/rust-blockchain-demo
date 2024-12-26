mod classes;

use classes::lamport_signature::key_pair::{KeyPair, KeyBlock, Key, initialize_empty_key_blocks};
use classes::transaction::tx::{Tx, TxInput, TxOutput};

fn main() {
    println!("Hello, world!");
    let new_key_pair1: KeyPair = KeyPair::new();
    let signature: [KeyBlock; 256] = new_key_pair1.create_signature("Hello!");
    
    let mut tx_inputs: Vec<TxInput> = Vec::new();
    let prev_tx_id = String::new();
    tx_inputs.push(TxInput::new(initialize_empty_key_blocks(), prev_tx_id, true));

    let mut tx_outputs: Vec<TxOutput> = Vec::new();
    tx_outputs.push(TxOutput::new(new_key_pair1.pub_key.clone(), 100));

    let new_tx = Tx::new(tx_inputs, tx_outputs);
    let tx_hash = new_tx.get_tx_hash();

    
}
