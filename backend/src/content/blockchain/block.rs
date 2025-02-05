use chrono::Utc;
use sha2::{Sha256, Digest};
use crate::content::user::transaction::Transaction;

#[derive(Debug)]
pub struct Block {
    pub index: u32,
    pub timestamp: i64,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64
}

impl Block {
    pub fn new(index: u32, transactions: Vec<Transaction>, previous_hash: String, nonce: u64) -> Self {
        let timestamp = Utc::now().timestamp(); 
        let mut block = Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            hash: String::new(), 
            nonce,
        };
        block.hash = String::new();
        return block;
    }

    /// Mines the block by finding a valid hash that meets the difficulty criteria.
    ///
    /// This function performs the Proof-of-Work (PoW) algorithm by repeatedly calculating 
    /// the block's hash until it finds one that starts with a specific number of leading zeros 
    /// defined by the `difficulty`. The process involves incrementing the `nonce` value 
    /// until the condition is met.
    ///
    /// # Arguments
    ///
    /// * `difficulty` - A `u32` value representing the mining difficulty. The higher the difficulty, 
    ///   the more leading zeros are required in the hash, making mining more computationally intensive.
    ///
    /// # Process
    ///
    /// - A target prefix of leading zeros is created based on the difficulty.
    /// - The block's `hash` is recalculated until it starts with the target prefix.
    /// - The `nonce` is incremented on each iteration to generate a new hash.
    /// - Once a valid hash is found, mining stops, and the hash is printed to the console.
    ///
    /// # Example
    ///
    /// ```
    /// let mut block = Block::new(1, transactions, "previous_hash".to_string(), 0);
    /// block.mine_block(4);  // Finds a hash starting with "0000"
    /// ```
    ///
    /// # Output
    ///
    /// This function prints the successfully mined block's hash to the console:
    ///
    /// ```text
    /// Block mined: 0000abcd1234ef...
    /// ```
    ///
    /// # Notes
    ///
    /// - Increasing the difficulty exponentially increases the time required to mine a block.
    /// - This function assumes the `calculate_hash()` method includes the `nonce` in its hash calculation.
    /// - The mining process is CPU-intensive and will block the thread until a valid hash is found.
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
    /// Calculates the SHA-256 hash of the block's contents.
    ///
    /// This function generates a unique hash for the block by concatenating its critical fields 
    /// (`index`, `timestamp`, `transactions`, `previous_hash`, and `nonce`) into a single string. 
    /// It then applies the SHA-256 hashing algorithm to this string to produce a hexadecimal hash.
    ///
    /// # Returns
    ///
    /// * `String` - The hexadecimal representation of the SHA-256 hash for the block.
    ///
    /// # Process
    ///
    /// 1. Concatenates the following fields into a formatted string:
    ///     - `index`: The block's position in the blockchain.
    ///     - `timestamp`: The time when the block was created.
    ///     - `transactions`: A list of transactions included in the block.
    ///     - `previous_hash`: The hash of the previous block in the chain.
    ///     - `nonce`: A number used for mining (Proof-of-Work).
    /// 2. Converts the concatenated string into bytes.
    /// 3. Passes the byte array through the SHA-256 hashing algorithm.
    /// 4. Encodes the resulting hash into a hexadecimal string.
    ///
    /// # Example
    ///
    /// ```
    /// let block_hash = block.calculate_hash();
    /// println!("Block hash: {}", block_hash);
    /// ```
    ///
    /// # Notes
    ///
    /// - This function is typically used in mining (`mine_block`) to verify that the hash meets 
    ///   the required difficulty.
    /// - Any change to the block's contents (e.g., transactions, nonce) will result in a completely 
    ///   different hash due to the properties of SHA-256.
    /// - Ensure that the `transactions` type implements the `Debug` trait (`{:?}`) for formatting.
    ///
    /// # Dependencies
    ///
    /// - Uses the `sha2` crate for SHA-256 hashing.
    /// - Uses the `hex` crate for converting bytes to a hexadecimal string.
    ///
    /// # Example Output
    ///
    /// ```text
    /// Block hash: a3f5e1b2d4c6e7f890123456789abcdef0123456789abcdef0123456789abcdef
    /// ```
    pub fn calculate_hash(&self) -> String {
        let input = format!(
            "{}{}{:?}{}{:?}",
            self.index,
            self.timestamp,
            self.transactions,
            self.previous_hash,
            self.nonce
        );

        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        hex::encode(result)
    }
}