mod utility;
mod content;

use axum::http;
use tower_http::cors::{CorsLayer, Any};
use http::header::CONTENT_TYPE; // Importă HeaderName și CONTENT_TYPE
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use content::{blockchain::Blockchain, user::Wallet};
use utility::{app_router, AppState};

#[tokio::main]
async fn main() {
    let app_state = AppState {
        blockchain: Arc::new(Mutex::new(Blockchain::new(1))),
        alice_wallet: Wallet::new(false),
        bob_wallet: Wallet::new(false),
        miner_wallet1: Wallet::new(true),
        miner_wallet2: Wallet::new(true),
        user_wallets: Arc::new(Mutex::new(HashMap::new())), // Initialize user_wallets as empty
    };

    // Set up routes using the app_router function
    let app = app_router(app_state)
        .layer(
            CorsLayer::new()
                .allow_origin(Any) // Permite toate origin-urile pentru cereri CORS
                .allow_headers(vec![CONTENT_TYPE]), // Permite header-ul Content-Type folosind HeaderName
        );

    // Start the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    println!("Server running on http://localhost:3000");
    axum::serve(listener, app)
        .await
        .unwrap();
}
