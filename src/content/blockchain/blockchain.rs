use crate::{content::blockchain::block::Block, content::user::transaction::Transaction};  

#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub mempool: Vec<Transaction>,
    pub difficulty: u32
}

impl Blockchain {
    pub fn new(difficulty: u32) -> Self {
        let mut genesis_block = Block::new(
            0,
            vec![],
            "0".to_string(),
            0 
        );

        genesis_block.mine_block(difficulty);  
        Blockchain {
            chain: vec![genesis_block],
            mempool: vec![],
            difficulty,  
        }
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) {
        let previous_block = self.chain.last().unwrap();
        let mut new_block = Block::new(
            previous_block.index + 1,
            transactions, 
            previous_block.hash.clone(), 
            0
        );
        new_block.mine_block(self.difficulty);
        self.chain.push(new_block);
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];
            
            if current.previous_hash != previous.hash {
                return false;
            }
            
            if current.hash != current.calculate_hash() {
                return false;
            }
        }
        return true;
    }

    pub fn mine_pending_transactions(&mut self, miner_address: &str) {
        let reward = Transaction::new(
            "system",
            miner_address,
            6.25, 
        );

        let mut block_transactions = vec![reward];
        block_transactions.extend(self.mempool.drain(..));

        self.add_block(block_transactions);
    }
}