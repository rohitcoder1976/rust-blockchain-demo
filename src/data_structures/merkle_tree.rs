use sha2::{Sha256, Digest};

pub struct MerkleTree {
    pub merkle_root: String,
    pub data: Vec<Vec<[u8; 32]>>,
}

impl MerkleTree {
    pub fn new(data: &Vec<[u8; 32]>){
        let mut merkle_data: Vec<Vec<[u8; 32]>> = Vec::new();
        merkle_data.push(data.clone());

        let mut data: Vec<[u8; 32]> = data.clone();

        while merkle_data[merkle_data.len() - 1].len() > 1 {
            let mut prev_is_pair: bool = false;
            let mut i: usize = 0;
            let mut higher_layer: Vec<[u8; 32]> = vec![];

            for data_item in &data {
                if i == data.len() - 1 { // if at the last data item
                    if prev_is_pair {
                        let prev_data_item = data[i-1].clone();
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
                        let prev_data_item = data[i-1].clone();
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
            println!("Higher layer length: {}", higher_layer.len());
        }

        println!("The merkle tree has {} layers", merkle_data.len());
        
    }
}