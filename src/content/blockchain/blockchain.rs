use rand::Rng;

use crate::content::{blockchain::block::Block, user::{transaction::Transaction, wallet, Wallet}};  

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
        let mut block_transactions = Vec::new();

        let reward = Transaction::new(
            "System",
            miner_address,
            6.25, 
            0.0, 
        );
        block_transactions.push(reward);

        let mut total_fee = 0.0;
        while let Some(tx) = self.mempool.pop() {
            block_transactions.push(tx.clone());
            total_fee += tx.fee;
        }
        if total_fee > 0.0 {
            let fee_reward = Transaction::new(
                "Fees",
                miner_address,
                total_fee,
                0.0, 
            );
            block_transactions.push(fee_reward);
        }

        self.add_block(block_transactions);
    }

    pub fn get_balance(&self, address: &str) -> f64 {
        let mut balance = 0.0;

        for block in &self.chain {
            for transaction in &block.transactions {
                if transaction.sender == address {
                    balance -= transaction.amount;
                }
                if transaction.receiver == address {
                    balance += transaction.amount;
                }
            }
        }
        return balance;
    }

    pub fn attempt_random_mining(&mut self, wallets: &[Wallet]) {
        // Filter miners from the list of wallets
        let miners: Vec<&Wallet> = wallets.iter().filter(|w| w.is_miner).collect();
        
        if !miners.is_empty() {
            // Randomly select a miner
            let mut rng = rand::rng();
            let miner = miners[rng.random_range(0..miners.len())];
            
            // If there are pending transactions, mine them
            if !self.mempool.is_empty() {
                self.mine_pending_transactions(&miner.address());
                println!("Block mined by miner: {}", miner.address());
            }
        }
    }
}