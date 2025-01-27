use base64::Engine;
use serde::{Deserialize, Deserializer, Serializer};
use solana_sdk::{bs58, transaction::Transaction};

pub fn trim_null_bytes(s: &str) -> String {
    s.trim_end_matches('\0').to_string()
}

pub fn serialize_transaction_to_base58(transaction: &Transaction) -> anyhow::Result<String> {
    let serialized_tx = bincode::serialize(transaction)
        .map_err(|e| anyhow::anyhow!("Failed to serialize transaction: {:?}", e))?;
    let base58_encoded_tx = bs58::encode(serialized_tx).into_string();
    Ok(base58_encoded_tx)
}

pub fn serialize_transaction_to_base64(transaction: &Transaction) -> anyhow::Result<String> {
    // Serialize the transaction using bincode
    let serialized_tx = bincode::serialize(transaction)
        .map_err(|e| anyhow::anyhow!("Failed to serialize transaction: {:?}", e))?;
    
    // Encode the serialized transaction to Base64
    let base64_encoded_tx = base64::engine::general_purpose::STANDARD.encode(serialized_tx);
    
    Ok(base64_encoded_tx)
}

pub fn u128_to_string<S>(value: &u128, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&value.to_string())
}

pub fn string_to_u128<'de, D>(deserializer: D) -> Result<u128, D::Error>
where
    D: Deserializer<'de>,
{
    // Deserialize as a string first
    let s: String = Deserialize::deserialize(deserializer)?;
    // Parse the string into a u128
    s.parse::<u128>().map_err(serde::de::Error::custom)
}