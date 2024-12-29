use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use crate::Tx;

#[derive(Clone ,Serialize, Deserialize)]
pub struct MerkleTree {
    pub merkle_root: String,
    pub data: Vec<Vec<[u8; 32]>>,
    pub base: Vec<Tx>,
}

impl MerkleTree {
    pub fn new(tx_data: &Vec<Tx>) -> MerkleTree{
        let mut merkle_data: Vec<Vec<[u8; 32]>> = Vec::new();

        let mut prev_is_pair: bool = false;
        let mut i: usize = 0;
        let mut higher_layer: Vec<[u8; 32]> = vec![];

        // first form the first layer above the base transaction data
        for data_item in tx_data {
            if i == tx_data.len() - 1 { // if at the last data item
                if prev_is_pair {
                    let prev_data_item: Tx = tx_data[i-1].clone();
                    let data_hash: [u8; 32] = hash_two_tx(&prev_data_item, &data_item);
                    higher_layer.push(data_hash);
                    prev_is_pair = false;
                } else {
                    let mut hasher = Sha256::new();
                    hasher.update(data_item.convert_tx_to_bytes());
                    let data_hash: [u8; 32] = hasher.finalize().into();
                    higher_layer.push(data_hash);
                }
            } else {  
                if prev_is_pair {
                    let prev_data_item: Tx = tx_data[i-1].clone();
                    let data_hash: [u8; 32] = hash_two_tx(&prev_data_item, &data_item);
                    higher_layer.push(data_hash);
                    prev_is_pair = false;
                } else { 
                    /* if not at the last data item, and the previous data item is paired up with another (or if at the first) data item and
                    data length is greater than 1, the next item must be paired up with the current one */ 
                    prev_is_pair = true;
                }
            }
            i += 1;
        }

        let data: Vec<[u8; 32]> = higher_layer.clone();
        merkle_data.push(higher_layer.clone());

        let mut data: Vec<[u8; 32]> = data.clone();

        /* use the first layer to build the remaining layers 
        as long as the highest layer of the merkle tree has more than one element, 
        keep hashing pairs to get to the merkle root */
        while merkle_data[merkle_data.len() - 1].len() > 1 {
            let mut prev_is_pair: bool = false;
            let mut i: usize = 0;
            let mut higher_layer: Vec<[u8; 32]> = vec![];

            for data_item in &data {
                if i == data.len() - 1 { // if at the last data item
                    if prev_is_pair {
                        let prev_data_item: [u8; 32] = data[i-1].clone();
                        let mut hasher = Sha256::new();
                        hasher.update([&prev_data_item[..], &data_item[..]].concat());
                        let data_hash: [u8; 32] = hasher.finalize().into();
                        higher_layer.push(data_hash);
                        prev_is_pair = false;
                    } else {
                        let mut hasher = Sha256::new();
                        hasher.update(data_item);
                        let data_hash: [u8; 32] = hasher.finalize().into();
                        higher_layer.push(data_hash);
                    }
                } else {  
                    if prev_is_pair {
                        let prev_data_item: [u8; 32] = data[i-1].clone();
                        let mut hasher = Sha256::new();
                        hasher.update([&prev_data_item[..], &data_item[..]].concat());
                        let data_hash: [u8; 32] = hasher.finalize().into();
                        higher_layer.push(data_hash);
                        prev_is_pair = false;
                    } else { 
                        /* if not at the last data item, and the previous data item is paired up with another (or if at the first) data item and
                        data length is greater than 1, the next item must be paired up with the current one */ 
                        prev_is_pair = true;
                    }
                }
                i += 1;
            }

            data = higher_layer.clone();
            merkle_data.push(higher_layer.clone());
        }

        // convert merkle root in byte form to hex form for the struct
        let merkle_root_hex_string: String = merkle_data[merkle_data.len()-1][0].iter().map(|byte: &u8| format!("{:02x}", byte)).collect();
        return MerkleTree {
            merkle_root: merkle_root_hex_string,
            data: merkle_data.clone(),
            base: tx_data.clone(),
        };
    }
}

fn hash_two_tx(first_tx: &Tx, second_tx: &Tx) -> [u8; 32] {
    let mut hasher = Sha256::new();
    let mut combined_txs = first_tx.convert_tx_to_bytes();
    combined_txs.extend(second_tx.convert_tx_to_bytes());
    hasher.update(combined_txs);
    let result: [u8; 32] = hasher.finalize().into();
    return result;
}