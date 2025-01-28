mod content;

use content::blockchain::Blockchain;
use content::user::Wallet;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

fn main() {
    // Initialize Blockchain
    let blockchain = Arc::new(Mutex::new(Blockchain::new(4)));

    // Creating some Dummy Wallets
    let alice_wallet = Wallet::new(false);
    let bob_wallet = Wallet::new(false);
    let miner_wallet1 = Wallet::new(true);
    let miner_wallet2 = Wallet::new(true); 

    // Give Alice some initial funds by mining a block
    let mut bc = blockchain.lock().unwrap();
    bc.mine_pending_transactions(&alice_wallet.address());

    // Simulate transactions in a separate thread to avoid blocking
    let blockchain_clone = Arc::clone(&blockchain);
    let alice_clone = alice_wallet.clone();
    let bob_clone = bob_wallet.clone();
    let transaction_thread = thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_secs(2));
            let mut bc = blockchain_clone.lock().unwrap();
            alice_clone.send_money(&bob_clone, 1.0, &mut bc).unwrap();
            println!("Transaction added to mempool");
        }
    });

    // Simulate multiple miners competing
    let miner1 = miner_wallet1.clone();
    let miner2 = miner_wallet2.clone();
    let blockchain_clone1 = Arc::clone(&blockchain);
    let blockchain_clone2 = Arc::clone(&blockchain);

    let miner_thread1 = thread::spawn(move || {
        loop {
            let mut bc = blockchain_clone1.lock().unwrap();
            if !bc.mempool.is_empty() {
                bc.mine_pending_transactions(&miner1.address());
                println!("Miner 1 mined a block");
            }
            // Release the lock immediately after mining or checking
            drop(bc);
            thread::sleep(Duration::from_secs(1));
        }
    });

    let miner_thread2 = thread::spawn(move || {
        loop {
            let mut bc = blockchain_clone2.lock().unwrap();
            if !bc.mempool.is_empty() {
                bc.mine_pending_transactions(&miner2.address());
                println!("Miner 2 mined a block");
            }
            // Release the lock immediately after mining or checking
            drop(bc);
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Wait for the transaction thread to finish
    transaction_thread.join().unwrap();

    // Let the miners run for a while
    thread::sleep(Duration::from_secs(20));

    // Stop the miners
    miner_thread1.join().unwrap();
    miner_thread2.join().unwrap();

    // Print final state
    let bc = blockchain.lock().unwrap();
    if bc.is_valid() {
        println!("Blockchain: {:#?}", bc);
        let wallets = vec![alice_wallet, bob_wallet, miner_wallet1, miner_wallet2];
        for wallet in &wallets {
            println!("{}'s balance: {}", wallet.address(), bc.get_balance(&wallet.address()));
        }
    } else {
        println!("Blockchain is not valid.");
    }
}