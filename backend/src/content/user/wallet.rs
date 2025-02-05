use secp256k1::{Secp256k1, SecretKey, PublicKey, Message, ecdsa::Signature};
use secp256k1::rand::rngs::OsRng;
use sha2::{Sha256, Digest};

use crate::content::blockchain::Blockchain;

use super::Transaction;

#[derive(Debug,  Clone)]
pub struct Wallet {
    secret_key: SecretKey, 
    pub public_key: PublicKey,
    pub is_miner: bool
}

impl Wallet {
    pub fn new(is_miner: bool) -> Self {
        let secp = Secp256k1::new();
        let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);
        Wallet { secret_key, public_key, is_miner }
    }

    pub fn address(&self) -> String {
        hex::encode(self.public_key.serialize())
    }

    /// Signs the given data using the private key of the user.
    ///
    /// This function signs the provided data using the `Secp256k1` elliptic curve, commonly used in blockchain systems.
    /// It hashes the data using SHA-256 and then signs the resulting hash with the user's private key.
    ///
    /// # Arguments
    ///
    /// * `data` - A byte slice (`&[u8]`) representing the data to be signed. The data can be any sequence of bytes.
    ///
    /// # Returns
    ///
    /// * `Signature` - The ECDSA (Elliptic Curve Digital Signature Algorithm) signature for the data. 
    ///   The signature is returned as a `Signature` object, which can be used for verification.
    ///
    /// # Example
    ///
    /// ```
    /// let data = b"Some important transaction data";
    /// let signature = user.sign(data);
    /// println!("Signed data: {:?}", signature);
    /// ```
    ///
    /// # Process
    ///
    /// 1. The input data is hashed using the SHA-256 hashing algorithm.
    /// 2. The resulting hash is converted into a `Message` type required by the `Secp256k1` library.
    /// 3. The `Secp256k1` elliptic curve is used to sign the `Message` with the user's private key.
    /// 4. The signature is returned as a `Signature` object.
    ///
    /// # Notes
    ///
    /// - This function assumes the user has a private key (`secret_key`) available for signing.
    /// - The `Secp256k1` curve is widely used in cryptocurrencies like Bitcoin and Ethereum for signing transactions.
    /// - The resulting `Signature` can be used for verifying the authenticity of the signed data using the corresponding public key.
    ///
    /// # Dependencies
    ///
    /// - Uses the `secp256k1` crate for elliptic curve operations.
    /// - Uses the `sha2` crate for the SHA-256 hashing algorithm.
    pub fn sign(&self, data: &[u8]) -> Signature {
        let secp = Secp256k1::new();
        let hash = Sha256::digest(data);
        let message = Message::from_digest(hash.into());
        secp.sign_ecdsa(&message, &self.secret_key)
    }

    /// Sends money from the sender's wallet to a receiver, including a transaction fee.
    ///
    /// This function facilitates the transfer of funds between two wallets, ensuring that the sender has 
    /// enough balance to cover the transaction amount and the associated fee. The transaction is then 
    /// signed by the sender and added to the blockchain's mempool for later mining.
    ///
    /// # Arguments
    ///
    /// * `receiver` - A reference to the `Wallet` of the recipient, who will receive the funds.
    /// * `amount` - A `f64` value representing the amount to send from the sender to the receiver.
    /// * `blockchain` - A mutable reference to the `Blockchain` instance, which manages the transactions.
    ///
    /// # Returns
    ///
    /// * `Result<(), String>` - Returns `Ok(())` if the transaction is successfully created and added to the mempool.
    ///   If the sender has insufficient funds, it returns an error with a string message.
    ///
    /// # Example
    ///
    /// ```
    /// let result = sender.send_money(&receiver, 50.0, &mut blockchain);
    /// match result {
    ///     Ok(_) => println!("Transaction successful"),
    ///     Err(err) => println!("Error: {}", err),
    /// }
    /// ```
    ///
    /// # Process
    ///
    /// 1. The function calculates a 1% fee on the transaction amount.
    /// 2. It checks if the sender has enough balance to cover the transaction amount and the fee.
    /// 3. If the balance is insufficient, it returns an error with the sender's address and available funds.
    /// 4. A `Transaction` is created with the sender's address, receiver's address, the amount, and the fee.
    /// 5. The transaction is hashed and signed using the sender's private key.
    /// 6. The transaction is serialized and added to the blockchain's mempool for future mining.
    /// 7. If the sender is a miner, a message is printed indicating that the transaction is in the mining pool (though mining is not triggered in this function).
    ///
    /// # Notes
    ///
    /// - This function assumes the `Blockchain`'s `get_balance` method returns the balance for a given address.
    /// - The `Transaction` includes the fee (1% of the amount), which is deducted from the sender's balance.
    /// - If the sender is a miner, it simulates the action of adding the transaction to the mining pool without immediately mining.
    /// - The transaction is added to the `mempool`, but mining is disabled by default in this method for all wallets.
    /// 
    /// # Dependencies
    ///
    /// - Uses the `Transaction` and `Blockchain` structures to manage the transaction and blockchain state.
    /// - Utilizes the `sign` method to sign the transaction, ensuring its authenticity.
    pub fn send_money(&self, receiver: &Wallet, amount: f64, blockchain: &mut Blockchain) -> Result<(), String> {
        let fee = amount * 0.01;

        let sender_balance = blockchain.get_balance(&self.address());
        if sender_balance < amount + fee {
            return Err(format!("Address: {} does not have enough funds", self.address()).to_string());
        }

        let mut tx = Transaction::new(
            &self.address(),
            &receiver.address(),
            amount,
            fee
        );

        let tx_hash = tx.hash();
        let signature = self.sign(&tx_hash);
        tx.signature = hex::encode(signature.serialize_der().as_ref());

        blockchain.mempool.push(tx);

        // If this wallet is a miner, it might simulate trying to mine after adding a transaction
        if self.is_miner {
            // Here, instead of mining, we could simulate the miner adding this transaction to their pool for later mining
            println!("Miner {} added transaction to mining pool", self.address());
        }

        Ok(())
    }

}