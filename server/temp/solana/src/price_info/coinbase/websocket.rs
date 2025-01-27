use std::collections::HashMap;
use std::sync::Arc;
use state::InitCell;
use tokio::sync::RwLock;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use serde::{Deserialize, Serialize};
use futures_util::{SinkExt, StreamExt};
use url::Url;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use anyhow::{Result, anyhow};
use chrono::Utc;
use base64::{engine::general_purpose::STANDARD as Base64Engine, Engine};

use crate::price_info::coinbase::channel_messages::{ChannelMessage, FullChannelMessage, TickerMessage};

type HmacSha256 = Hmac<Sha256>;


#[derive(Debug, Serialize, Deserialize)]
pub struct CoinbaseWebsocket {
    pub url: String,
    pub product_ids: Vec<String>,
    pub channels: Vec<String>,
    pub api_key: String,
    pub secret_key: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Channel {
    name: String,
    product_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct AuthenticatedSubscribeMessage {
    r#type: String,
    channels: Vec<Channel>,
    signature: String,
    key: String,
    timestamp: String,
}


impl CoinbaseWebsocket {
    pub fn new(
        url: &str,
        product_ids: Vec<&str>,
        channels: Vec<&str>,
        api_key: &str,
        secret_key: &str,
    ) -> Self {
        Self {
            url: url.to_string(),
            product_ids: product_ids.into_iter().map(|s| s.to_string()).collect(),
            channels: channels.into_iter().map(|s| s.to_string()).collect(),
            api_key: api_key.to_string(),
            secret_key: secret_key.to_string(),
        }
    }

    pub async fn start() -> Result<()> {
        let coinbase_api_key = std::env::var("COINBASE_API_KEY")?;
        let coinbase_secret_key = std::env::var("COINBASE_SECRET_KEY")?;

        let coinbase_websocket = CoinbaseWebsocket::new(
            "wss://ws-feed.exchange.coinbase.com",
            vec!["SOL-USD"],
            vec!["ticker"],
            &coinbase_api_key,
            &coinbase_secret_key,
        );

        loop {
            match coinbase_websocket.connect_and_subscribe().await {
                Ok(_) => {
                    eprintln!("WebSocket connection closed gracefully. Reconnecting...");
                    // Sleep briefly before reconnecting
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                }
                Err(e) => {
                    let retry_seconds = 5;
                    eprintln!(
                        "WebSocket connection failed: {}. Retrying in {} seconds...",
                        e, retry_seconds
                    );
                    tokio::time::sleep(std::time::Duration::from_secs(retry_seconds)).await;
                }
            }
        }
    }

    fn generate_signature(&self, timestamp: &str, request_path: &str) -> Result<String> {
        let message = format!("{}GET{}", timestamp, request_path);
        let decoded_secret = Base64Engine.decode(&self.secret_key)
            .map_err(|_| anyhow!("Failed to decode secret key"))?;
        let mut mac = HmacSha256::new_from_slice(&decoded_secret)
            .map_err(|_| anyhow!("Invalid secret key"))?;
        mac.update(message.as_bytes());
        let signature = mac.finalize().into_bytes();
        Ok(Base64Engine.encode(signature))
    }

    pub async fn connect_and_subscribe(&self) -> Result<()> {
        let url = Url::parse(&self.url)?;
        let timestamp = Utc::now().timestamp().to_string();
        let signature = self.generate_signature(&timestamp, "/users/self/verify")?;

        let channels: Vec<Channel> = self
            .channels
            .iter()
            .map(|name| Channel {
                name: name.clone(),
                product_ids: self.product_ids.clone(),
            })
            .collect();

        let subscribe_msg = AuthenticatedSubscribeMessage {
            r#type: "subscribe".to_string(),
            channels,
            signature,
            key: self.api_key.clone(),
            timestamp,
        };

        let (ws_stream, _) = connect_async(url.as_str()).await?;
        println!("Connected to Coinbase WebSocket!");
        let (mut write, mut read) = ws_stream.split();

        let msg = serde_json::to_string(&subscribe_msg)?;
        write.send(Message::Text(msg.into())).await?;
        println!("Subscribed to channels: {:?}", self.channels);


        while let Some(Ok(msg)) = read.next().await {
            if let Message::Text(text) = msg {
                // println!("Raw WebSocket Message: {}", text);
                match serde_json::from_str::<ChannelMessage>(&text) {
                    Ok(message) => {
                        match &*message.r#type {
                            "ticker" => {
                                let ticker_message = serde_json::from_str::<TickerMessage>(&text)?;
                                // println!("Ticker Message: {:?}", ticker_message);
                                let ticker_state = ticker_message.to_ticker_state()?;
                                match ticker_state.update() {
                                    Ok(_) => {}
                                    Err(e) => {
                                        eprintln!("Failed to update ticker state: {}", e);
                                    }
                                }
                            }
                            _ => {}
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
