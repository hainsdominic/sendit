use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    str::FromStr,
    thread,
};

use sendit::{
    chain::BlockChain,
    operations::{run_operation, Operations},
    peertable::PeerTable,
};

fn handle_client(mut stream: TcpStream, peer_table: PeerTable, mut blockchain: BlockChain) {
    println!("New client: {:?}", stream.peer_addr().unwrap());
    let mut buffer = [0; 1024];
    loop {
        let bytes_read = match stream.read(&mut buffer) {
            Ok(n) => n,
            Err(_) => break,
        };
        if bytes_read == 0 {
            break;
        }
        let raw_message = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();

        let raw_message = raw_message.trim_end_matches("\r\n");

        let operation = Operations::from_str(&raw_message).unwrap();

        let mut result = run_operation(operation, &peer_table, &mut blockchain);

        result.push_str("\n");

        stream.write(result.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

fn main() -> std::io::Result<()> {
    let addr = "0.0.0.0:5000";
    let listener = TcpListener::bind(addr)?;
    let peer_table = PeerTable::new();
    let blockchain = BlockChain::new();

    println!("Listening on {}", addr);

    thread::scope(|s| {
        for stream in listener.incoming() {
            let peer_table = peer_table.clone();
            let blockchain = blockchain.clone();
            s.spawn(move || {
                handle_client(stream.unwrap(), peer_table, blockchain);
            });
        }
    });
    Ok(())
}
