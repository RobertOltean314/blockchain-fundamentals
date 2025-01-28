mod content; 

use content::blockchain::Blockchain;
use content::user::{Transaction, Wallet};

fn main() {
    let mut blockchain = Blockchain::new(4);
    let alice_wallet = Wallet::new();
    let bob_wallet = Wallet::new();

    let mut tx = Transaction::new(
        &alice_wallet.address(),
        &bob_wallet.address(),
        1.0
    );

    let tx_hash = tx.hash();
    let signature = alice_wallet.sign(&tx_hash);
    tx.signature = hex::encode(signature.serialize_der().as_ref());

    blockchain.mempool.push(tx);
    blockchain.mine_pending_transactions(&alice_wallet.address());

    println!("Blockchain: {:#?}", blockchain);
}