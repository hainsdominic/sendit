#![allow(dead_code)]
use sha2::Digest;
use std::time::{SystemTime, UNIX_EPOCH};

struct BlockChain {
    blocks: Vec<Block>,
    nb_blocks: u32,
    pending_blocks: Vec<Block>,
}

impl BlockChain {
    fn new() -> BlockChain {
        BlockChain {
            blocks: vec![Block {
                index: 0,
                timestamp: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis(),
                sender: String::from("Alice"),
                receiver: String::from("Bob"),
                file_hash: String::from("foo"),
                hash: String::from("genesis"),
                prev_hash: String::new(),
            }],
            nb_blocks: 0,
            pending_blocks: Vec::new(),
        }
    }

    fn add_block(&mut self, block: Block) {
        self.pending_blocks.push(block);
    }

    fn mine(&mut self, block_index: u32, file_hash: String, sender: String, receiver: String) {
        let mut block = self.pending_blocks.remove(block_index as usize);
        block.file_hash = file_hash;
        block.sender = sender;
        block.receiver = receiver;
        block.timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        block.hash = block.hash();
        block.prev_hash = self.blocks.last().unwrap().hash.clone();
        self.blocks.push(block);
    }
}

pub struct Block {
    index: u32,
    timestamp: u128,
    sender: String,
    receiver: String,
    file_hash: String,
    hash: String,
    prev_hash: String,
}

impl Block {
    fn hash(&self) -> String {
        let mut hasher = sha2::Sha256::new();
        hasher.update(self.index.to_string());
        hasher.update(self.timestamp.to_string());
        hasher.update(self.sender.clone());
        hasher.update(self.receiver.clone());
        hasher.update(self.file_hash.clone());
        hasher.update(self.prev_hash.clone());
        format!("{:x}", hasher.finalize())
    }
}
