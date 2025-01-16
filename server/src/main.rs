use std::{env, sync::Arc, thread};
// use centralized_marketplaces::coinbase::Coinbase;
use cnctd_server::{
    router::message::Message, server::{CnctdServer, ServerConfig}, socket::SocketConfig
};
use local_ip_address::local_ip;
use router::{rest::RestRouter, socket::SocketRouter};
use serde_json::json;
use solana::pool_manager::{message::{MessageType, PoolManagerMessage}, PoolManager};
use tokio::sync::mpsc;
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
    // let ip_address = local_ip().map(|ip| ip.to_string()).unwrap_or_else(|_| "127.0.0.1".to_string());
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

    // let coinbase_future = async {
    //     Coinbase::start_sol_websocket().await;
    // };
  
    // Create the channel
    let (tx, rx) = mpsc::channel::<PoolManagerMessage>(100);

    // Prepare a future for the server
    let server_future = async {
        if let Err(e) = CnctdServer::start(server_config, Some(socket_config)).await {
            println!("Server error: {:?}", e);
        } else {
            println!("Server started successfully.");
        }
    };

    // Prepare a future for the PoolManager
    // We pass in `tx.clone()` so we can continue sending even if we retry.
    let pool_manager_future = async move {
        loop {
            match PoolManager::start(
                tx.clone()
            ).await {
                Ok(_) => {
                    println!("PoolManager exited successfully.");
                    // If PoolManager::start() ever returns Ok, we break.
                    break;
                }
                Err(e) => {
                    println!("PoolManager Error: {:?}. Retrying in 30 seconds...", e);
                    tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
                }
            }
        }
    };

    // Prepare a future to receive data from the channel
    let rx_future = async move {
        let mut rx = rx;  // make rx mut in this scope
        while let Some(pool_manager_message) = rx.recv().await {
            println!("Received a new PoolManagerMessage");

            let (channel, instruction) = match pool_manager_message.message_type {
                MessageType::UpdatePosition => ("managed-position", "update"),
                MessageType::RemovePosition => ("managed-position", "remove"),
            };

            let message_data = json!({"data": pool_manager_message.data, "frequency": pool_manager_message.frequency_seconds});
            let message = Message::new(channel, instruction, Some(message_data));
            match message.broadcast().await {
                Ok(_) => println!("Broadcasted managed position successfully."),
                Err(e) => println!("Failed to broadcast managed position: {:?}", e),
            }
        }
    };

    // Now run all three tasks concurrently:
    // - The server will keep running until it returns or errors.
    // - The PoolManager will keep retrying in a loop.
    // - The receiver loop will keep reading until `tx` is dropped.
    tokio::select! {
        // _ = coinbase_future => (),
        _ = server_future => (),
        _ = pool_manager_future => (),
        _ = rx_future => (),
    }

    println!("All tasks have finished, shutting down main.");
}
