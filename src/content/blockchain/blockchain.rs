use std::time::{SystemTime, UNIX_EPOCH};    
use crate::content::{blockchain::block::Block, user::{transaction::Transaction, wallet, Wallet}};  

#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub mempool: Vec<Transaction>,
    pub difficulty: u32,
    last_mined_time: u64
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
            last_mined_time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        }
    }

    /// Adjusts the mining difficulty based on the time elapsed since the last mining operation.
    ///
    /// The difficulty is dynamically adjusted to maintain a target block time:
    /// - **Increases difficulty** if the time since last mining was less than 10 seconds
    /// - **Decreases difficulty** (to minimum 1) if the time exceeded 20 seconds
    /// - Updates the last mined time to the current system time after adjustment
    ///
    /// # Behavior
    /// - Uses a target window of 10 seconds for difficulty calibration
    /// - Difficulty increases by 1 for fast mining (sub-10-second intervals)
    /// - Difficulty decreases by 1 for slow mining (over-20-second intervals)
    /// - Maintains a minimum difficulty of 1
    /// - Always updates the last mined time to current system time
    ///
    /// # Panics
    /// Panics if the system time is before UNIX_EPOCH (should never happen in normal operation)
    ///
    /// # Example
    /// ```
    /// let mut miner = YourStruct {
    ///     difficulty: 2,
    ///     last_mined_time: 0,  // Represents 1970-01-01 00:00:00 UTC
    /// };
    ///
    /// // Simulate mining after 5 seconds (faster than target)
    /// miner.adjust_difficulty();
    /// assert_eq!(miner.difficulty, 3);
    ///
    /// // Simulate mining after 25 seconds (slower than target)
    /// miner.adjust_difficulty();
    /// assert_eq!(miner.difficulty, 2);
    /// ```
    pub fn adjust_difficulty(&mut self) {
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let time_diff = current_time - self.last_mined_time;
        let expected_time = 10;

        if time_diff < expected_time {
            self.difficulty += 1;
        }
        else if time_diff > expected_time * 2 {
            if self.difficulty > 1 {
                self.difficulty -= 1;
            }
        }
        self.last_mined_time = current_time;
    }

    /// Adds a new block to the blockchain with the provided transactions.
    ///
    /// This function creates a new block using the given list of `transactions`, 
    /// links it to the previous block in the chain, and mines it based on the current 
    /// `difficulty` level. Once mined, the new block is appended to the blockchain.
    ///
    /// # Arguments
    ///
    /// * `transactions` - A vector of `Transaction` objects to be included in the new block.
    ///
    /// # Example
    ///
    /// ```
    /// let transactions = vec![Transaction::new("Alice", "Bob", 50)];
    /// blockchain.add_block(transactions);
    /// ```
    ///
    /// # Panics
    ///
    /// This function will panic if the blockchain (`self.chain`) is empty, as it 
    /// calls `unwrap()` on the last block to retrieve the previous block.
    ///
    /// # Notes
    ///
    /// - The `mine_block` function is assumed to adjust the `nonce` until the block's
    ///   hash meets the required difficulty.
    /// - Ensure the blockchain is initialized with at least one block (genesis block)
    ///   before calling this method to avoid panics.
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

    /// Validates the integrity of the blockchain.
    ///
    /// This function checks the blockchain to ensure its integrity by verifying two conditions:
    /// 1. Each block's `previous_hash` matches the `hash` of the preceding block.
    /// 2. Each block's `hash` is consistent with its computed hash (via `calculate_hash()`).
    ///
    /// If any of these conditions fail, the blockchain is considered invalid, and the function 
    /// returns `false`. If all checks pass, the function returns `true`, indicating the blockchain 
    /// is valid.
    ///
    /// # Returns
    ///
    /// * `true` if the blockchain is valid.
    /// * `false` if any block has been tampered with or if the chain structure is broken.
    ///
    /// # Example
    ///
    /// ```
    /// if blockchain.is_valid() {
    ///     println!("The blockchain is valid.");
    /// } else {
    ///     println!("The blockchain has been compromised.");
    /// }
    /// ```
    ///
    /// # Notes
    ///
    /// - This function assumes that the blockchain has been initialized properly with a genesis block.
    /// - The function starts validation from the second block, as the genesis block has no predecessor.
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

    /// Mines all pending transactions and adds them to the blockchain.
    ///
    /// This function performs the following steps:
    /// 1. Creates a mining reward transaction of **6.25** units, assigned to the provided `miner_address`.
    /// 2. Moves all transactions from the `mempool` into a new block, accumulating transaction fees.
    /// 3. If there are any transaction fees, it creates an additional reward transaction for the miner.
    /// 4. Adds the new block containing the reward and pending transactions to the blockchain.
    /// 5. Adjusts the mining difficulty after successfully adding the block.
    ///
    /// # Arguments
    ///
    /// * `miner_address` - A string slice representing the address of the miner who will receive the mining reward and transaction fees.
    ///
    /// # Example
    ///
    /// ```
    /// blockchain.mine_pending_transactions("Miner123");
    /// println!("New block mined! Reward sent to Miner123.");
    /// ```
    ///
    /// # Notes
    ///
    /// - The mining reward is fixed at **6.25** units (similar to Bitcoin's block reward structure).
    /// - The function moves all transactions from the `mempool` into the block, leaving it empty afterward.
    /// - If no transactions with fees are present, only the mining reward will be included.
    /// - After mining, the difficulty is adjusted based on your blockchain’s rules (handled by `adjust_difficulty()`).
    pub fn mine_pending_transactions(&mut self, miner_address: &str) {
        let mut block_transactions = Vec::new();

        // Create mining reward transaction
        let reward = Transaction::new(
            "System",         // Sender: System
            miner_address,    // Receiver: Miner
            6.25,             // Mining reward
            0.0,              // No fee for reward
        );
        block_transactions.push(reward);

        // Collect pending transactions and accumulate fees
        let mut total_fee = 0.0;
        while let Some(tx) = self.mempool.pop() {
            block_transactions.push(tx.clone());
            total_fee += tx.fee;
        }

        // Add transaction fee reward if applicable
        if total_fee > 0.0 {
            let fee_reward = Transaction::new(
                "Fees",         // Sender: Fees system
                miner_address,  // Receiver: Miner
                total_fee,      // Total accumulated fees
                0.0,            // No fee for fee reward
            );
            block_transactions.push(fee_reward);
        }

        // Add the block to the blockchain
        self.add_block(block_transactions);

        // Adjust the mining difficulty
        self.adjust_difficulty();
    }

    /// Calculates and returns the balance of a given address.
    ///
    /// This function iterates through all the blocks and their transactions in the blockchain
    /// to compute the balance of the specified `address`. It subtracts the amounts sent 
    /// and adds the amounts received by the address.
    ///
    /// # Arguments
    ///
    /// * `address` - A string slice representing the wallet address whose balance is to be calculated.
    ///
    /// # Returns
    ///
    /// * `f64` - The net balance of the provided address. A positive value indicates funds received 
    ///   exceed funds sent, while a negative value indicates more funds were sent than received.
    ///
    /// # Example
    ///
    /// ```
    /// let balance = blockchain.get_balance("Alice");
    /// println!("Alice's balance: {}", balance);
    /// ```
    ///
    /// # Notes
    ///
    /// - This function does **not** account for transaction fees unless fees are explicitly deducted 
    ///   in the transaction amounts.
    /// - Mining rewards are treated as regular transactions from the "System" to the miner's address.
    /// - Ensure that all transactions in the blockchain are valid before using this function to 
    ///   retrieve accurate balances.
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
}