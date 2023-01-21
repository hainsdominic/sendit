#![allow(dead_code)]
use anyhow::{bail, Result};
use sha2::Digest;
use std::{
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
        println!("Pending blocks: {:?}", data.pending_blocks);
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
}

#[derive(Debug)]
struct BlockChainData {
    blocks: Vec<Block>,
    nb_blocks: u32,
    pending_blocks: Vec<Block>,
}

impl BlockChainData {
    fn new() -> BlockChainData {
        BlockChainData {
            blocks: vec![Block {
                index: 0,
                timestamp: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis(),
                sender: String::from("Alice"),
                receiver: String::from("Bob"),
                file_hash: String::from("foo"),
                hash: Some(String::from("genesis")),
                prev_hash: Some(String::new()),
            }],
            nb_blocks: 0,
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
        self.nb_blocks += 1;
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
                println!("Blocks: {:?}", self.blocks);
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
            self.nb_blocks + 1,
            self.blocks.last().unwrap().hash.clone().unwrap(),
        )
    }
}

#[derive(Debug)]
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
