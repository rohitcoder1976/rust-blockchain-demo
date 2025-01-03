use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, RwLock};

use crate::classes::block::blockchain::Blockchain;

fn handle_client(mut stream: TcpStream, blockchain: Arc<RwLock<Blockchain>>) {
    let mut buffer: [u8; 1024] = [0; 1024];
    stream.read(&mut buffer).expect("Failed to read request buffer");
    let request_str: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer[..]);
    println!("Request received: {}", request_str);
    let response: &[u8] = "Hello There!".as_bytes();

    if request_str.contains("GET /blocks") {

    } else if request_str.contains("GET /utxo") {

    } else if request_str.contains("POST /new_tx") {

    }

    stream.write(response).expect("Could not send back response...");

}

pub fn init_tcp_server(blockchain: Blockchain) {
    let blockchain = Arc::new(RwLock::new(blockchain));
    let listener: TcpListener = TcpListener::bind("127.0.0.1:8080").expect("Could not bind server to address");
    println!("Server listening on 127.0.0.1:8080");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let blockchain_copy = Arc::clone(&blockchain);
                std::thread::spawn( || handle_client(stream, blockchain_copy));
            },
            Err(e) => {
                eprint!("Error: could not read the stream -- {}", e);
            }
        }
    }
}