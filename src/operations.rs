use std::str::FromStr;

use crate::{
    chain::{BlockChain, BlockInput},
    peertable::{PeerTable, PublicKey, IP},
};

pub enum Operations {
    AddPeer(PublicKey, IP),
    GetPeer(PublicKey),
    AddBlock(BlockInput),
    MineBlock(BlockInput),
}

impl FromStr for Operations {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();

        let operation = parts.next().unwrap();
        match operation {
            "add_peer" => {
                let public_key = parts.next().unwrap();
                let ip = parts.next().unwrap();
                Ok(Operations::AddPeer(public_key.to_string(), ip.to_string()))
            }
            "get_peer" => {
                let public_key = parts.next().unwrap();
                Ok(Operations::GetPeer(public_key.to_string()))
            }
            "add_block" => {
                let block_input = BlockInput {
                    file_hash: parts.next().unwrap().to_string(),
                    sender: parts.next().unwrap().to_string(),
                    receiver: parts.next().unwrap().to_string(),
                };
                Ok(Operations::AddBlock(block_input))
            }
            "mine_block" => {
                let block_input = BlockInput {
                    file_hash: parts.next().unwrap().to_string(),
                    sender: parts.next().unwrap().to_string(),
                    receiver: parts.next().unwrap().to_string(),
                };
                Ok(Operations::MineBlock(block_input))
            }
            _ => Err("Invalid operation".to_string()),
        }
    }
}

pub fn run_operation(
    operation: Operations,
    peer_table: &PeerTable,
    blockchain: &mut BlockChain,
) -> String {
    match operation {
        Operations::AddPeer(public_key, ip) => {
            match peer_table.add_peer(public_key.to_string(), ip.to_string()) {
                Ok(_) => "Peer added".to_string(),
                Err(_) => return "Peer already exists".to_string(),
            }
        }
        Operations::GetPeer(public_key) => {
            let ip = peer_table.get_peer(public_key.to_string());
            match ip {
                Some(ip) => ip,
                None => "Peer not found".to_string(),
            }
        }
        Operations::AddBlock(block_input) => {
            let block = blockchain.input_to_block(block_input);
            blockchain.add_pending_block(block);
            "Block added".to_string()
        }
        Operations::MineBlock(block_input) => match blockchain.mine(block_input) {
            Ok(_) => "Block mined".to_string(),
            Err(_) => "Block not added".to_string(),
        },
    }
}
