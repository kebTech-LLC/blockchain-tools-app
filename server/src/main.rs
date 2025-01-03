use std::{env, sync::Arc, thread};
use centralized_marketplaces::coinbase::Coinbase;
use cnctd_server::{
    server::{CnctdServer, ServerConfig},
    socket::SocketConfig,
};
use local_ip_address::local_ip;
use router::{rest::RestRouter, socket::SocketRouter};
use solana::{pool_manager::PoolManager, wallet::Wallet};
use solana_pools::SolanaPools;
use solana_sdk::signer::Signer;
// use session::client_session::ClientSession;

pub mod router;
pub mod external_apis;
pub mod centralized_marketplaces;
pub mod solana_pools;
// pub mod db;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    // Load secrets and environment variables
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET not set");
    let server_id = env::var("SERVER_ID").unwrap_or_else(|_| "1".to_string());
    let port_str = env::var("SERVER_PORT").unwrap_or_else(|_| "5050".to_string());
    let client_dir = env::var("CLIENT_DIR").ok();
    let jwt_secret_bytes = jwt_secret.as_bytes().to_owned();

    // Initialize shared components
    router::rest::JWT_SECRET.set(jwt_secret.into());
    let rest_router = RestRouter;
    let socket_router = SocketRouter;

    // Allowed origins for CORS
    let ip_address = local_ip().map(|ip| ip.to_string()).unwrap_or_else(|_| "127.0.0.1".to_string());
    // let allowed_origins = vec![
    //     "http://localhost:3000".to_string(),
    //     "https://example.com".to_string(),
    //     format!("http://{}:{}", ip_address, port_str),
    // ];

    // Server configuration
    let server_config = ServerConfig::new(
        &server_id,
        &port_str,
        client_dir,
        rest_router,
        Some(10),                  // Maximum concurrent connections
        None,     // Allowed CORS origins
        None,                      // Optional TLS config
    );

    // Socket configuration
    let socket_config = SocketConfig::new(
        socket_router,
        Some(jwt_secret_bytes),    // Secret for WebSocket auth
        None,    
        None,                  // Optional Redis URL
        // Some(Arc::new(|client_info| {
        //     let client_info_clone = client_info.clone();
        //     tokio::spawn(async move {
        //         let mut session = ClientSession::new(client_info_clone);
        //         if let Err(e) = session.disconnect().upload().await {
        //             println!("Error uploading session: {:?}", e);
        //         } else {
        //             println!("Session successfully uploaded.");
        //         }
        //     });
        // })),
    );

    // Start periodic session uploads
    // ClientSession::start_periodic_upload().await;

    // Initialize databases
    // if let Err(e) = db::DB::start().await {
    //     println!("Database error: {:?}", e);
    // } else {
    //     println!("Database initialized.");
    // }
    // PoolManager::get_orca_sol_usdc_pool().await.expect("Failed to get Orca SOL/USDC pool");
    PoolManager::open_sol_usdc_position().await.expect("Failed to open SOL/USDC position");

    SolanaPools::get_sol_balance().await.expect("Failed to get SOL balance");

    let wallet = Wallet::get_public_key().await.unwrap();
    PoolManager::get_orca_positions_for_wallet("312yxT6PFcauztXCfG5jNqcRXqMDCm9HeLBJwbaHL6kH").await.expect("Failed to get Orca positions for wallet");

    let coinbase = Coinbase::new(
        "wss://ws-feed.exchange.coinbase.com",
        vec!["SOL-USD"],
        vec!["ticker"],
    );

    // Use tokio::spawn to manage the WebSocket connection
    // tokio::spawn(async move {
    //     coinbase.connect_and_subscribe().await;
    // });
    
   

    // Start the server
    if let Err(e) = CnctdServer::start(server_config, Some(socket_config)).await {
        println!("Server error: {:?}", e);
    } else {
        println!("Server started successfully.");
    }
}
