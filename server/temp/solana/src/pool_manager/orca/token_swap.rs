use chrono::Utc;
use figlet_rs::FIGfont;
use helius::types::PriorityLevel;
use serde::{Deserialize, Serialize};
use solana_sdk::signature::Signature;
use kebtech_utils::*;

use crate::{pool_manager::{new_position::NewPositionData, POOL_MANAGER}, rpc::{ComputeUnitLimit, PriorityFee, Rpc, RpcMode}, wallet::Wallet};

use super::Orca;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenSwap {
    pub wallet_key: String,
    pub pool_address: String,
    pub amount: u64,
    pub amount_is_in: bool,
    pub mint_out_address: String,
    pub slippage_tolerance: Option<u16>,
}

impl TokenSwap {
    pub fn new(wallet_key: String, pool_address: String, amount: u64, amount_is_in: bool, mint_out_address: String, slippage_tolerance: Option<u16>) -> Self {
        TokenSwap {
            wallet_key,
            pool_address,
            amount,
            amount_is_in,
            mint_out_address,
            slippage_tolerance,
        }
    }

    pub async fn swap(self) -> anyhow::Result<Signature> {
        let start = Utc::now();
        blue!("Getting swap instructions");
        let swap_instructions = Orca::get_swap_instructions(self.clone()).await?;
        green!("Got swap instructions in {}ms", start.signed_duration_since(Utc::now()).num_milliseconds());

        println!("swap instructions: {:?}", swap_instructions);

        let start = Utc::now();
        blue!("Performing swap transaction");
        let signature = Orca::perform_orca_transaction(
            swap_instructions.instructions,
            swap_instructions.additional_signers,
            Some(PriorityLevel::High),
        ).await?;

        green!("Performed swap transaction in {}ms", start.signed_duration_since(Utc::now()).num_milliseconds());

        let font = FIGfont::standard().unwrap();
        let banner = font.convert("Swapped Tokens").unwrap();
        green!("\n\n{}\n\n", banner);

        Ok(signature)
    }
    
}