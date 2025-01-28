mod content;

use content::blockchain::Blockchain;
use content::user::Wallet;

fn main() {
    // Initialize Blockchain with a lower difficulty for demonstration
    let mut blockchain = Blockchain::new(1);

    // Creating Wallets
    let alice_wallet = Wallet::new(false);
    let bob_wallet = Wallet::new(false);
    let miner_wallet1 = Wallet::new(true);
    let miner_wallet2 = Wallet::new(true);

    // Give Alice some initial funds by mining a block
    blockchain.mine_pending_transactions(&alice_wallet.address());
    println!("Alice received initial mining reward");

    // Simulate transactions between Alice and Bob
    for i in 0..3 {
        if let Err(e) = alice_wallet.send_money(&bob_wallet, 1.0, &mut blockchain) {
            println!("Transaction {} failed: {}", i + 1, e);
        } else {
            println!("Transaction {} from Alice to Bob successful", i + 1);
        }
    }

    // Simulate miners mining blocks
    for _ in 0..2 {
        // Miner 1 mines a block
        blockchain.mine_pending_transactions(&miner_wallet1.address());
        println!("Miner 1 mined a block and received reward");

        // Miner 2 mines a block
        blockchain.mine_pending_transactions(&miner_wallet2.address());
        println!("Miner 2 mined a block and received reward");
    }

    // Print final state
    if blockchain.is_valid() {
        println!("Blockchain: {:#?}", blockchain);
        let wallets = vec![&alice_wallet, &bob_wallet, &miner_wallet1, &miner_wallet2];
        for wallet in &wallets {
            println!("{}'s balance: {}", wallet.address(), blockchain.get_balance(&wallet.address()));
        }
    } else {
        println!("Blockchain is not valid.");
    }
}