use std::{net::TcpStream, str::FromStr};

use log::{info, warn};

use crate::{
    chain::{BlockChain, BlockInput},
    peertable::{PeerTable, PublicKey},
};

pub enum Operations {
    AddPeer(PublicKey),
    GetPeer(PublicKey),
    AddBlock(BlockInput),
    MineBlock(BlockInput),
    GetBlocks,
    NumBlocks,
    NoOp,
}

impl FromStr for Operations {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();

        let operation = parts.next().unwrap_or_default();
        match operation {
            "add_peer" => {
                let public_key = parts.next().unwrap();
                Ok(Operations::AddPeer(public_key.to_string()))
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
            "num_blocks" => Ok(Operations::NumBlocks),
            "get_blocks" => Ok(Operations::GetBlocks),
            _ => Ok(Operations::NoOp),
        }
    }
}

pub fn run_operation(
    operation: Operations,
    peer_table: &PeerTable,
    blockchain: &mut BlockChain,
    stream: &TcpStream,
) -> String {
    match operation {
        Operations::AddPeer(public_key) => {
            match peer_table.add_peer(
                public_key.to_string(),
                stream.peer_addr().unwrap().ip().to_string(),
            ) {
                Ok(_) => {
                    log::info!("Peer added");
                    "Peer added".to_string()
                }
                Err(_) => {
                    log::warn!("Peer already exists");
                    "Peer already exists".to_string()
                }
            }
        }
        Operations::GetPeer(public_key) => {
            let ip = peer_table.get_peer(public_key.to_string());
            match ip {
                Some(ip) => {
                    log::info!("Peer found");
                    ip
                }
                None => {
                    log::warn!("Peer not found");
                    "Peer not found".to_string()
                }
            }
        }
        Operations::AddBlock(block_input) => {
            let block = blockchain.input_to_block(block_input);
            blockchain.add_pending_block(block);
            info!("Block added");
            "Block added".to_string()
        }
        Operations::MineBlock(block_input) => match blockchain.mine(block_input) {
            Ok(_) => {
                info!("Block mined");
                "Block mined".to_string()
            }
            Err(e) => {
                warn!("Block not added: {}", e.to_string());
                "Block not added".to_string()
            }
        },
        Operations::NumBlocks => {
            let num_blocks = blockchain.num_blocks();
            info!("Get number of blocks");
            num_blocks.to_string()
        }
        Operations::GetBlocks => {
            info!("Get blocks");
            blockchain.get_blocks()
        }
        Operations::NoOp => "No operation".to_string(),
    }
}
