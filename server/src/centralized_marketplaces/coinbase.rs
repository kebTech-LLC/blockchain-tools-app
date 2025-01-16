use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH, Instant};
use tokio::sync::RwLock;
use tokio::time::{interval_at, sleep_until, Instant as TokioInstant};
use serde::{Deserialize, Serialize};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use futures_util::{StreamExt, SinkExt};
use url::Url;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use base64::{encode as b64_encode, decode as b64_decode};
use anyhow::{Result, anyhow};

type HmacSha256 = Hmac<Sha256>;

pub static ORDER_BOOK: InitCell<Arc<RwLock<OrderBookState>>> = InitCell::new();

#[derive(Debug, Clone, Default)]
pub struct OrderBookState {
    pub bids: HashMap<String, f64>, // price -> size
    pub asks: HashMap<String, f64>, // price -> size
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Coinbase {
    pub url: String,
    pub product_ids: Vec<String>,
    pub channels: Vec<String>,
    pub api_key: String,
    pub secret_key: String,
    pub passphrase: String,
}

#[derive(Serialize, Deserialize)]
struct AuthenticatedSubscribeMessage {
    r#type: String,
    product_ids: Vec<String>,
    channels: Vec<String>,
    signature: String,
    key: String,
    passphrase: String,
    timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct L2UpdateMessage {
    r#type: String,
    product_id: String,
    changes: Vec<(String, String, String)>, // (side, price, size)
    time: String,
}

impl Coinbase {
    pub fn new(
        url: &str,
        product_ids: Vec<&str>,
        channels: Vec<&str>,
        api_key: &str,
        secret_key: &str,
        passphrase: &str,
    ) -> Self {
        Coinbase {
            url: url.to_string(),
            product_ids: product_ids.into_iter().map(|s| s.to_string()).collect(),
            channels: channels.into_iter().map(|s| s.to_string()).collect(),
            api_key: api_key.to_string(),
            secret_key: secret_key.to_string(),
            passphrase: passphrase.to_string(),
        }
    }

    fn generate_signature(&self, timestamp: &str, request_path: &str) -> Result<String> {
        let message = format!("{}GET{}", timestamp, request_path);
        let decoded_secret = b64_decode(&self.secret_key)
            .map_err(|_| anyhow!("Failed to decode secret key"))?;
        let mut mac = HmacSha256::new_from_slice(&decoded_secret)
            .map_err(|_| anyhow!("Invalid secret key"))?;
        mac.update(message.as_bytes());
        let signature = mac.finalize().into_bytes();
        Ok(b64_encode(signature))
    }

    pub async fn connect_and_subscribe(&self) -> Result<()> {
        let url = Url::parse(&self.url)?;
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_secs()
            .to_string();
        let signature = self.generate_signature(&timestamp, "/users/self/verify")?;

        let subscribe_msg = AuthenticatedSubscribeMessage {
            r#type: "subscribe".to_string(),
            product_ids: self.product_ids.clone(),
            channels: self.channels.clone(),
            signature,
            key: self.api_key.clone(),
            passphrase: self.passphrase.clone(),
            timestamp,
        };

        let (ws_stream, _) = connect_async(url).await?;
        println!("Connected to Coinbase WebSocket!");
        let (mut write, mut read) = ws_stream.split();

        // Send subscription message
        let msg = serde_json::to_string(&subscribe_msg)?;
        write.send(Message::Text(msg)).await?;
        println!("Subscribed to channels: {:?}", self.channels);

        let order_book = Arc::new(RwLock::new(OrderBookState::default()));
        ORDER_BOOK.set(order_book.clone());

        // Start a loop aligned to seconds
        let now = Instant::now();
        let start_time = now + Duration::from_secs(1) - Duration::from_millis(now.elapsed().as_millis() % 1000);
        let mut interval = interval_at(TokioInstant::from_std(start_time), Duration::from_secs(1));

        tokio::spawn(async move {
            loop {
                interval.tick().await;

                // Snapshot the order book state
                let snapshot = {
                    let lock = order_book.read().await;
                    lock.clone()
                };

                // Process snapshot as needed
                println!("Order Book Snapshot (Bids: {}, Asks: {})", snapshot.bids.len(), snapshot.asks.len());
            }
        });

        // Read WebSocket messages
        while let Some(Ok(msg)) = read.next().await {
            if let Message::Text(text) = msg {
                if let Ok(update) = serde_json::from_str::<L2UpdateMessage>(&text) {
                    let mut lock = order_book.write().await;
                    for (side, price, size) in update.changes {
                        let price = price.parse::<f64>().unwrap_or(0.0);
                        let size = size.parse::<f64>().unwrap_or(0.0);

                        if side == "buy" {
                            if size == 0.0 {
                                lock.bids.remove(&price.to_string());
                            } else {
                                lock.bids.insert(price.to_string(), size);
                            }
                        } else if side == "sell" {
                            if size == 0.0 {
                                lock.asks.remove(&price.to_string());
                            } else {
                                lock.asks.insert(price.to_string(), size);
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
