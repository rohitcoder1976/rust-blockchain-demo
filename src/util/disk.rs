use std::{fs::File, io::{self, Read, Write}};

use bincode::de;

use crate::classes::block::blockchain::Blockchain;

pub fn save_chain_branches_to_file(chains: &Vec<Blockchain>) -> Result<(), ()> {
    let mut file_result = File::create("branches.bin");
    let mut file: File = match file_result {
        Ok(val) => val,
        Err(err) => {
            println!("Could not save create file");
            return Err(());
        }
    };
    
    let encoded_result = bincode::serialize(chains);
    let encoded = match encoded_result {
        Ok(val) => val,
        Err(e) => vec![]
    };
    
    let write_result = file.write_all(&encoded);
    if let Err(write_error) = write_result {
        println!("Failed to write chain branches to disk: {}", write_error);
        return Err(());
    }
    
    return Ok(());
}

pub fn load_branches_from_file() -> Result<Vec<Blockchain>, ()> {
    let file_result: Result<File, io::Error> = File::open("branches.bin");
    let mut file = match file_result {
        Ok(val) => val,
        Err(err) => {
            // println!("Failed to read chain branches file...");
            return Err(())
        }
    } ;

    let mut encoded= Vec::new();
    file.read_to_end(&mut encoded);
    let decoded_result: Result<Vec<Blockchain>, Box<bincode::ErrorKind>> = bincode::deserialize(&encoded);
    let decoded: Vec<Blockchain> = match decoded_result {
        Ok(val) => val,
        Err(e) => {
            println!("Failed to decode chain branches saved to disk...");
            vec![]
        }
    };

    if decoded.len() == 0 {
        return Err(());
    }

    Ok(decoded)
}