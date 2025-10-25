use sha2::{Sha256, Digest};

use crate::classes::lamport_signature::key_pair::{Key, KeyBlock};
use crate::util::conversions::hex_string_to_bit_vector;

use bincode;
use serde::{Serialize, Deserialize};
use serde_big_array::big_array;

big_array! { BigArray; 256 }

#[derive(Clone, Serialize, Deserialize)]
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

    pub fn convert_tx_to_bytes(&self) -> Vec<u8> {
        let bytes: Vec<u8> = match bincode::serialize(self) {
            Ok(val) => val,
            Err(_e) => {
                println!("Error! Could not convert transaction to bytes.");
                vec![]
            }
        };
        return bytes;
    }

    pub fn get_tx_id(&self) -> String {
        let bytes: Vec<u8> = match bincode::serialize(self) {
            Ok(val) => val,
            Err(_e) => {
                println!("Error! Could not convert transaction to bytes.");
                vec![]
            }
        };
        let mut hasher = Sha256::new();
        hasher.update(bytes);
        let result = hasher.finalize();
        let hex_result = hex::encode(result);
        return hex_result;
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

            for pub_key_one_block in &output.pub_key.one_blocks {
                let block_bytes = [
                    pub_key_one_block.first_part.to_be_bytes(),
                    pub_key_one_block.second_part.to_be_bytes()
                ].concat();
                hasher.update(&block_bytes);
            }

            hasher.update(&output.amount.to_be_bytes());
        }

        let result = hasher.finalize();
        let hex_result = result.iter().map(|byte| format!("{:02x}", byte)).collect::<String>();
        return hex_result;
    }

    pub fn verify_transaction(&self, utxo: &Vec<Tx>) -> bool {
        // check if the total input amount >= total output amount
        
        let mut input_sum: u64 = 0;
        let mut output_sum: u64 = 0;

        for tx_input in &self.inputs {
            let prev_tx_id: &String = &tx_input.prev_tx_id;
            let prev_tx_index = &tx_input.index;

            let mut found_consumed_output: bool = false;
            for utxo_tx in utxo {
                // if found the transaction that holds the consumed output
                if &utxo_tx.get_tx_id() == prev_tx_id && &utxo_tx.outputs.len() > prev_tx_index { 
                    found_consumed_output = true;
                    input_sum += utxo_tx.outputs[prev_tx_index.clone()].amount;
                }
            }

            if !found_consumed_output {
                println!("Could not find a matching output for a new transaction...");
                return false;
            }
        }

        for tx_output in &self.outputs {
            output_sum += tx_output.amount;
        }

        if input_sum < output_sum {
            println!("New transaction has more output amount than input amount...");
            return false;
        }

        // verify all of the signatures
        let verified_tx_signature = self.verify_signature(utxo);

        return verified_tx_signature;
    }

    pub fn verify_signature(&self, utxo: &Vec<Tx>) -> bool {
        let tx_hash_bits: Vec<u8> = hex_string_to_bit_vector(self.get_tx_hash());
        let mut verified: bool = true;

        for input in &self.inputs {
            let input_signature: &[KeyBlock; 256] = &input.signature;

            let mut pub_key_option: Option<Key> = None;
            for utxo_tx in utxo {
                if utxo_tx.get_tx_id() == input.prev_tx_id {
                    pub_key_option = Some(utxo_tx.outputs[input.index].pub_key.clone());
                    break;
                }
            }

            if pub_key_option.is_none() {
                println!("Could not find matching output for transaction input...");
                return false;
            }

            let pub_key: Key = pub_key_option.unwrap();

            let mut j = 0;
            for bit in &tx_hash_bits {
                if *bit == (0 as u8) {
                    // get the corresponding public key block from zero blocks
                    let pub_key_block: KeyBlock = pub_key.zero_blocks[j].clone();
                    // construct it yourself using the signature blocks that are supposed to be chosen from the private key blocks
                    let constructed_pub_key_block: KeyBlock = input_signature[j].hash_priv_key_block();

                    // if they don't match, set verified to false
                    if pub_key_block.first_part != constructed_pub_key_block.first_part || pub_key_block.second_part != constructed_pub_key_block.second_part {
                        verified = false;
                    }
                } else {
                    // repeat the same process as above but for one blocks
                    let pub_key_block: KeyBlock = pub_key.one_blocks[j].clone();
                    let constructed_pub_key_block: KeyBlock = input_signature[j].hash_priv_key_block();


                    if pub_key_block.first_part != constructed_pub_key_block.first_part || pub_key_block.second_part != constructed_pub_key_block.second_part {
                        verified = false;
                    }
                }
                j += 1;
            }
        }
        
        return verified;
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TxInput {
    #[serde(with = "BigArray")]
    pub signature: [KeyBlock; 256],
    pub prev_tx_id: String,
    pub index: usize,
    pub is_coinbase: bool,
}

impl TxInput {
    pub fn new(signature: [KeyBlock; 256], prev_tx_id: String, is_coinbase: bool, index:usize) -> TxInput{
        return TxInput {
            signature,
            prev_tx_id,
            is_coinbase,
            index
        };
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TxOutput {
    pub pub_key: Key,
    pub amount: u64,
}

impl TxOutput {
    pub fn new(pub_key: Key, amount: u64) -> TxOutput {
        return TxOutput {
            pub_key,
            amount
        };
    }
}