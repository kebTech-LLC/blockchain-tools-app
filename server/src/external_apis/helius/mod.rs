pub mod stake_account;
pub mod account_info;

use account_info::{AccountInfo, AccountInfoResponse};
use reqwest::Client;
use stake_account::{StakeAccount, StakeAccountsResponse};

pub struct Helius {
    pub api_key: String,
    pub client: Client, // Reqwest client to make HTTP requests
}

impl Helius {
    /// Initialize the Helius client with an API key
    pub fn new() -> anyhow::Result<Self> {
        let api_key = dotenv::var("HELIUS_API_KEY")?;
        let client = Client::new();
        Ok(Self { api_key, client })
    }

    /// Initialize the Helius client with a provided API key
    pub fn with_api_key(api_key: String) -> Self {
        let client = Client::new();
        Self { api_key, client }
    }

    /// Fetch stake accounts for a given wallet public key
    pub async fn get_stake_accounts(&self, wallet_pubkey: &str) -> anyhow::Result<Vec<StakeAccount>> {
        let url = format!(
            "https://api.helius.xyz/v0/stake-accounts?api-key={}",
            self.api_key
        );

        // Prepare the request body
        let body = serde_json::json!({ "wallet": wallet_pubkey });

        // Send the POST request
        let response = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to send request: {}", e))?;

        // Ensure the response is successful
        if !response.status().is_success() {
            let error_message = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(anyhow::anyhow!(
                "Failed to fetch stake accounts: {}",
                error_message
            ));
        }

        // Deserialize the response JSON
        let stake_accounts: StakeAccountsResponse = response
            .json()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to parse response: {}", e))?;

        Ok(stake_accounts.accounts)
    }
}