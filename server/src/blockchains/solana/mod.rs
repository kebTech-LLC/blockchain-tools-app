pub mod account_snapshot;
pub mod liquidity_pools;

use std::{str::FromStr, time::Duration};

use anyhow::{Result, anyhow};
use serde_json::Value;
use solana_client::{nonblocking::rpc_client::RpcClient, rpc_config::RpcProgramAccountsConfig, rpc_filter::RpcFilterType};
use solana_sdk::{account::Account, pubkey::Pubkey, signature::Signature};
use solana_transaction_status::UiTransactionEncoding;
pub struct Solana {}

impl Solana {
    pub async fn get_stake_accounts(wallet_pubkey: &str) -> Result<Vec<(Pubkey, Account)>, Box<dyn std::error::Error>> {
        let rpc_url = "https://api.mainnet-beta.solana.com";
        let client = RpcClient::new_with_timeout(rpc_url.into(), Duration::from_secs(360));
    
        // Stake Program ID
        let stake_program_id = Pubkey::from_str("Stake11111111111111111111111111111111111111")?;
    
        // Wallet public key
        let wallet_pubkey = Pubkey::from_str(wallet_pubkey)?;
    
        // Filters: data size = 200, authorized staker = wallet_pubkey
        let filters = Some(vec![
            RpcFilterType::DataSize(200), // Only stake accounts (200 bytes in size)
            RpcFilterType::Memcmp(
                solana_client::rpc_filter::Memcmp::new_base58_encoded(
                    44,                           // Offset in StakeState where `authorized staker` starts
                    wallet_pubkey.as_ref(),    // Wallet public key in base58 format
                )
            ),
        ]);
    
        // Query the stake accounts
        let config = RpcProgramAccountsConfig {
            filters,
            account_config: solana_client::rpc_config::RpcAccountInfoConfig {
                encoding: Some(solana_account_decoder::UiAccountEncoding::Base64),
                ..Default::default()
            },
            ..Default::default()
        };
    
        let accounts = client.get_program_accounts_with_config(&stake_program_id, config).await?;

    
        Ok(accounts)
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
            .await
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
            .await
            .map_err(|e| anyhow!("Failed to fetch transaction signatures: {}", e))?;
    
        // Get the most recent signature (if any exist)
        if let Some(signature_str) = signatures.get(0) {
            // Convert the signature string to a Signature type
            let signature = Signature::from_str(&signature_str.signature)
                .map_err(|e| anyhow!("Invalid signature: {}", e))?;
    
            // Fetch the transaction details
            let transaction = client
                .get_transaction(&signature, UiTransactionEncoding::Json)
                .await
                .map_err(|e| anyhow!("Failed to fetch transaction details: {}", e))?;
    
            // Return the transaction as a JSON object
            Ok(Some(serde_json::to_value(transaction)?))
        } else {
            // No transactions found for the public key
            Ok(None)
        }
    }

    pub async fn get_liquidity_pools() -> Result<Value> {
        let liquidity_pools = liquidity_pools::SolanaLiquidityPools::new();
        let pools = liquidity_pools.get_liquidity_pools().await?;
        Ok(pools)
    }
}