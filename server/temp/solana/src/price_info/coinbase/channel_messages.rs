use serde::{Deserialize, Serialize};

use super::ticker::TickerState;


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum FullChannelMessage {
    #[serde(rename = "received")]
    Received {
        product_id: String,
        order_id: String,
        side: String,
        price: Option<String>,
        size: Option<String>,
    },
    #[serde(rename = "open")]
    Open {
        product_id: String,
        order_id: String,
        side: String,
        price: String,
        remaining_size: String,
    },
    #[serde(rename = "done")]
    Done {
        product_id: String,
        order_id: String,
        side: String,
        price: Option<String>,
        reason: String,
    },
    #[serde(rename = "match")]
    Match {
        product_id: String,
        price: String,
        size: String,
        side: String,
    },
    #[serde(rename = "change")]
    Change {
        product_id: String,
        order_id: String,
        side: String,
        price: Option<String>,
        new_size: Option<String>,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TickerMessage {
    pub r#type: String,
    pub sequence: u64,
    pub product_id: String,
    pub price: String,
    pub open_24h: String,
    pub volume_24h: String,
    pub low_24h: String,
    pub high_24h: String,
    pub volume_30d: String,
    pub best_bid: String,
    pub best_bid_size: String,
    pub best_ask: String,
    pub best_ask_size: String,
    pub side: String,
    pub time: String,
    pub trade_id: u64,
    pub last_size: String,
}

impl TickerMessage {
    pub fn to_ticker_state(&self) -> anyhow::Result<TickerState> {
        let price = self.price.parse::<f64>().map_err(|_| anyhow::anyhow!("Failed to parse price"))?;
        Ok(TickerState::from_iso8601(price, &self.time)?)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChannelMessage {
    pub r#type: String,
}