use std::str::FromStr;

use crate::external_apis::helius::{stake_account::StakeAccount, Helius};
use anyhow::{Result, anyhow};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{pubkey::Pubkey, signature::Signature};
use solana_transaction_status::UiTransactionEncoding;
pub struct Solana {}

impl Solana {
    pub async fn get_stake_accounts(public_key: &str) -> anyhow::Result<Vec<StakeAccount>> {
        let helius = Helius::new()?;
        let stake_accounts = helius.get_stake_accounts(public_key).await?;
        Ok(stake_accounts)
    }

    pub async fn get_balance(public_key: &str) -> Result<u64> {
        // Solana RPC endpoint (Devnet or Mainnet)
        let rpc_url = "https://api.mainnet-beta.solana.com";
        let client = RpcClient::new(rpc_url.to_string());
    
        // Convert the public key to a Solana Pubkey
        let pubkey = Pubkey::from_str(public_key).map_err(|e| anyhow::anyhow!("Invalid public key: {}", e))?;
    
        // Fetch the account balance
        let balance = client
            .get_balance(&pubkey)
            .map_err(|e| anyhow::anyhow!("Failed to fetch balance: {}", e))?;
    
        Ok(balance)
    }

    pub async fn get_latest_transaction(public_key: &str) -> Result<Option<serde_json::Value>> {
        // Solana RPC endpoint
        let rpc_url = "https://api.mainnet-beta.solana.com";
        let client = RpcClient::new(rpc_url.to_string());
    
        // Convert the public key to a Solana Pubkey
        let pubkey = Pubkey::from_str(public_key).map_err(|e| anyhow!("Invalid public key: {}", e))?;
    
        // Get recent transaction signatures for the public key
        let signatures = client
            .get_signatures_for_address(&pubkey)
            .map_err(|e| anyhow!("Failed to fetch transaction signatures: {}", e))?;
    
        // Get the most recent signature (if any exist)
        if let Some(signature_str) = signatures.get(0) {
            // Convert the signature string to a Signature type
            let signature = Signature::from_str(&signature_str.signature)
                .map_err(|e| anyhow!("Invalid signature: {}", e))?;
    
            // Fetch the transaction details
            let transaction = client
                .get_transaction(&signature, UiTransactionEncoding::Json)
                .map_err(|e| anyhow!("Failed to fetch transaction details: {}", e))?;
    
            // Return the transaction as a JSON object
            Ok(Some(serde_json::to_value(transaction)?))
        } else {
            // No transactions found for the public key
            Ok(None)
        }
    }
}