mod content;

use content::blockchain::Blockchain;
use content::user::Wallet;
use std::thread;
use std::time::Duration;

fn main() {
    let mut blockchain = Blockchain::new(4);
    let alice_wallet = Wallet::new(false); // Regular wallet
    let bob_wallet = Wallet::new(false); // Regular wallet
    let miner_wallet1 = Wallet::new(true); // Miner wallet
    let miner_wallet2 = Wallet::new(true); // Another miner wallet

    // Give Alice some initial funds by mining a block
    blockchain.mine_pending_transactions(&alice_wallet.address());

    // Simulate transactions
    match alice_wallet.send_money(&bob_wallet, 1.0, &mut blockchain) {
        Ok(_) => println!("Transaction successful"),
        Err(e) => println!("Error: {}", e),
    }

    // Simulate random mining attempts
    let wallets = vec![alice_wallet, bob_wallet, miner_wallet1, miner_wallet2];
    for _ in 0..5 { // Attempt mining 5 times for demonstration
        // Simulate some time passing
        thread::sleep(Duration::from_secs(1));
        blockchain.attempt_random_mining(&wallets);
    }

    if blockchain.is_valid() {
        println!("Blockchain: {:#?}", blockchain);
        for wallet in &wallets {
            println!("{}'s balance: {}", wallet.address(), blockchain.get_balance(&wallet.address()));
        }
    } else {
        println!("Blockchain is not valid.");
    }
}