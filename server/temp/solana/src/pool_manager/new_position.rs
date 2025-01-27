use std::{sync::Arc, time::Instant};

use chrono::{DateTime, Utc};
use figlet_rs::FIGfont;
use helius::types::PriorityLevel;
use orca_pools_ipc_types::response::open_position_instruction::OrcaOpenPositionInstruction;
use serde::{Deserialize, Serialize};
use solana_client::rpc_response::RpcSimulateTransactionResult;
use solana_sdk::{instruction::Instruction, signer::Signer, hash::Hash};
use kebtech_utils::*;
use state::InitCell;
use tokio::sync::Mutex;

use crate::{pool_manager::{new_position, orca::token_swap::TokenSwap, PoolManager}, rpc::{Rpc, RpcMode}, token::Token, wallet::Wallet};

use super::{orca::Orca, position_manager::managed_position::{ManagedPosition, PoolType}, POOL_MANAGER};

pub static NEW_POSITION_DATA: InitCell<Arc<Mutex<NewPositionData>>> = InitCell::new();

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewProgrammaticPosition {
    pub pool_type: PoolType,
    pub pool_address: String,
    pub token_mint_a: String,
    pub token_mint_b: String,
}

impl NewProgrammaticPosition {
    pub fn default() -> Self {
        Self {
            pool_type: PoolType::Orca,
            pool_address: "Czfq3xZZDmsdGdUyrNLtRhGc47cXcZtLG4crryfu44zE".to_string(),
            token_mint_a: "So11111111111111111111111111111111111111112".to_string(),
            token_mint_b: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".to_string(),
        }
    }

    pub async fn open(&self) -> anyhow::Result<()> {
        // let mut new_position_data_lock = NEW_POSITION_DATA.get().lock().await;
        magenta!("opening new position: {:?}", self);
        
        let (_token_amount_a, token_amount_b, range_lower, range_upper) = self.balance_tokens().await?;
        println!("finished balancing tokens");
        let buffer_percent = 0.075;
        let token_amount_b_with_buffer = token_amount_b.saturating_sub((token_amount_b as f64 * buffer_percent) as u64);

        let start = Utc::now();
        blue!("getting open position instructions");
        let open_position_instructions = Orca::get_prog_open_position_instructions(
            &self.pool_address,
            token_amount_b_with_buffer,
            500,
            range_lower,
            range_upper
        ).await?;

        green!("got open position instructions in {:?}ms", start.signed_duration_since(Utc::now()).num_milliseconds());

        magenta!("open position instructions: {:?}", open_position_instructions);

        yellow!("token amount b with buffer: {}", token_amount_b_with_buffer);

        let start = Utc::now();
        blue!("performing open position transaction");
        let _signature = Orca::perform_orca_transaction(
            open_position_instructions.instructions,
            open_position_instructions.additional_signers,
            Some(PriorityLevel::High),
        ).await?;

        green!("performed open position transaction in {:?}ms", start.signed_duration_since(Utc::now()).num_milliseconds());

        let font = FIGfont::standard().unwrap();
        let banner = font.convert("Opened Position").unwrap();
        green!("\n\n{}\n\n", banner);

        PoolManager::fetch_and_update_managed_positions(0).await?;

        Ok(())
    }

    pub async fn balance_tokens(&self) -> anyhow::Result<(u64, u64, f64, f64)> {
        loop {
            let (balance_a, balance_b, range_lower, range_upper, swapped) = self.balance_tokens_core().await?;
            if !swapped {
                return Ok((balance_a, balance_b, range_lower, range_upper)); // Exit when no swap is needed
            }
    
            println!("Rebalancing again after swap...");
        }
    }
    
    pub async fn balance_tokens_core(&self) -> anyhow::Result<(u64, u64, f64, f64, bool)> {
        let token_a = Token::from_mint_address(&self.token_mint_a).await?;
        let token_b = Token::from_mint_address(&self.token_mint_b).await?;
        let wallet = Wallet::get_programmatic_keypair()?;
    
        let mut balance_a_amount = NewPositionData::get_balance_a_amount(&self).await?;
        let mut balance_b_amount = NewPositionData::get_balance_b_amount(&self).await?;
        let decimals_a = 10u64.pow(token_a.decimals as u32);
        let decimals_b = 10u64.pow(token_b.decimals as u32);
    
        // Adjust for SOL fees
        let sol_fee_reserve = 0.1; // Reserve 0.1 SOL for fees
        if token_a.address == Token::solana().address {
            balance_a_amount = balance_a_amount.saturating_sub((sol_fee_reserve * decimals_a as f64) as u64);
        }
    
        let current_price = NewPositionData::get_pool_price(self).await?;
        let value_a_usd = balance_a_amount as f64 / decimals_a as f64 * current_price;
        let value_b_usd = balance_b_amount as f64 / decimals_b as f64;
        let total_value_usd = value_a_usd + value_b_usd;
    
        let ratio_a = value_a_usd / total_value_usd;
    
        println!(
            "Imbalance Ratios: Token A: {:.2}%, Token B: {:.2}%",
            ratio_a * 100.0,
            (1.0 - ratio_a) * 100.0
        );
    
        let tolerance_lower = 0.45;
        let tolerance_upper = 0.55;
    
        if ratio_a >= tolerance_lower && ratio_a <= tolerance_upper {
            println!("Balances are within tolerance. No swap needed.");
            let (range_lower, range_upper) = NewPositionData::get_ranges(current_price);
            return Ok((balance_a_amount, balance_b_amount, range_lower, range_upper, false)); // No swap performed
        }
    
        println!("Balances are outside tolerance. Performing swap...");
        if value_a_usd > value_b_usd {
            let excess_usd = value_a_usd - total_value_usd / 2.0;
            let swap_amount = (excess_usd / current_price) * decimals_a as f64;
            println!("Swapping {} Token A to balance 50/50.", swap_amount);
            let default_pool = NewProgrammaticPosition::default();
            TokenSwap::new(
                wallet.pubkey().to_string(),
                default_pool.pool_address.clone(),
                swap_amount as u64,
                true,
                token_a.address.clone(),
                Some(50),
            )
            .swap()
            .await?;
        } else {
            let excess_usd = value_b_usd - total_value_usd / 2.0;
            let swap_amount = excess_usd * decimals_b as f64;
            println!("Swapping {} Token B to balance 50/50.", swap_amount);
            TokenSwap::new(
                wallet.pubkey().to_string(),
                self.pool_address.clone(),
                swap_amount as u64,
                true,
                token_b.address.clone(),
                Some(50),
            )
            .swap()
            .await?;
        }
        
        NewPositionData::set_token_amounts(&self).await?;
        // Indicate that a swap was performed
        Ok((balance_a_amount, balance_b_amount, 0.0, 0.0, true))
    }
    
    
    pub fn from_managed_position(managed_position: &ManagedPosition) -> anyhow::Result<Self> {
        let token_mint_a = managed_position.token_a.clone().ok_or_else(|| anyhow::anyhow!("Token A not found in managed position")).map_err(|e| anyhow::anyhow!("Failed to create NewProgrammaticPosition: {:?}", e))?.address;
        let token_mint_b = managed_position.token_b.clone().ok_or_else(|| anyhow::anyhow!("Token B not found in managed position")).map_err(|e| anyhow::anyhow!("Failed to create NewProgrammaticPosition: {:?}", e))?.address;
        let position = Self {
            pool_type: managed_position.pool_type.clone(),
            pool_address: managed_position.pool_address.clone(),
            token_mint_a,
            token_mint_b,
        };

        Ok(position)
    }
    
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewPosition {
    pub wallet: Wallet,
    pub pool_type: PoolType,
    pub range_lower: f64,
    pub range_upper: f64,
    pub pool_address: String,
    pub amount_a: u64,
    pub amount_b: u64,
    pub amount_total: u64,
    pub wallet_balance_token_a: f64,
    pub wallet_balance_token_b: f64,
    pub wallet_balance_total: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewPositionData {
    pub pool_price: Option<f64>,
    pub balance_a_amount: Option<u64>,
    pub balance_b_amount: Option<u64>,
    pub sol_amount: Option<f64>,
    pub loop_active: bool,
}

impl NewPositionData {
    pub fn new() -> Self {
        Self {
            pool_price: None,
            balance_a_amount: None,
            balance_b_amount: None,
            sol_amount: None,
            loop_active: false,
        }
    }

    pub fn init() {
        NEW_POSITION_DATA.set(Arc::new(Mutex::new(Self::new())));
    }

    pub fn get_ranges(pool_price: f64) -> (f64, f64) {
        let range_lower = pool_price - (pool_price * 0.01);
        let range_upper = pool_price + (pool_price * 0.01);

        (range_lower, range_upper)
    }

    pub async fn pool_price_loop(position: &NewProgrammaticPosition) {
        let new_position_data = NEW_POSITION_DATA.get().lock().await.clone();
        if new_position_data.loop_active {
            drop(new_position_data);
            return;
        }

        let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));

        loop {
            let mut new_position_data_lock = NEW_POSITION_DATA.get().lock().await;
            new_position_data_lock.loop_active = true;
            drop(new_position_data_lock);
            interval.tick().await;
            // blue!("fetching pool price");
            let start = Instant::now();
            let price = match Self::fetch_pool_price(position).await {
                Ok(price) => price,
                Err(e) => {
                    eprintln!("Error fetching pool price: {:?}", e);
                    continue;
                }
            };
            // green!("fetched pool price in {:?}ms", start.elapsed().as_millis());
            let new_position_in_progress = POOL_MANAGER.get().lock().await.clone().position_to_open.is_some();
            if !new_position_in_progress {
                break;
            }
            let mut new_position_data_lock = NEW_POSITION_DATA.get().lock().await;
            
            new_position_data_lock.pool_price = Some(price);
            drop(new_position_data_lock);
        }
        let mut new_position_data_lock = NEW_POSITION_DATA.get().lock().await;
        new_position_data_lock.loop_active = false;
        drop(new_position_data_lock);
    }

    pub async fn set_token_amounts(position: &NewProgrammaticPosition) -> anyhow::Result<()> {
        let mut new_position_data_lock = NEW_POSITION_DATA.get().lock().await;
        let balance_a_amount = Self::fetch_balance_a_amount(position).await?;
        let balance_b_amount = Self::fetch_balance_b_amount(position).await?;
        let sol_amount = Self::fetch_sol_balance().await?;
        new_position_data_lock.balance_a_amount = Some(balance_a_amount);
        new_position_data_lock.balance_b_amount = Some(balance_b_amount);
        new_position_data_lock.sol_amount = Some(sol_amount);

        drop(new_position_data_lock);

        blue!("Balance A: {}", balance_a_amount);
        blue!("Balance B: {}", balance_b_amount);
        blue!("SOL Balance: {}", sol_amount);

        Ok(())
    }

    pub async fn fetch_pool_price(position: &NewProgrammaticPosition) -> anyhow::Result<f64> {
        let price = Orca::get_pool_price(RpcMode::fast(), &position.pool_address).await?;
       
        Ok(price)
    }

    pub async fn fetch_balance_a_amount(position: &NewProgrammaticPosition) -> anyhow::Result<u64> {
        let token_a = Token::from_mint_address(&position.token_mint_a).await?;
        let wallet = Wallet::get_programmatic_keypair()?;

        let (balance_a_amount, _balance_a_ui_amount) = Wallet::get_token_balance(
            &wallet.pubkey().to_string(),
            &token_a.address,
            RpcMode::fast(),
        )
            .await
            .unwrap_or((0, 0.0));

        Ok(balance_a_amount)
    }

    pub async fn fetch_balance_b_amount(position: &NewProgrammaticPosition) -> anyhow::Result<u64> {
        let token_b = Token::from_mint_address(&position.token_mint_b).await?;
        let wallet = Wallet::get_programmatic_keypair()?;

        let (balance_b_amount, _balance_b_ui_amount) = Wallet::get_token_balance(
            &wallet.pubkey().to_string(),
            &token_b.address,
            RpcMode::fast(),
        )
            .await
            .unwrap_or((0, 0.0));

        Ok(balance_b_amount)
    }

    pub async fn fetch_sol_balance() -> anyhow::Result<f64> {
        let wallet = Wallet::get_programmatic_keypair()?;
        let sol_balance = Wallet::get_sol_balance(&wallet.pubkey().to_string(), RpcMode::fast()).await.unwrap_or(0.0);

        Ok(sol_balance)
    }

    pub async fn get_balance_a_amount(position: &NewProgrammaticPosition) -> anyhow::Result<u64> {
        let new_position_data = NEW_POSITION_DATA.get().lock().await.clone();

        if let Some(balance_a_amount) = new_position_data.balance_a_amount {
            Ok(balance_a_amount)
        } else {
            let balance = Self::fetch_balance_a_amount(position).await?;
            let mut new_position_data_lock = NEW_POSITION_DATA.get().lock().await;
            new_position_data_lock.balance_a_amount = Some(balance);
            drop(new_position_data_lock);

            Ok(balance)
        }
    }

    pub async fn get_balance_b_amount(position: &NewProgrammaticPosition) -> anyhow::Result<u64> {
        let new_position_data = NEW_POSITION_DATA.get().lock().await.clone();

        if let Some(balance_b_amount) = new_position_data.balance_b_amount {
            Ok(balance_b_amount)
        } else {
            let balance = Self::fetch_balance_b_amount(position).await?;
            let mut new_position_data_lock = NEW_POSITION_DATA.get().lock().await;
            new_position_data_lock.balance_b_amount = Some(balance);
            drop(new_position_data_lock);

            Ok(balance)
        }
    }

    pub async fn get_sol_balance() -> anyhow::Result<f64> {
        let new_position_data = NEW_POSITION_DATA.get().lock().await.clone();

        if let Some(sol_amount) = new_position_data.sol_amount {
            Ok(sol_amount)
        } else {
            let sol_balance = Wallet::get_sol_balance(&Wallet::get_programmatic_keypair()?.pubkey().to_string(), RpcMode::fast()).await.unwrap_or(0.0);
            let mut new_position_data_lock = NEW_POSITION_DATA.get().lock().await;
            new_position_data_lock.sol_amount = Some(sol_balance);
            drop(new_position_data_lock);

            Ok(sol_balance)
        }
    }

    pub async fn get_pool_price(position: &NewProgrammaticPosition) -> anyhow::Result<f64> {
        let new_position_data = NEW_POSITION_DATA.get().lock().await.clone();

        if let Some(pool_price) = new_position_data.pool_price {
            Ok(pool_price)
        } else {
            let price = Orca::get_pool_price(RpcMode::fast(), &position.pool_address).await?;
            let mut new_position_data_lock = NEW_POSITION_DATA.get().lock().await;
            new_position_data_lock.pool_price = Some(price);
            drop(new_position_data_lock);

            Ok(price)
        }
    }

}