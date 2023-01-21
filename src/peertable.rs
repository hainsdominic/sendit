use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use anyhow::Result;

pub(crate) type IP = String;
pub(crate) type PublicKey = String;
pub struct PeerTable {
    data: Arc<Mutex<HashMap<PublicKey, IP>>>,
}

impl PeerTable {
    pub fn new() -> PeerTable {
        PeerTable {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn add_peer(&self, public_key: PublicKey, ip: IP) -> Result<()> {
        let mut data = self.data.lock().unwrap();
        if data.contains_key(&public_key) {
            return Err(anyhow::anyhow!("Peer already exists"));
        }
        data.insert(public_key, ip);
        Ok(())
    }

    pub fn get_peer(&self, public_key: PublicKey) -> Option<IP> {
        let data = self.data.lock().unwrap();
        data.get(&public_key).cloned()
    }

    pub fn clone(&self) -> PeerTable {
        PeerTable {
            data: self.data.clone(),
        }
    }
}
