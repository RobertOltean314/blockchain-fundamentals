use crate::content::blockchain::Blockchain;
use crate::content::user::Wallet;
use std::sync::{Arc, Mutex};
use axum::extract::State;
use axum::Json;
use serde_json::json;

#[derive(Clone)]
pub struct AppState {
    pub blockchain: Arc<Mutex<Blockchain>>,
    pub alice_wallet: Wallet,
    pub bob_wallet: Wallet,
    pub miner_wallet1: Wallet,
    pub miner_wallet2: Wallet,
}

pub async fn mine_initial_block(State(state): State<AppState>) -> Json<serde_json::Value> {
    let mut blockchain = state.blockchain.lock().unwrap();
    blockchain.mine_pending_transactions(&state.alice_wallet.address());
    Json(json!({"message": "Alice received initial mining reward"}))
}

pub async fn simulate_transactions(State(state): State<AppState>) -> Json<serde_json::Value> {
    let mut blockchain = state.blockchain.lock().unwrap();
    let mut results = Vec::new();

    for i in 0..3 {
        match state.alice_wallet.send_money(&state.bob_wallet, 1.0, &mut blockchain) {
            Ok(_) => results.push(format!("Transaction {} from Alice to Bob successful", i + 1)),
            Err(e) => results.push(format!("Transaction {} failed: {}", i + 1, e)),
        }
    }

    Json(json!({"transactions": results}))
}

pub async fn simulate_mining(State(state): State<AppState>) -> Json<serde_json::Value> {
    let mut blockchain = state.blockchain.lock().unwrap();
    let mut results = Vec::new();

    for _ in 0..2 {
        blockchain.mine_pending_transactions(&state.miner_wallet1.address());
        results.push("Miner 1 mined a block and received reward".to_string());

        blockchain.mine_pending_transactions(&state.miner_wallet2.address());
        results.push("Miner 2 mined a block and received reward".to_string());
    }

    Json(json!({"mining": results}))
}

pub async fn print_final_state(State(state): State<AppState>) -> Json<serde_json::Value> {
    let blockchain = state.blockchain.lock().unwrap();
    let mut balances = Vec::new();

    if blockchain.is_valid() {
        let wallets = vec![
            (&state.alice_wallet, "Alice"),
            (&state.bob_wallet, "Bob"),
            (&state.miner_wallet1, "Miner 1"),
            (&state.miner_wallet2, "Miner 2"),
        ];

        for (wallet, name) in wallets {
            balances.push(json!({
                "name": name,
                "address": wallet.address(),
                "balance": blockchain.get_balance(&wallet.address())
            }));
        }

        Json(json!({
            "status": "Blockchain is valid",
            "balances": balances
        }))
    } else {
        Json(json!({"status": "Blockchain is not valid."}))
    }
}