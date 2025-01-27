use serde::{Serialize, Deserialize};
use serde_json::Value;

use super::PoolManager;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    UpdatePosition,
    RemovePosition,
    Stats,
}

impl MessageType {
    pub fn to_string(&self) -> String {
        match self {
            MessageType::UpdatePosition => "update-position".to_string(),
            MessageType::RemovePosition => "remove-position".to_string(),
            MessageType::Stats => "stats".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolManagerMessage {
    pub message_type: MessageType,
    pub data: Option<Value>,
    pub frequency_seconds: u64,
}

impl PoolManagerMessage {
    pub fn new(message_type: MessageType, data: Option<Value>, frequency_seconds: u64) -> Self {
        PoolManagerMessage {
            message_type,
            data,
            frequency_seconds,
        }
    }

    pub async fn add_to_queue(&self) -> anyhow::Result<()> {
        PoolManager::add_to_message_queue(self.clone()).await
    }
}