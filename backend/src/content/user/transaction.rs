use serde::{Serialize, Deserialize};
use sha2::Digest;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: f64,
    pub fee: f64,
    pub signature: String,
}

impl Transaction {
    pub fn new(sender: &str, receiver: &str, amount: f64, fee: f64) -> Self {
        Transaction {
            sender: sender.to_string(),
            receiver: receiver.to_string(),
            amount,
            fee,
            signature: String::new(),
        }
    }
    /// Computes the SHA-256 hash of the transaction's essential data.
    ///
    /// This function generates a unique hash for the transaction by concatenating its key fields 
    /// (`sender`, `receiver`, `amount`, and `fee`) into a single string. It then computes the 
    /// SHA-256 hash of this string and returns the result as a vector of bytes.
    ///
    /// # Returns
    ///
    /// * `Vec<u8>` - A `Vec<u8>` representing the transaction's SHA-256 hash. The returned vector
    ///   contains the raw bytes of the computed hash.
    ///
    /// # Example
    ///
    /// ```
    /// let transaction_hash = transaction.hash();
    /// println!("Transaction hash: {:?}", transaction_hash);
    /// ```
    ///
    /// # Process
    ///
    /// 1. Concatenates the following fields into a formatted string:
    ///     - `sender`: The sender's address.
    ///     - `receiver`: The receiver's address.
    ///     - `amount`: The amount of the transaction.
    ///     - `fee`: The transaction fee.
    /// 2. Converts the concatenated string into bytes.
    /// 3. Passes the byte array through the SHA-256 hashing algorithm.
    /// 4. Returns the resulting hash as a vector of bytes.
    ///
    /// # Notes
    ///
    /// - This function does **not** include any additional metadata (like timestamp) or digital signature 
    ///   in the hash. You might want to include those if relevant to your system.
    /// - The resulting hash is deterministic: the same inputs will always result in the same hash.
    ///
    /// # Dependencies
    ///
    /// - Uses the `sha2` crate for SHA-256 hashing.
    pub fn hash(&self) -> Vec<u8> {
        let data = format!("{}{}{}{}", self.sender, self.receiver, self.amount, self.fee);
        sha2::Sha256::digest(data.as_bytes()).to_vec()
    }
}