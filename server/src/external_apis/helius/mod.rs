pub mod stake_account;
pub mod account_info;

use std::time::Duration;

use account_info::{AccountInfo, AccountInfoResponse};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use stake_account::{StakeAccount, StakeAccountsResponse};
use anyhow::{Result, anyhow};

pub struct Helius {
    pub api_key: String,
    pub client: Client, // Reqwest client to make HTTP requests
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Transaction {
    #[serde(rename = "blockTime")]
    pub block_time: Option<i64>,      // Corresponds to "blockTime" (nullable)
    #[serde(rename = "confirmationStatus")]
    pub confirmation_status: String, // Corresponds to "confirmationStatus"
    pub err: Option<serde_json::Value>, // Corresponds to "err" (nullable, JSON object or null)
    pub memo: Option<String>,        // Corresponds to "memo" (nullable)
    pub signature: String,           // Corresponds to "signature"
    pub slot: u64,                   // Corresponds to "slot"
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TransactionHistoryResponse {
    pub jsonrpc: String,         // Corresponds to "jsonrpc" (e.g., "2.0")
    pub result: Vec<Transaction>, // Corresponds to the "result" array of transactions
    pub id: u32,                 // Corresponds to "id" (request ID)
}

impl Helius {
    /// Initialize the Helius client with an API key from environment variables
    pub fn new() -> Result<Self> {
        let api_key = dotenv::var("HELIUS_API_KEY")?;
        let client = Client::builder()
            .timeout(Duration::from_secs(120)) // Set a custom timeout (120 seconds)
            .build()?; // Build the reqwest client
        Ok(Self { api_key, client })
    }

    /// Initialize the Helius client with a provided API key
    pub fn with_api_key(api_key: String) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(360)) // Set a custom timeout (120 seconds)
            .build()
            .expect("Failed to build client");
        Self { api_key, client }
    }

    /// Fetch the transaction history for a given wallet address
    pub async fn get_transaction_history(
        &self,
        wallet_pubkey: &str,
    ) -> Result<TransactionHistoryResponse> {
        let url = format!(
            "https://mainnet.helius-rpc.com/?api-key={}",
            self.api_key
        );
    
        let request_body = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getSignaturesForAddress",
            "params": [
                wallet_pubkey,
                { "limit": 1000 }
            ]
        });
    
        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;
    
        // Save the status and process the response
        let raw_body = process_response(response).await?;
    
        // Deserialize into TransactionHistoryResponse
        let parsed_response: TransactionHistoryResponse =
            serde_json::from_str(&raw_body).map_err(|e| {
                anyhow!(
                    "Failed to parse response body: {}. Raw body: {}",
                    e,
                    raw_body
                )
            })?;

            println!("Parsed response: {:?}", parsed_response);
    
        Ok(parsed_response)
    }

    pub async fn get_full_transaction(
        &self,
        signature: &str,
    ) -> Result<serde_json::Value> {
        let url = format!(
            "https://mainnet.helius-rpc.com/?api-key={}",
            self.api_key
        );
    
        let request_body = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getTransaction",
            "params": [
                signature,
                { "encoding": "json" }
            ]
        });
    
        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;
    
        let raw_body = process_response(response).await?;
        let transaction_details: serde_json::Value = serde_json::from_str(&raw_body)
            .map_err(|e| anyhow!("Failed to parse transaction: {}. Raw body: {}", e, raw_body))?;
    
        Ok(transaction_details)
    }

    pub async fn get_stake_accounts_from_history(
        &self,
        wallet_pubkey: &str,
    ) -> Result<Vec<String>> {
        // Step 1: Get transaction history
        let history = self.get_transaction_history(wallet_pubkey).await?;
    
        let mut stake_account_ids = Vec::new();
    
        for transaction in history.result {
            // Step 2: Fetch full transaction details
            let full_transaction = self.get_full_transaction(&transaction.signature).await?;
    
            // Step 3: Extract accounts from the transaction
            if let Some(accounts) = full_transaction["transaction"]["message"]["accountKeys"].as_array()
            {
                for account in accounts {
                    if account.as_str().unwrap_or("").starts_with("Stake11111111111111111111111111111111111111") {
                        stake_account_ids.push(account.as_str().unwrap().to_string());
                    }
                }
            }
        }
    
        Ok(stake_account_ids)
    }
    
    
}


pub async fn process_response(response: reqwest::Response) -> Result<String> {
    // Save the status for error handling
    let status = response.status();

    // Clone the body content into a string before consuming it
    let raw_body = response.text().await?; // Consume response here

    // Handle HTTP errors
    if !status.is_success() {
        // Use the captured raw body for error reporting
        return Err(anyhow!("Request failed: HTTP {} - Body: {}", status, raw_body));
    }

    // Return the raw body for further processing if successful
    Ok(raw_body)
}