use std::{env, sync::Arc, thread};
// use centralized_marketplaces::coinbase::Coinbase;
use cnctd_server::{
    router::message::Message, server::{CnctdServer, ServerConfig}, socket::SocketConfig
};
use local_ip_address::local_ip;
use router::{pool_manager_message::route_pool_manager_message, rest::RestRouter, socket::SocketRouter};
use serde_json::json;
use solana::pool_manager::{message::{MessageType, PoolManagerMessage}, PoolManager};
use tokio::sync::mpsc;
// use session::client_session::ClientSession;

pub mod router;
pub mod external_apis;
pub mod centralized_marketplaces;
pub mod solana_pools;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    match blockchain_db::db::DB::start().await {
        Ok(()) => println!("DB started"),
        Err(e) => println!("DB error: {:?}", e),
    }

    // Load secrets and environment variables
   

    // Allowed origins for CORS
    // let ip_address = local_ip().map(|ip| ip.to_string()).unwrap_or_else(|_| "127.0.0.1".to_string());
    // let allowed_origins = vec![
    //     "http://localhost:3000".to_string(),
    //     "https://example.com".to_string(),
    //     format!("http://{}:{}", ip_address, port_str),
    // ];

    // Server configuration


    // Socket configuration


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
        loop {
            let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET not set");
            let google_client_id = env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID not set");
            let server_id = env::var("SERVER_ID").unwrap_or_else(|_| "1".to_string());
            let port_str = env::var("SERVER_PORT").unwrap_or_else(|_| "5050".to_string());
            let jwt_secret_bytes = jwt_secret.as_bytes().to_owned();
            router::rest::JWT_SECRET.set(jwt_secret.into());
            router::rest::GOOGLE_CLIENT_ID.set(google_client_id.into());
            let rest_router = RestRouter;
            let socket_router = SocketRouter;
            let client_dir = env::var("CLIENT_DIR").ok();
            let server_config = ServerConfig::new(
                &server_id,
                &port_str,
                client_dir,
                rest_router,
                Some(10),                  // Maximum concurrent connections
                None,     // Allowed CORS origins
                None,                      // Optional TLS config
            );
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
            if let Err(e) = CnctdServer::start(server_config, Some(socket_config)).await {
                println!("Server error: {:?}", e);
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            } else {
                println!("Server exited. Retrying in 5 seconds...");
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            }
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
                    println!("PoolManager exited successfully. Retrying in 30 seconds...");
                    tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
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
            // println!("Received a new PoolManagerMessage: {:?}", pool_manager_message.message_type);
            route_pool_manager_message(pool_manager_message).await;
           
        }
    };

    // Now run all three tasks concurrently:
    // - All tasks will keep running unless the process exits or panics.
    let (_server_res, _pool_manager_res, _rx_res) = tokio::join!(
        server_future,
        pool_manager_future,
        rx_future,
    );

    println!("All tasks have finished, shutting down main.");
}
