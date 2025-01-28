use chrono::Utc;
use sha2::{Sha256, Digest};

#[derive(Debug)]
pub struct Block {
    pub index: u32,
    pub timestamp: i64,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64
}

impl Block {
    pub fn new(index: u32, data: String, previous_hash: String, nonce: u64) -> Self {
        let timestamp = Utc::now().timestamp(); 
        let mut block = Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash: String::new(), 
            nonce,
        };
        block.hash = String::new();
        return block;
    }

    pub fn calculate_hash(&self) -> String {

        let input = format!(
            "{}{}{}{}{}",
            self.index,
            self.timestamp,
            self.data,
            self.previous_hash,
            self.nonce);
    
        let mut hasher = Sha256::new();
    
        hasher.update(input.as_bytes());
    
        let result = hasher.finalize();
    
        return hex::encode(result);
    }

    pub fn mine_block(&mut self, difficulty: u32) {
        let target_prefix = "0".repeat(difficulty as usize);
        loop {
            self.hash = self.calculate_hash(); 
            if self.hash.starts_with(&target_prefix) {
                break;
            }
            self.nonce += 1; 
        }
        println!("Block mined: {}", self.hash);
    }
}