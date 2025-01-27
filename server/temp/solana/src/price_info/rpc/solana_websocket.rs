use std::collections::HashMap;
use std::sync::Arc;
use base64::Engine;
use state::InitCell;
use tokio::sync::RwLock;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use serde::{Deserialize, Serialize};
use futures_util::{SinkExt, StreamExt};
use tokio::time::{interval_at, Instant};
use serde_json::json;
use anyhow::Result;

pub static ACCOUNT_UPDATES: InitCell<Arc<RwLock<HashMap<String, AccountUpdate>>>> = InitCell::new();

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountUpdate {
    pub lamports: u64,
    pub owner: String,
    pub data: String,
    pub executable: bool,
    #[serde(rename = "rentEpoch")]
    pub rent_epoch: u64,
    pub space: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SolanaWebSocket {
    pub url: String,
    pub accounts: Vec<String>,
}

impl SolanaWebSocket {
    pub fn new(url: &str, accounts: Vec<&str>) -> Self {
        SolanaWebSocket {
            url: url.to_string(),
            accounts: accounts.into_iter().map(|s| s.to_string()).collect(),
        }
    }

    pub async fn connect_and_subscribe(&self) -> Result<()> {
        let (ws_stream, _) = connect_async(&self.url).await?;
        println!("Connected to Solana WebSocket!");
        let (mut write, mut read) = ws_stream.split();

        // Initialize the account updates storage
        let account_updates = Arc::new(RwLock::new(HashMap::new()));
        ACCOUNT_UPDATES.set(account_updates.clone());

        // Subscribe to accounts
        for account in &self.accounts {
            let subscription_message = json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "accountSubscribe",
                "params": [account, {"encoding": "base64"}]
            });
            let msg = serde_json::to_string(&subscription_message)?;
            write.send(Message::Text(msg.into())).await?;
            println!("Subscribed to account: {}", account);
        }

        // Spawn a task to log snapshots every second
        let account_updates_clone = account_updates.clone();
        tokio::spawn(async move {
            let start_time = Instant::now();
            let mut interval = interval_at(start_time, tokio::time::Duration::from_secs(1));
            loop {
                interval.tick().await;

                let snapshot = {
                    let lock = account_updates_clone.read().await;
                    lock.clone()
                };

                println!(
                    "Account Snapshot: {} accounts tracked",
                    snapshot.len()
                );
            }
        });

        // Read WebSocket messages
        while let Some(Ok(msg)) = read.next().await {
            if let Message::Text(text) = msg {
                println!("Raw WebSocket Message: {}", text);
                match serde_json::from_str::<AccountNotification>(&text) {
                    Ok(notification) => {
                        let mut lock = account_updates.write().await;
                        if let Some(value) = notification.params.result.value {
                            let account_update = AccountUpdate {
                                lamports: value.lamports,
                                owner: value.owner,
                                data: value.data.0.clone(),
                                executable: value.executable,
                                rent_epoch: value.rent_epoch,
                                space: value.space,
                            };
                            println!("\n\ndata decoded: {:?}\n\n", base64::engine::general_purpose::STANDARD.decode(&value.data.0).unwrap());
                            lock.insert(notification.params.subscription.to_string(), account_update);
                        } else {
                            eprintln!("Account value is None for subscription ID: {}", notification.params.subscription);
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to parse message: {} | Raw: {}", e, text);
                    }
                }
            }
        }

        Ok(())
    }
    
}

#[derive(Debug, Serialize, Deserialize)]
struct AccountNotification {
    jsonrpc: String,
    method: String,
    params: AccountParams,
}

#[derive(Debug, Serialize, Deserialize)]
struct AccountParams {
    result: AccountResult,
    subscription: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct AccountResult {
    context: AccountContext,
    value: Option<AccountValue>, // Handle cases where the value might be null
}

#[derive(Debug, Serialize, Deserialize)]
struct AccountContext {
    slot: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct AccountValue {
    lamports: u64,
    owner: String,
    data: (String, String), // (data, encoding)
    executable: bool,
    #[serde(rename = "rentEpoch")]
    rent_epoch: u64,
    space: u64,
}
