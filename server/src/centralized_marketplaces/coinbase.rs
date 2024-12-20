use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use futures_util::{StreamExt, SinkExt}; // For split and async operations
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coinbase {
    pub url: String,
    pub product_ids: Vec<String>,
    pub channels: Vec<String>,
}

impl Coinbase {
    pub fn new(url: &str, product_ids: Vec<&str>, channels: Vec<&str>) -> Self {
        Coinbase {
            url: url.to_string(),
            product_ids: product_ids.into_iter().map(|s| s.to_string()).collect(),
            channels: channels.into_iter().map(|s| s.to_string()).collect(),
        }
    }

    pub async fn connect_and_subscribe(&self) {
        let url = Url::parse(&self.url).expect("Invalid WebSocket URL");

        // Convert Url to string and connect to the WebSocket
        let (ws_stream, _) = connect_async(url.to_string()).await.expect("Failed to connect to Coinbase WebSocket");
        println!("Connected to Coinbase WebSocket!");

        let (mut write, mut read) = ws_stream.split();

        // Create subscription message
        let subscribe_msg = serde_json::to_string(&SubscribeMessage {
            r#type: "subscribe".to_string(),
            product_ids: self.product_ids.clone(),
            channels: self.channels.clone(),
        }).expect("Failed to serialize subscribe message");

        // Send subscription message
        write.send(Message::Text(subscribe_msg.into())).await.expect("Failed to send subscribe message");
        println!("Subscribed to channels: {:?}", self.channels);

        // Keep reading messages in a loop
        while let Some(Ok(msg)) = read.next().await {
            if let Message::Text(text) = msg {
                println!("Received: {}", text);
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
struct SubscribeMessage {
    r#type: String,
    product_ids: Vec<String>,
    channels: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TickerMessage {
    r#type: String,
    sequence: u64,
    product_id: String,
    price: String,
    open_24h: String,
    volume_24h: String,
    low_24h: String,
    high_24h: String,
    volume_30d: String,
    best_bid: String,
    best_bid_size: String,
    best_ask: String,
    best_ask_size: String,
    side: String,
    time: String,
    trade_id: u64,
    last_size: String,
}