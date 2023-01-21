use std::str::FromStr;

use crate::peertable::{PeerTable, PublicKey, IP};

pub enum Operations {
    AddPeer(PublicKey, IP),
    GetPeer(PublicKey),
}

impl FromStr for Operations {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let operation = parts.next().unwrap();
        let public_key = parts.next().unwrap();

        match operation {
            "add_peer" => {
                let ip = parts.next().unwrap();
                Ok(Operations::AddPeer(public_key.to_string(), ip.to_string()))
            }
            "get_peer" => Ok(Operations::GetPeer(public_key.to_string())),
            _ => Err("Invalid operation".to_string()),
        }
    }
}

pub fn run_operation(operation: &Operations, peer_table: &PeerTable) -> String {
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
    }
}
