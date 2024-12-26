// use lamport_signature::key_pair::{Key, KeyBlock};
use crate::classes::lamport_signature::key_pair::{Key, KeyBlock};
use sha2::{Sha256, Digest};

pub struct Tx {
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,    
}

impl Tx {
    pub fn new(inputs: Vec<TxInput>, outputs: Vec<TxOutput>) -> Tx {
        return Tx {
            inputs: inputs,
            outputs: outputs
        };
    }

    pub fn get_tx_hash(&self) -> String{
        let mut hasher = Sha256::new();
        
        for input in &self.inputs {
            hasher.update(&[if input.is_coinbase {1} else {0}]);
            hasher.update(input.prev_tx_id.as_bytes());
        }

        for output in &self.outputs {
            for pub_key_zero_block in &output.pub_key.zero_blocks {
                let block_bytes = [
                    pub_key_zero_block.first_part.to_be_bytes(),
                    pub_key_zero_block.second_part.to_be_bytes()
                ].concat();
                hasher.update(&block_bytes);
            }

            hasher.update(&output.amount.to_be_bytes());
        }

        let result = hasher.finalize();
        let hex_result = result.iter().map(|byte| format!("{:02x}", byte)).collect::<String>();
        return hex_result;
    }
}

pub struct TxOutput {
    pub pub_key: Key,
    pub amount: u64,
}

impl TxOutput {
    pub fn new(pub_key: Key, amount: u64) -> TxOutput {
        return TxOutput {
            pub_key: pub_key,
            amount: amount
        };
    }
}

pub struct TxInput {
    pub signature: [KeyBlock; 256],
    pub prev_tx_id: String,
    pub is_coinbase: bool,
}

impl TxInput {
    pub fn new(signature: [KeyBlock; 256], prev_tx_id: String, is_coinbase: bool) -> TxInput{
        return TxInput {
            signature: signature,
            prev_tx_id: prev_tx_id,
            is_coinbase: is_coinbase
        };
    }
}