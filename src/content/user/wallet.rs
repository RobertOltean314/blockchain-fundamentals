use secp256k1::{Secp256k1, SecretKey, PublicKey, Message, ecdsa::Signature};
use secp256k1::rand::rngs::OsRng;
use sha2::{Sha256, Digest};

#[derive(Debug)]
pub struct Wallet {
    secret_key: SecretKey, 
    pub public_key: PublicKey,
}

impl Wallet {
    pub fn new() -> Self {
        let secp = Secp256k1::new();
        let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);
        Wallet { secret_key, public_key }
    }

    pub fn address(&self) -> String {
        hex::encode(self.public_key.serialize())
    }

    pub fn sign(&self, data: &[u8]) -> Signature {
        let secp = Secp256k1::new();
        let hash = Sha256::digest(data);
        let message = Message::from_digest(hash.into());
        secp.sign_ecdsa(&message, &self.secret_key)
    }
}