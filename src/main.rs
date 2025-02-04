use axum::{routing::{get, post}, Router};
use content::{blockchain::Blockchain, user::Wallet};
use std::sync::Mutex;
use utility::{mine_initial_block, print_final_state, simulate_mining, simulate_transactions, AppState};

mod content;
mod utility;

#[tokio::main]
async fn main() {
    let app_state = AppState {
        blockchain: Mutex::new(Blockchain::new(1)).into(),
        alice_wallet: Wallet::new(false),
        bob_wallet: Wallet::new(false),
        miner_wallet1: Wallet::new(true),
        miner_wallet2: Wallet::new(true),
    };

    // Set up routes using AppState
    let app = Router::new()
    .route("/mine/initial", post(mine_initial_block))
    .route("/transactions/simulate", post(simulate_transactions))
    .route("/mine/simulate", post(simulate_mining))
    .route("/blockchain/status", get(print_final_state))
    .with_state(app_state);

    // Start the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    println!("Server running on http://localhost:3000");
    axum::serve(listener, app)
        .await
        .unwrap();
}

