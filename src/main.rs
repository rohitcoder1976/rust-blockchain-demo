mod classes;

use classes::lamport_signature::key_pair::{KeyPair, KeyBlock};
use std::mem;

fn main() {
    println!("Hello, world!");
    let new_key_pair: KeyPair = KeyPair::new();
    println!("{}", new_key_pair.priv_key.zero_blocks[0].first_part);
    let total_priv_key_size: u16 = (mem::size_of::<[KeyBlock; 256]>() as u16) / (1000 as u16);
    println!("Total Private Key Size: {} KB", total_priv_key_size);
    println!("Total Key Pair Size: {} KB", total_priv_key_size*2);
}
