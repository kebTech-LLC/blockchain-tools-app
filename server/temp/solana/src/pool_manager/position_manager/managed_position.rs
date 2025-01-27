use chrono::{DateTime, Utc};
use figlet_rs::FIGfont;
use helius::types::PriorityLevel;
use orca_pools_ipc_types::response::{orca_pool_info::OrcaPoolInfo, orca_position_info::{OrcaPositionInfo, OrcaPositionRewardInfo}};
use serde::{Deserialize, Serialize, Serializer};
use kebtech_utils::*;

use crate::{pool_manager::{orca::Orca, PoolManager, POOL_MANAGER}, price_info::coinbase::ticker::TickerState, rpc::RpcMode, token::Token, utils::*};


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RangeState {
    OutUnder(f64),
    OutOver(f64),
    InLower(f64),
    InHigher(f64),
    Centered,
}

impl RangeState {
    pub fn get_score(&self) -> f64 {
        match self {
            RangeState::OutUnder(score) => *score,
            RangeState::OutOver(score) => *score,
            RangeState::InLower(score) => *score,
            RangeState::InHigher(score) => *score,
            RangeState::Centered => 0.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PoolType {
    Orca,
    Raydium,
    Saber,
    Mango,
    Serum,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedPosition {
    pub pool_type: PoolType,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
    pub address: String,
    pub wallet_key: String,
    pub position_mint: String,
    pub pool_address: String,
    pub tick_spacing: u16,
    #[serde(
        serialize_with = "u128_to_string",
        deserialize_with = "string_to_u128"
    )]
    pub sqrt_price: u128,
    pub token_a: Option<Token>,
    pub token_b: Option<Token>,
    pub balance_token_a: f64,
    pub balance_token_a_usd: f64,
    pub balance_token_a_percentage: f64,
    pub balance_token_b: f64,
    pub balance_token_b_usd: f64,
    pub balance_token_b_percentage: f64,
    pub balance_total_usd: f64,
    pub yield_token_a: f64,
    pub yield_token_a_usd: f64,
    pub yield_token_b: f64,
    pub yield_token_b_usd: f64,
    pub yield_total_usd: f64,
    pub range_lower: f64,
    pub range_upper: f64,
    pub reward_infos: Vec<PositionRewardInfo>,
    pub rewards_owed: Vec<u64>,
    pub current_price: f64,
    pub current_ticker_price: f64,
    // pub range_state_history: Vec<RangeState>,
    pub out_of_range_start: Option<DateTime<Utc>>,
    pub auto_rebalance: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionRewardInfo {
    pub growth_inside_checkpoint: u128,
    pub amount_owed: u64,
}

impl PositionRewardInfo {
    pub fn from_orca_position_reward_info(orca_position_reward_infos: Vec<OrcaPositionRewardInfo>) -> Vec<Self> {
        let mut reward_infos: Vec<Self> = Vec::new();
        for reward_info in orca_position_reward_infos {
            reward_infos.push(Self {
                growth_inside_checkpoint: reward_info.growth_inside_checkpoint,
                amount_owed: reward_info.amount_owed,
            });
        }
        reward_infos
    }
}

impl ManagedPosition {
    pub fn from_orca_position_info(orca_position_info: OrcaPositionInfo, created_at: DateTime<Utc>) -> Self {
        Self {
            pool_type: PoolType::Orca,
            created_at,
            updated_at: Utc::now(),
            closed_at: None,
            address: orca_position_info.address,
            wallet_key: orca_position_info.wallet_pubkey,
            position_mint: orca_position_info.position_mint,
            pool_address: orca_position_info.whirlpool_address,
            tick_spacing: 0,
            sqrt_price: 0,
            token_a: None,
            token_b: None,
            balance_token_a: 0.0,
            balance_token_a_usd: 0.0,
            balance_token_a_percentage: 0.0,
            balance_token_b: 0.0,
            balance_token_b_usd: 0.0,
            balance_token_b_percentage: 0.0,
            balance_total_usd: 0.0,
            yield_token_a: 0.0,
            yield_token_a_usd: 0.0,
            yield_token_b: 0.0,
            yield_token_b_usd: 0.0,
            yield_total_usd: 0.0,
            range_lower: 0.0,
            range_upper: 0.0,
            reward_infos: PositionRewardInfo::from_orca_position_reward_info(orca_position_info.reward_infos),
            rewards_owed: Vec::new(),
            current_price: 0.0,
            current_ticker_price: 0.0,
            // range_state_history: Vec::new(),
            out_of_range_start: None,
            auto_rebalance: true,
        }
    }

    pub fn balance_token_a_usd(&self) -> f64 {
        if let (Some(token_a), Some(token_b)) = (&self.token_a, &self.token_b) {
            if token_a.is_stablecoin {
                self.balance_token_a // Stablecoin A has 1:1 USD value
            } else if token_b.is_stablecoin {
                self.balance_token_a * self.current_price // Use current price for non-stablecoin A
            } else {
                self.balance_token_a * self.fetch_external_multiplier(token_a) // Placeholder for other cases
            }
        } else {
            0.0
        }
    }
    
    pub fn balance_token_b_usd(&self) -> f64 {
        if let (Some(token_a), Some(token_b)) = (&self.token_a, &self.token_b) {
            if token_b.is_stablecoin {
                self.balance_token_b // Stablecoin B has 1:1 USD value
            } else if token_a.is_stablecoin {
                self.balance_token_b / self.current_price // Use inverted price for non-stablecoin B
            } else {
                self.balance_token_b * self.fetch_external_multiplier(token_b) // Placeholder for other cases
            }
        } else {
            0.0
        }
    }

    pub fn balance_total_usd(&self) -> f64 {
        self.balance_token_a_usd() + self.balance_token_b_usd()
    }

    pub fn balance_token_a_percentage(&self) -> f64 {
        self.balance_token_a_usd() / self.balance_total_usd() * 100.0
    }

    pub fn balance_token_b_percentage(&self) -> f64 {
        self.balance_token_b_usd() / self.balance_total_usd() * 100.0
    }

    pub fn yield_token_a_usd(&self) -> f64 {
        if let (Some(token_a), Some(token_b)) = (&self.token_a, &self.token_b) {
            if token_a.is_stablecoin {
                self.yield_token_a // Stablecoin A has 1:1 USD value
            } else if token_b.is_stablecoin {
                self.yield_token_a * self.current_price // Use current price for non-stablecoin A
            } else {
                self.yield_token_a * self.fetch_external_multiplier(token_a) // Placeholder for other cases
            }
        } else {
            0.0
        }
    }
    
    pub fn yield_token_b_usd(&self) -> f64 {
        if let (Some(token_a), Some(token_b)) = (&self.token_a, &self.token_b) {
            if token_b.is_stablecoin {
                self.yield_token_b // Stablecoin B has 1:1 USD value
            } else if token_a.is_stablecoin {
                self.yield_token_b / self.current_price // Use inverted price for non-stablecoin B
            } else {
                self.yield_token_b * self.fetch_external_multiplier(token_b) // Placeholder for other cases
            }
        } else {
            0.0
        }
    }
    
    pub fn yield_total_usd(&self) -> f64 {
        self.yield_token_a_usd() + self.yield_token_b_usd()
    }

    fn fetch_external_multiplier(&self, _token: &Token) -> f64 {
        // Fetch external multiplier from external source
        // need to implement
        1.0
    }

    pub fn calculate_range(&self, tick_index: i32) -> f64 {
        if let (Some(token_a), Some(token_b)) = (&self.token_a, &self.token_b) {
            let base_multiplier = if token_a.is_stablecoin && token_b.is_stablecoin {
                1.0 // Skip multiplier for stablecoin-stablecoin pools
            } else {
                1000.0 // Apply multiplier for other pools
            };
            1.0001f64.powi(tick_index) * base_multiplier
        } else {
            // Default to no multiplier if token metadata is missing
            1.0001f64.powi(tick_index)
        }
    }

    pub async fn update_prices(
        &mut self,
        pool: OrcaPoolInfo,
        position: OrcaPositionInfo,
    ) -> anyhow::Result<Self> {
        self.current_price = pool.price;
        self.tick_spacing = pool.tick_spacing;
        self.sqrt_price = pool.sqrt_price;
    
        let token_a = Token::from_mint_address(&pool.token_mint_a).await?;
        let token_b = Token::from_mint_address(&pool.token_mint_b).await?;
    
        self.token_a = Some(token_a.clone());
        self.token_b = Some(token_b.clone());

        self.range_lower = self.calculate_range(position.tick_lower_index);
        self.range_upper = self.calculate_range(position.tick_upper_index);

        // let price_tick_info = PriceTickInfo {
        //     sqrt_price: pool.sqrt_price,
        //     tick_lower_index: position.tick_lower_index,
        //     tick_upper_index: position.tick_upper_index,
        // };

        let close_position_instruction = Orca::get_close_position_instructions(
            RpcMode::conservative(),
            self.position_mint.clone(),
            self.wallet_key.clone(),
            None,
        ).await?;
    
        // Scale raw balances using decimals
        self.balance_token_a = close_position_instruction.quote.token_est_a as f64 
            / 10u64.pow(token_a.decimals as u32) as f64;
    
        self.balance_token_b = close_position_instruction.quote.token_est_b as f64 
            / 10u64.pow(token_b.decimals as u32) as f64;
    
        // Scale raw yields using decimals
        self.yield_token_a = close_position_instruction.fees_quote.fee_owed_a as f64 
            / 10u64.pow(token_a.decimals as u32) as f64;
    
        self.yield_token_b = close_position_instruction.fees_quote.fee_owed_b as f64 
            / 10u64.pow(token_b.decimals as u32) as f64;
    
        // Call helper methods for derived values
        self.balance_token_a_usd = self.balance_token_a_usd();
        self.balance_token_b_usd = self.balance_token_b_usd();
        self.balance_total_usd = self.balance_total_usd();
    
        self.balance_token_a_percentage = self.balance_token_a_percentage();
        self.balance_token_b_percentage = self.balance_token_b_percentage();
    
        self.yield_token_a_usd = self.yield_token_a_usd();
        self.yield_token_b_usd = self.yield_token_b_usd();
        self.yield_total_usd = self.yield_total_usd();
    
        self.rewards_owed = close_position_instruction.rewards_quote;
        // self.in_range = close_position_instruction.in_range.unwrap_or(false);
        self.updated_at = Utc::now();
    
        Ok(self.clone())
    }

    pub async fn toggle_auto_rebalance(&mut self) -> anyhow::Result<()> {
        self.auto_rebalance = !self.auto_rebalance;
        let mut managed_positions = POOL_MANAGER.get().lock().await.clone().managed_positions;
        
        for position in managed_positions.iter_mut() {
            if position.address == self.address {
                position.auto_rebalance = self.auto_rebalance;
            }
        }
        POOL_MANAGER.get().lock().await.managed_positions = managed_positions;

        PoolManager::fetch_and_update_managed_positions(0).await?;

        Ok(())
    }
    
    // pub async fn rebalance(&self) -> anyhow::Result<()> {
    //     // Retrieve historical data
    //     let price_history_last_five_seconds = TickerState::get_history(TimePeriod::FiveSeconds)?;
    //     let price_history_last_ten_seconds = TickerState::get_history(TimePeriod::TenSeconds)?;
    //     let price_history_last_fifteen_seconds = TickerState::get_history(TimePeriod::FifteenSeconds)?;
    //     let price_history_last_thirty_seconds = TickerState::get_history(TimePeriod::ThirtySeconds)?;
    //     let price_history_last_one_minute = TickerState::get_history(TimePeriod::OneMinute)?;
    //     let price_history_last_five_minutes = TickerState::get_history(TimePeriod::FiveMinutes)?;

    //     // Compute average prices for each time period
    //     let avg_five_seconds = calculate_average_price(&price_history_last_five_seconds);
    //     let avg_ten_seconds = calculate_average_price(&price_history_last_ten_seconds);
    //     let avg_fifteen_seconds = calculate_average_price(&price_history_last_fifteen_seconds);
    //     let avg_thirty_seconds = calculate_average_price(&price_history_last_thirty_seconds);
    //     let avg_one_minute = calculate_average_price(&price_history_last_one_minute);
    
    //     // Determine trends
    //     let potential_bear_run = avg_five_seconds < avg_ten_seconds
    //         && avg_ten_seconds < avg_fifteen_seconds
    //         && avg_fifteen_seconds < avg_thirty_seconds
    //         && avg_thirty_seconds < avg_one_minute;
    
    //     let potential_bull_run = avg_five_seconds > avg_ten_seconds
    //         && avg_ten_seconds > avg_fifteen_seconds
    //         && avg_fifteen_seconds > avg_thirty_seconds
    //         && avg_thirty_seconds > avg_one_minute;
    
    //     // Log the results
    //     if potential_bull_run {
    //         println!("Potential bull run detected!");
    //     } else if potential_bear_run {
    //         println!("Potential bear run detected!");
    //     } else {
    //         println!("No significant trend detected.");
    //     }

    //      // Calculate volatility
    //     let volatility_last_five_seconds = calculate_volatility(&price_history_last_five_seconds);
    //     let volatility_last_ten_seconds = calculate_volatility(&price_history_last_ten_seconds);
    //     let volatility_last_fifteen_seconds = calculate_volatility(&price_history_last_fifteen_seconds);
    //     let volatility_last_thirty_seconds = calculate_volatility(&price_history_last_thirty_seconds);
    //     let volatility_last_minute = calculate_volatility(&price_history_last_one_minute);
    //     let volatility_last_five_minutes = calculate_volatility(&price_history_last_five_minutes);

    //     // Define a volatility threshold (tunable parameter)
    //     let volatility_threshold = 1.0; // Example value

    //     if volatility_last_five_seconds > volatility_threshold {
    //         println!(
    //             "High volatility detected in the last five seconds! Volatility: {:.2}",
    //             volatility_last_five_seconds
    //         );
    //     }

    //     if volatility_last_ten_seconds > volatility_threshold {
    //         println!(
    //             "High volatility detected in the last ten seconds! Volatility: {:.2}",
    //             volatility_last_ten_seconds
    //         );
    //     }

    //     if volatility_last_fifteen_seconds > volatility_threshold {
    //         println!(
    //             "High volatility detected in the last fifteen seconds! Volatility: {:.2}",
    //             volatility_last_fifteen_seconds
    //         );
    //     }

    //     if volatility_last_thirty_seconds > volatility_threshold {
    //         println!(
    //             "High volatility detected in the last thirty seconds! Volatility: {:.2}",
    //             volatility_last_thirty_seconds
    //         );
    //     }

    //     if volatility_last_minute > volatility_threshold {
    //         println!(
    //             "High volatility detected in the last minute! Volatility: {:.2}",
    //             volatility_last_minute
    //         );
    //     }

    //     if volatility_last_five_minutes > volatility_threshold {
    //         println!(
    //             "High volatility detected in the last five minutes! Volatility: {:.2}",
    //             volatility_last_five_minutes
    //         );
    //     }

    //     // Additional logic for adjusting range based on volatility
    //     if volatility_last_minute > volatility_threshold || volatility_last_five_minutes > volatility_threshold {
    //         println!("Consider increasing range or adjusting strategy.");
    //     }

    //     let new_position = NewProgrammaticPosition {
    //         pool_type: self.pool_type.clone(),
    //         pool_address: self.pool_address.clone(),
    //         token_mint_a: self.token_a.clone().unwrap().address,
    //         token_mint_b: self.token_b.clone().unwrap().address,
    //     };

    //     PoolManager::open_programmatic_position(new_position.clone()).await?;

    //     // let (new_lower_range, new_upper_range) = calculate_new_range(
    //     //     self.current_price,
    //     //     volatility_last_minute,
    //     //     avg_one_minute,
    //     //     volatility_last_five_minutes,
    //     // );
    
    //     // // Retrieve tokens
    //     // let token_a = self.token_a.clone().ok_or(anyhow::anyhow!("Token A not found"))?;
    //     // let token_b = self.token_b.clone().ok_or(anyhow::anyhow!("Token B not found"))?;
    
    //     // Get wallet balances
    //     // let (balance_a_amount, balance_a_ui_amount) = Wallet::get_token_balance(
    //     //     &self.wallet_key,
    //     //     &token_a.address,
    //     //     RpcMode::fast(),
    //     // ).await.unwrap_or((0, 0.0));
    
    //     // let (balance_b_amount, balance_b_ui_amount) = Wallet::get_token_balance(
    //     //     &self.wallet_key,
    //     //     &token_b.address,
    //     //     RpcMode::fast(),
    //     // ).await.unwrap_or((0, 0.0));
    
    //     // // Calculate balances after accounting for transaction fees
    //     // let estimated_sol_needed_for_tx = 0.01; // Estimated SOL required for transaction
    //     // let est_lamports_needed = (estimated_sol_needed_for_tx * 1_000_000_000.0) as u64;
    
    //     // let available_balance_token_a = if token_a.address == Token::solana().address {
    //     //     balance_a_amount.saturating_sub(est_lamports_needed)
    //     // } else {
    //     //     balance_a_amount
    //     // };
    
    //     // let available_balance_token_b = if token_b.address == Token::solana().address {
    //     //     balance_b_amount.saturating_sub(est_lamports_needed)
    //     // } else {
    //     //     balance_b_amount
    //     // };
    
    //     // // Total available balance with slippage considered
    //     // let slippage = 2.5; // Slippage tolerance in percentage
    //     // let total_available_balance = ((available_balance_token_a + available_balance_token_b) as f64
    //     //     * (100.0 - slippage) / 100.0) as u64;
    
    //     // // Determine required balances for equal distribution
    //     // let balance_needed_token_a = total_available_balance / 2;
    //     // let balance_needed_token_b = total_available_balance / 2;
    
    //     // // Perform token swap if necessary
    //     // if balance_needed_token_a > available_balance_token_a || balance_needed_token_b > available_balance_token_b {
    //     //     let (balance_needed, token_mint) = if balance_needed_token_a > available_balance_token_a {
    //     //         (balance_needed_token_a - available_balance_token_a, token_a.address)
    //     //     } else {
    //     //         (balance_needed_token_b - available_balance_token_b, token_b.address)
    //     //     };
    
    //     //     let token_swap = TokenSwap {
    //     //         wallet_key: self.wallet_key.clone(),
    //     //         pool_address: self.pool_address.clone(),
    //     //         amount: balance_needed,
    //     //         amount_is_in: false,
    //     //         mint_out_address: token_mint,
    //     //         slippage_tolerance: Some(slippage as u16),
    //     //     };

    //     //     println!("need to swap: {:?}", token_swap);
    
    //     //     let swap_instruction = Orca::get_swap_instructions(token_swap).await?;
    //     //     let instructions_array: Vec<Instruction> = swap_instruction
    //     //         .instructions
    //     //         .iter()
    //     //         .map(|sol_instr| Orca::convert_to_instruction(sol_instr))
    //     //         .collect::<Result<Vec<_>, _>>()?;
    //     //     let recent_blockhash = Rpc::get_latest_blockhash(RpcMode::fast()).await?;
    //     //     let payer = Pubkey::from_str(&self.wallet_key)?;
    //     //     let message = Message::new_with_blockhash(
    //     //         &instructions_array,
    //     //         Some(&payer),
    //     //         &recent_blockhash,
    //     //     );
            
    //     //     // Build the signer list
    //     //     // let wallet_keypair = Keypair::from_base58_string(&self.wallet_key);
    //     //     let wallet_keypair = Wallet::get_programmatic_keypair().map_err(|e| anyhow::anyhow!("Error getting programmatic keypair: {:?}", e))?;
    //     //     let mut signers: Vec<&dyn Signer> = vec![&wallet_keypair];
            
    //     //     let additional_keypairs: Vec<Keypair> = swap_instruction
    //     //     .additional_signers
    //     //     .iter()
    //     //     .map(|s| {
    //     //         let decoded = BASE64_STANDARD.decode(s).map_err(|e| {
    //     //             anyhow::anyhow!("Failed to decode signer: {:?}, error: {}", s, e)
    //     //         })?;
    //     //         Keypair::from_bytes(&decoded).map_err(|e| {
    //     //             anyhow::anyhow!("Failed to create Keypair from bytes: {:?}", e)
    //     //         })
    //     //     })
    //     //     .collect::<Result<Vec<_>, _>>()?;
        
    //     //     let additional_signers: Vec<&dyn Signer> = additional_keypairs.iter().map(|kp| kp as &dyn Signer).collect();
    //     //     signers.extend(additional_signers);
            
    //     //     // Create and send the transaction
    //     //     let transaction = Transaction::new(
    //     //         &signers,
    //     //         message,
    //     //         recent_blockhash,
    //     //     );
            
    //     //     // let signature = Rpc::send_and_confirm_transaction(RpcMode::fast(), &transaction).await?;
    //     //     // println!("Transaction confirmed with signature: {}", signature);
    //     // }

    //     // println!("balance_needed_token_a: {}, balance_needed_token_b: {}", balance_needed_token_a, balance_needed_token_b);
    
    //     // Create a new position
    //     // let new_position = NewPosition {
    //     //     wallet: Wallet {
    //     //         pubkey: self.wallet_key.clone(),
    //     //         name: "Programmatic".to_string(), // Example: Adjust as needed
    //     //     },
    //     //     pool_type: self.pool_type.clone(),
    //     //     range_lower: new_lower_range,
    //     //     range_upper: new_upper_range,
    //     //     pool_address: self.pool_address.clone(),
    //     //     amount_a: balance_needed_token_a,
    //     //     amount_b: balance_needed_token_b,
    //     //     amount_total: total_available_balance,
    //     //     wallet_balance_token_a: balance_a_ui_amount,
    //     //     wallet_balance_token_b: balance_b_ui_amount,
    //     //     wallet_balance_total: (balance_a_ui_amount + balance_b_ui_amount),
    //     // };

    //     // println!("New Range: Lower: {:.2}, Upper: {:.2}", new_lower_range, new_upper_range);

    //     // new_position.open().await?;
    
    //     Ok(())
    // }

    pub async fn should_rebalance(&mut self) -> anyhow::Result<bool> {
        let current_ticker_price = TickerState::get_current_price()?;
        self.current_ticker_price = current_ticker_price;
        let range_lower = self.range_lower;
        let range_upper = self.range_upper;

        let middle_of_range = (range_lower + range_upper) / 2.0;

        let range_state = if current_ticker_price < range_lower {
            let range_score = (range_lower - current_ticker_price) / (range_lower - middle_of_range);
            if self.out_of_range_start.is_none() {
                self.out_of_range_start = Some(Utc::now());
            }
            RangeState::OutUnder(range_score)
        } else if current_ticker_price > range_upper {
            let range_score = (current_ticker_price - range_upper) / (middle_of_range - range_upper);
            if self.out_of_range_start.is_none() {
                self.out_of_range_start = Some(Utc::now());
            }
            RangeState::OutOver(range_score)
        } else if current_ticker_price < middle_of_range {
            let range_score = (middle_of_range - current_ticker_price) / (middle_of_range - range_lower);
            self.out_of_range_start = None;
            RangeState::InLower(range_score)
        } else if (current_ticker_price > middle_of_range) {
            let range_score = (current_ticker_price - middle_of_range) / (range_upper - middle_of_range);
            self.out_of_range_start = None;
            RangeState::InHigher(range_score)
        } else {
            self.out_of_range_start = None;
            RangeState::Centered

        };

        score_to_color!(range_state.get_score(), format!("Range State: {:?}", range_state));

        // self.range_state_history.push(range_state.clone());

        let time_position_active_sec = Utc::now().signed_duration_since(self.created_at).num_seconds();

        let should_rebalance = matches!(range_state, RangeState::OutUnder(_) | RangeState::OutOver(_))
            || match range_state {
                RangeState::InLower(score) if score > 0.95 && time_position_active_sec > 60 => true,
                RangeState::InHigher(score) if score > 0.95 && time_position_active_sec > 60 => true,
                _ => false,
            };

        if should_rebalance {
            // make sure pool price is also outside of range
            self.current_price = Orca::get_pool_price(RpcMode::fast(), &self.pool_address.clone()).await?;
            println!("Ticker price out of range. Current Pool Price: {}", self.current_price);
            if self.current_price < range_lower || self.current_price > range_upper {

                println!("Pool price out of range. Rebalancing position for wallet: {}", self.wallet_key);

                return Ok(true)
            }
        }

        Ok(false)
       
    }



    pub async fn close(&self) -> anyhow::Result<()> {
        let start = Utc::now();
        blue!("Getting close position instructions");
        let close_position_instruction = Orca::get_close_position_instructions(
            RpcMode::fast(),
            self.position_mint.clone(),
            self.wallet_key.clone(),
            None).await?;
        green!("Got close position instructions in {}", start.signed_duration_since(Utc::now()).num_milliseconds());

        // let token_min_a = close_position_instruction.quote.token_min_a;
        // let token_min_b = close_position_instruction.quote.token_min_b;

        // println!("Token min a: {} - Token min b: {}", token_min_a, token_min_b);

        // Double check if price is back in range
        let should_rebalance = self.clone().should_rebalance().await?;

        if !should_rebalance {
            let mut pool_manager_lock = POOL_MANAGER.get().lock().await;
            pool_manager_lock.position_to_close = None;
            drop(pool_manager_lock);
            return Err(anyhow::anyhow!("Position is not out of range, no need to close"));
        }

        let start = Utc::now();
        blue!("Closing position");
        
        let _signature = Orca::perform_orca_transaction(
            close_position_instruction.instructions,
            close_position_instruction.additional_signers,
            Some(PriorityLevel::High),
        ).await?;

        green!("Closed position in {}", start.signed_duration_since(Utc::now()).num_milliseconds());
        let font = FIGfont::standard().unwrap();
        let banner = font.convert("Closed Position").unwrap();
        green!("\n\n{}\n\n", banner);

        Ok(())
    }
    
}


fn calculate_average_price(history: &[TickerState]) -> f64 {
    if history.is_empty() {
        return 0.0;
    }
    let total: f64 = history.iter().map(|state| state.price).sum();
    total / history.len() as f64
}

fn calculate_volatility(history: &[TickerState]) -> f64 {
    if history.is_empty() {
        return 0.0;
    }

    let mean_price = calculate_average_price(history);
    let variance: f64 = history
        .iter()
        .map(|state| (state.price - mean_price).powi(2))
        .sum::<f64>()
        / history.len() as f64;

    variance.sqrt() // Standard deviation
}

fn calculate_new_range(
    current_price: f64,
    volatility: f64,
    target_price: f64,
    target_volatility: f64,
) -> (f64, f64) {
    let new_lower = current_price - (volatility * 2.0);
    let new_upper = current_price + (volatility * 2.0);

    if target_price > current_price {
        (new_lower, new_upper + (target_volatility * 2.0))
    } else {
        (new_lower - (target_volatility * 2.0), new_upper)
    }
}

