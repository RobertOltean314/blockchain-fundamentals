mod content;

use content::blockchain::Blockchain;
use content::user::Wallet;

fn main() {
    let mut blockchain = Blockchain::new(4);
    let alice_wallet = Wallet::new();
    let bob_wallet = Wallet::new();

    // First, we'll give Alice some money by mining a reward to her
    blockchain.mine_pending_transactions(&alice_wallet.address());

    // Now Alice can try to send money
    match alice_wallet.send_money(&bob_wallet, 1.0, &mut blockchain) {
        Ok(_) => println!("Transaction successful"),
        Err(e) => println!("Error: {}", e),
    }

    // Trying to send more than she has
    match alice_wallet.send_money(&bob_wallet, 100.0, &mut blockchain) {
        Ok(_) => println!("Transaction successful"),
        Err(e) => println!("Error: {}", e),
    }

    if blockchain.is_valid() {
        //println!("Blockchain:a {:#?}", blockchain);
        println!("Alice's balance: {}", blockchain.get_balance(&alice_wallet.address()));
        println!("Bob's balance: {}", blockchain.get_balance(&bob_wallet.address()));
    } else {
        println!("Blockchain is not valid.");
    }
}
