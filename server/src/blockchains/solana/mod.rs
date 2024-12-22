pub mod account_snapshot;
pub mod liquidity_pools;

use std::{env, str::FromStr, time::Duration};

use anyhow::{Result, anyhow};
use liquidity_pools::PoolInfoResponse;
use pyth_sdk_solana::state::SolanaPriceAccount;
use serde::Deserialize;
use serde_json::{json, Value};
use solana_account_decoder::UiAccountData;
use solana_client::{nonblocking::rpc_client::RpcClient, rpc_config::RpcProgramAccountsConfig, rpc_filter::RpcFilterType, rpc_request::TokenAccountsFilter};
use solana_sdk::{account::Account, pubkey::Pubkey, signature::Signature};
use solana_transaction_status::UiTransactionEncoding;

use crate::centralized_marketplaces::coinbase::Coinbase;

pub struct Solana {
    client: RpcClient,
    raydium_base: String,
}



#[derive(Debug, Deserialize)]
struct GetBalanceResponse {
    result: BalanceResult,
}

#[derive(Debug, Deserialize)]
struct BalanceResult {
    value: u64, // Lamports (1 SOL = 10^9 lamports)
}


impl Solana {
    pub fn new() -> Self {
        // let api_key = env::var("HELIUS_API_KEY").expect("HELIUS_API_KEY not set");
        // let rpc_url = format!("https://api.helius.xyz/v0/rpc?api-key={}", api_key);
        let rpc_url = "https://api.mainnet-beta.solana.com";
        let client = RpcClient::new(rpc_url.to_string());
        let raydium_base = "https://api-v3.raydium.io".to_string();
        Self { client, raydium_base: raydium_base }
    }

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
        // Solana RPC endpoint
        let client = Self::new().client;

        // Get the liquidity pool program ID
        let liquidity_pool_program_id = Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")?;

        // Get the liquidity pool accounts
        let accounts = client.get_program_accounts(&liquidity_pool_program_id).await?;

        Ok(json!(accounts))
    }

    pub async fn get_pool_info(&self, pool_pubkey: &str) -> Result<()> {
        let raydium_url = format!("{}/pools/info/ids?ids={}", self.raydium_base, pool_pubkey);
        
        // Use a reqwest Client for connection reuse
        let client = reqwest::Client::new();
        let response = client.get(&raydium_url).send().await?;
        

        if response.status().is_success() {
            // Deserialize JSON directly into a serde_json::Value
            let json: Value = response.json().await?;

            let pool_info: PoolInfoResponse = serde_json::from_value(json)?;
            println!("{:#?}", pool_info);

            if pool_info.data.is_empty() {
                return Err(anyhow!("Pool not found"));
            } else {
                let pool_data = &pool_info.data[0];
                println!("Pool ID: {}", pool_data.id);
                println!("Pool Type: {}", pool_data.type_);
                println!("Program ID: {}", pool_data.program_id);
                println!("Mint A: {}", pool_data.mint_a.symbol);
                println!("Mint B: {}", pool_data.mint_b.symbol);
                println!("TVL: ${:.2}", pool_data.tvl);
                println!("Price: ${:.2}", pool_data.price);
                println!("Fee Rate: {:.2}%", pool_data.fee_rate);
                println!("Burn Percent: {:.2}%", pool_data.burn_percent);
                println!("Config ID: {}", pool_data.config.id);
                println!("Protocol Fee Rate: {:.2}%", pool_data.config.protocol_fee_rate as f64 / 100.0);
                println!("Trade Fee Rate: {:.2}%", pool_data.config.trade_fee_rate as f64 / 100.0);
                println!("Tick Spacing: {}", pool_data.config.tick_spacing);
                println!("Default Range: {:.2}", pool_data.config.default_range);
                println!("Default Range Point: {:?}", pool_data.config.default_range_point);
                println!("Fund Fee Rate: {:.2}%", pool_data.config.fund_fee_rate as f64 / 100.0);
                println!("Day APR: {:.2}%", pool_data.day.apr);
                println!("Day Fee APR: {:.2}%", pool_data.day.fee_apr);
                println!("Day Volume: ${:.2}", pool_data.day.volume);
                println!("Day Volume Quote: ${:.2}", pool_data.day.volume_quote);
                println!("Day Volume Fee: ${:.2}", pool_data.day.volume_fee);
                println!("Day Price Min: ${:.2}", pool_data.day.price_min);
                println!("Day Price Max: ${:.2}", pool_data.day.price_max);
                println!("Week APR: {:.2}%", pool_data.week.apr);
                println!("Week Fee APR: {:.2}%", pool_data.week.fee_apr);
                println!("Week Volume: ${:.2}", pool_data.week.volume);
                println!("Week Volume Quote: ${:.2}", pool_data.week.volume_quote);
                println!("Week Volume Fee: ${:.2}", pool_data.week.volume_fee);
                println!("Week Price Min: ${:.2}", pool_data.week.price_min);
                println!("Week Price Max: ${:.2}", pool_data.week.price_max);
            }

            Ok(())
        } else {
            Err(anyhow!(
                "Failed to fetch pool info: {} - {}",
                response.status(),
                response.text().await.unwrap_or_else(|_| "No additional error message".to_string())
            ))
        }
    }

    pub async fn get_token_balance(&self, wallet_pubkey: &str, mint_address: &str) -> Result<f64> {
        // Convert wallet and mint addresses into Pubkey objects
        let pubkey = Pubkey::from_str(wallet_pubkey)
            .map_err(|e| anyhow!("Invalid wallet public key: {}", e))?;
        let mint_pubkey = Pubkey::from_str(mint_address)
            .map_err(|e| anyhow!("Invalid mint public key: {}", e))?;

        // Query token accounts owned by the wallet that match the mint address
        let token_accounts = self
            .client
            .get_token_accounts_by_owner(
                &pubkey,
                TokenAccountsFilter::Mint(mint_pubkey),
            )
            .await
            .map_err(|e| anyhow!("Failed to fetch token accounts: {}", e))?;

        // If no token accounts are found, return a balance of 0
        if token_accounts.is_empty() {
            return Ok(0.0);
        }

        // Parse the token accounts to accumulate the total token balance
        let mut total_balance = 0.0;
        for account in token_accounts {
            match &account.account.data {
                UiAccountData::Json(json_data) => {
                    // Extract "tokenAmount.uiAmount" from the parsed JSON
                    if let Some(ui_amount) = json_data
                        .parsed
                        .get("info")
                        .and_then(|info| info.get("tokenAmount"))
                        .and_then(|token_amount| token_amount.get("uiAmount"))
                        .and_then(|ui_amount| ui_amount.as_f64())
                    {
                        total_balance += ui_amount; // Sum up balances
                    }
                }
                _ => continue, // Skip accounts that are not JSON-parsed
            }
        }

        Ok(total_balance) // Return the total token balance
    }

    pub async fn get_sol_balance(&self, wallet_pubkey: &str) -> Result<f64> {
        // Convert the wallet's public key into a Pubkey type
        let pubkey = Pubkey::from_str(wallet_pubkey)
            .map_err(|e| anyhow!("Invalid public key: {}", e))?;

        // Use the existing client to fetch the SOL balance
        let lamports = self
            .client
            .get_balance(&pubkey)
            .await
            .map_err(|e| anyhow!("Failed to fetch SOL balance: {}", e))?;

        // Convert lamports to SOL (1 SOL = 10^9 lamports)
        let sol_balance = lamports as f64 / 1_000_000_000.0;
        Ok(sol_balance)
    }

    pub async fn get_usdc_balance(&self, wallet_pubkey: &str) -> Result<f64> {
        let mint_address = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
        self.get_token_balance(wallet_pubkey, mint_address).await
    }

    pub async fn get_sol_usd_price(&self) -> Result<f64> {
        let price = Coinbase::get_sol_usd_price().await?;
        Ok(price)

        // let client = &self.client;
    
        // // Pyth SOL/USD Price Account
        // let pyth_sol_usd_pubkey = Pubkey::from_str("H6nQZwEgjyrpu9bH3as3t4E7Qz8mV9qYXeiaN6m1K36M")
        //     .map_err(|e| anyhow!("Invalid Pyth SOL/USD public key: {}", e))?;
    
        // // Fetch the account data
        // let account_data = client
        //     .get_account_data(&pyth_sol_usd_pubkey)
        //     .await
        //     .map_err(|e| anyhow!("Failed to fetch Pyth account data: {}", e))?;
    
        // // Decode the account data using Pyth's library
        // let price_data: &pyth_sdk_solana::state::PriceAccount = 
        //     pyth_sdk_solana::state::load_price_account(&account_data)
        //     .map_err(|e| anyhow!("Failed to parse Pyth account data: {}", e))?;
    
        // // Debugging: Print the raw price and exponent
        // println!("Raw price (agg.price): {}", price_data.agg.price);
        // println!("Exponent (expo): {}", price_data.expo);
    
        // // Correctly scale the price
        // let price = price_data.agg.price as f64 * 10_f64.powi(price_data.expo);
    
        // Ok(price)
    }
    
    
}