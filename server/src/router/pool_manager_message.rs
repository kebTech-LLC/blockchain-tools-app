use cnctd_server::router::message::Message;
use serde_json::json;
use solana::pool_manager::message::{MessageType, PoolManagerMessage};

pub async fn route_pool_manager_message(pool_manager_message: PoolManagerMessage) {
    match pool_manager_message.message_type {
        MessageType::UpdatePosition 
        | MessageType::RemovePosition 
        | MessageType::Stats => {
            let (channel, instruction, frequency) = match pool_manager_message.message_type {
                MessageType::UpdatePosition => ("managed-position", "update", pool_manager_message.frequency_seconds),
                MessageType::RemovePosition => ("managed-position", "remove", pool_manager_message.frequency_seconds),
                MessageType::Stats => ("stats", "update", pool_manager_message.frequency_seconds),
            };
        
            let message_data = json!({"data": pool_manager_message.data, "frequency": frequency});
            let message = Message::new(channel, instruction, Some(message_data));
        
            match message.broadcast().await {
                Ok(_) => {},
                Err(e) => println!("Failed to broadcast managed position: {:?}", e),
            }
        }
        _ => {}
    }   
}