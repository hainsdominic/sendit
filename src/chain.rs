use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::{
    fs::{self, OpenOptions},
    io::Write,
    sync::{Arc, Mutex},
    time::{SystemTime, UNIX_EPOCH},
};

use crate::peertable::PublicKey;

pub struct BlockChain {
    data: Arc<Mutex<BlockChainData>>,
}

impl BlockChain {
    pub fn new() -> BlockChain {
        BlockChain {
            data: Arc::new(Mutex::new(BlockChainData::new())),
        }
    }

    pub fn clone(&self) -> BlockChain {
        BlockChain {
            data: self.data.clone(),
        }
    }

    pub fn add_pending_block(&self, block: Block) {
        let mut data = self.data.lock().unwrap();
        data.add_pending_block(block);
    }

    pub fn add_block(&self, block: Block) {
        let mut data = self.data.lock().unwrap();
        data.add_block(block);
    }

    pub fn mine(&self, block_input: BlockInput) -> Result<()> {
        let mut data = self.data.lock().unwrap();
        data.mine(block_input)
    }

    pub fn input_to_block(&self, block_input: BlockInput) -> Block {
        let data = self.data.lock().unwrap();
        data.input_to_block(block_input)
    }

    pub fn num_blocks(&self) -> usize {
        let data = self.data.lock().unwrap();
        data.blocks.len()
    }

    pub fn get_blocks(&self) -> String {
        let data = self.data.lock().unwrap();
        serde_json::to_string(&data.blocks).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct BlockChainData {
    blocks: Vec<Block>,
    pending_blocks: Vec<Block>,
}

impl BlockChainData {
    fn new() -> BlockChainData {
        let file =
            fs::read_to_string("blockchain.json").expect("Something went wrong reading the file");

        let blocks: Vec<Block> = serde_json::from_str(&file).unwrap();

        BlockChainData {
            blocks,
            pending_blocks: Vec::new(),
        }
    }
}

impl BlockChainData {
    fn add_pending_block(&mut self, block: Block) {
        self.pending_blocks.push(block);
    }

    fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
        self.save_to_file();
    }

    fn save_to_file(&self) {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open("blockchain.json")
            .unwrap();

        file.write_all(serde_json::to_string(&self.blocks).unwrap().as_bytes())
            .unwrap();
    }

    fn mine(&mut self, input: BlockInput) -> Result<()> {
        let block = self
            .pending_blocks
            .iter()
            .position(|b| b.file_hash == input.file_hash && b.receiver == input.receiver)
            .map(|i| self.pending_blocks.remove(i));

        match block {
            Some(mut b) => {
                b.prev_hash = Some(self.blocks.last().unwrap().hash.clone().unwrap());
                b.hash();
                self.add_block(b);
                Ok(())
            }
            None => {
                bail!("Block not found in pending blocks");
            }
        }
    }

    pub fn input_to_block(&self, input: BlockInput) -> Block {
        Block::new(
            input,
            self.blocks.len() as u32,
            self.blocks.last().unwrap().hash.clone().unwrap(),
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    index: u32,
    timestamp: u128,
    sender: PublicKey,
    receiver: PublicKey,
    file_hash: String,
    hash: Option<String>,
    prev_hash: Option<String>,
}

impl Block {
    fn new(input: BlockInput, index: u32, prev_hash: String) -> Block {
        Block {
            index,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis(),
            sender: input.sender,
            receiver: input.receiver,
            file_hash: input.file_hash,
            hash: None,
            prev_hash: Some(prev_hash),
        }
    }

    fn hash(&mut self) {
        let mut hasher = sha2::Sha256::new();
        hasher.update(self.index.to_string());
        hasher.update(self.timestamp.to_string());
        hasher.update(self.sender.clone());
        hasher.update(self.receiver.clone());
        hasher.update(self.file_hash.clone());
        hasher.update(self.prev_hash.clone().unwrap());
        self.hash = Some(hex::encode(hasher.finalize()));
    }
}

pub struct BlockInput {
    pub file_hash: String,
    pub sender: PublicKey,
    pub receiver: PublicKey,
}
