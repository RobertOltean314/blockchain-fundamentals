use serde::{Serialize, Deserialize};
use sha2::Digest;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: f64,
    pub signature: String,  
}

impl Transaction {
    pub fn new(sender: &str, receiver: &str, amount: f64) -> Self {
        Transaction {
            sender: sender.to_string(),
            receiver: receiver.to_string(),
            amount,
            signature: String::new(),
        }
    }

    pub fn hash(&self) -> Vec<u8> {
        let data = format!("{}{}{}", self.sender, self.receiver, self.amount);
        sha2::Sha256::digest(data.as_bytes()).to_vec()
    }
}