use sha2::{Sha256, Digest};

use crate::classes::lamport_signature::key_pair::{Key, KeyBlock};
use crate::util::conversions::hex_string_to_bit_vector;

#[derive(Clone)]
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
        let mut bytes: Vec<u8> = vec![];
        for input in &self.inputs {
            bytes.push(if input.is_coinbase {1} else {0});
            let prev_tx_id_bytes = input.prev_tx_id.as_bytes();
            for byte in prev_tx_id_bytes {
                bytes.push(*byte);
            }
            for signature_block in &input.signature {
                let first_part_bytes: [u8; 16] = signature_block.first_part.to_be_bytes();
                let second_part_bytes: [u8; 16] = signature_block.second_part.to_be_bytes();

                for byte in first_part_bytes {
                    bytes.push(byte);
                }

                for byte in second_part_bytes {
                    bytes.push(byte);
                }
            }
        }

        for output in &self.outputs {
            for pub_key_zero_block in &output.pub_key.zero_blocks {
                let block_bytes: Vec<u8> = [
                    pub_key_zero_block.first_part.to_be_bytes(),
                    pub_key_zero_block.second_part.to_be_bytes()
                ].concat();
                for byte in block_bytes {
                    bytes.push(byte);
                }
            }

            for pub_key_one_block in &output.pub_key.one_blocks {
                let block_bytes = [
                    pub_key_one_block.first_part.to_be_bytes(),
                    pub_key_one_block.second_part.to_be_bytes()
                ].concat();
                for byte in block_bytes {
                    bytes.push(byte);
                }
            }

            let amount_bytes = &output.amount.to_be_bytes();
            for byte in amount_bytes {
                bytes.push(*byte);
            }
        }
        return bytes.clone();
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

    pub fn verify_signature(&self, pub_key: &Key) -> bool {
        let tx_hash_bits: Vec<u8> = hex_string_to_bit_vector(self.get_tx_hash());
        let mut verified: bool = true;

        for input in &self.inputs {
            let input_signature: &[KeyBlock; 256] = &input.signature;

            let mut j= 0;
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

#[derive(Clone)]
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

#[derive(Clone)]
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