mod classes;
mod util;
mod data_structures;

use classes::lamport_signature::key_pair::{KeyPair, KeyBlock, initialize_empty_key_blocks};
use classes::transaction::tx::{Tx, TxInput, TxOutput};
use rand::Rng;
use sha2::{Sha256, Digest};

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