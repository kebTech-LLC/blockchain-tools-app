// pub mod solana_token;

use std::{collections::HashMap, str::FromStr, sync::{Arc, Mutex}};

use mpl_token_metadata::accounts::Metadata;
use serde::{Deserialize, Serialize};
use solana_sdk::{program_pack::Pack, pubkey::Pubkey};
use spl_token::state::Mint;
use state::InitCell;

use crate::{rpc::{Rpc, RpcMode}, utils::trim_null_bytes};

pub static TOKEN_STORE: InitCell<Arc<Mutex<HashMap<String, Token>>>> = InitCell::new();

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Token {
    pub name: String,
    pub symbol: String,
    pub address: String,
    pub decimals: u8,
    pub is_stablecoin: bool,
}

impl Token {
    pub fn new(name: String, symbol: String, address: String, decimals: u8) -> Self {
        let name = trim_null_bytes(&name);
        let symbol = trim_null_bytes(&symbol);
        let is_stablecoin = matches!(symbol.as_str(), "USDC" | "USDT" | "DAI" | "USDH" | "UXD" | "PAI");
        Token {
            name,
            symbol,
            address,
            decimals,
            is_stablecoin,
        }
    }

    pub fn initiate_token_store() {
        TOKEN_STORE.set(Arc::new(Mutex::new(HashMap::new())));
    }

    pub async fn from_mint_address(address: &str) -> anyhow::Result<Self> {
        // Check if the token is already in the store
        {
            let token_store = TOKEN_STORE.get().lock().map_err(|e| anyhow::anyhow!(e.to_string()))?;
            if let Some(token) = token_store.get(address) {
                // println!("Token found in store: {}", token.symbol);
                return Ok(token.clone());
            }
        }

        let mint_pubkey = Pubkey::from_str(address)?;

        let account_data = Rpc::call(
            move |client| {
                let mint_pubkey = mint_pubkey.clone();
                Box::pin(async move {
                    client.get_account(&mint_pubkey).await.map_err(|e| e.into())
                })
            },
            Some(5000),
            RpcMode::fast(),
        ).await?;
        
        let mint = Mint::unpack(&account_data.data)?;
        
        let metadata_program_id = Pubkey::from_str("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s")?;
        
        let seeds = &[
            b"metadata",
            metadata_program_id.as_ref(),
            mint_pubkey.as_ref(),
        ];

        let (metadata_pda, _) = Pubkey::find_program_address(seeds, &metadata_program_id);

        // Fetch the metadata account
        let metadata_account = Rpc::call(
            move |client| {
                let metadata_pda = metadata_pda.clone();
                Box::pin(async move {
                    client.get_account(&metadata_pda).await.map_err(|e| e.into())
                })
            },
            Some(5000),
            RpcMode::fast(),
        ).await?;
    
        let metadata_data = metadata_account.data;

        // Deserialize the metadata account
        let metadata = Metadata::safe_deserialize(&mut metadata_data.as_slice())?;
    
        let token = Token::new(
            metadata.name,
            metadata.symbol,
            address.to_string(),
            mint.decimals,
        );

        println!("Adding token to store: {}", token.symbol);
        {
            let mut token_store = TOKEN_STORE.get().lock().map_err(|e| anyhow::anyhow!(e.to_string()))?;
            token_store.insert(address.to_string(), token.clone());
        } 

        Ok(token)

    }

    pub fn solana() -> Self {
        Token {
            name: "Solana".to_string(),
            symbol: "SOL".to_string(),
            address: "So11111111111111111111111111111111111111112".to_string(),
            decimals: 9,
            is_stablecoin: false,
        }
    }
}