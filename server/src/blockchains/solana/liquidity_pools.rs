use serde_json::{json, Value};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use anyhow::Result;

pub struct SolanaLiquidityPools {
    client: RpcClient,
}

impl SolanaLiquidityPools {
    pub fn new() -> Self {
        let rpc_url = "https://api.mainnet-beta.solana.com";
        let client = RpcClient::new(rpc_url.to_string());
        Self { client }
    }

    pub async fn get_liquidity_pools(&self) -> Result<Value> {
        // Solana RPC endpoint
        let client = Self::new().client;

        // Get the liquidity pool program ID
        let liquidity_pool_program_id = Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")?;

        // Get the liquidity pool accounts
        let accounts = client.get_program_accounts(&liquidity_pool_program_id).await?;

        Ok(json!(accounts))
    }
}