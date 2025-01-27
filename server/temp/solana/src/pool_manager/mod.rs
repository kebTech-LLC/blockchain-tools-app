use std::{str::FromStr, sync::Arc, time::Duration};

use chrono::{DateTime, Utc};
use kebtech_utils::*;
use message::{MessageType, PoolManagerMessage};
use new_position::{NewPosition, NewPositionData, NewProgrammaticPosition};
use orca::{token_swap::TokenSwap, Orca};
use orca_pools_ipc_types::response::{close_position_instruction::OrcaClosePositionInstruction, open_position_instruction::OrcaOpenPositionInstruction, orca_position_info::OrcaPositionInfo, orca_swap_instructions::OrcaSwapInstructions};
use position_manager::managed_position::{ManagedPosition, PoolType};
use serde::{Deserialize, Serialize};
use serde_json::json;
use solana_sdk::pubkey::Pubkey;
use state::InitCell;
use tokio::{sync::Mutex, time::interval};

use crate::{price_info::{coinbase::{ticker::TickerState, websocket::CoinbaseWebsocket}, price_checker::PriceChecker}, rpc::{Rpc, RpcMode}, token::Token, wallet::Wallet};

pub mod position_manager;
pub mod message;
pub mod new_position;
pub mod orca;
pub mod raydium;

pub static POOL_MANAGER: InitCell<Arc<Mutex<PoolManager>>> = InitCell::new();

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PoolManager {
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub active: bool,
    pub managed_positions: Vec<ManagedPosition>,
    pub local_wallet_pubkey: Option<Pubkey>,
    pub programmatic_wallet_pubkey: Option<Pubkey>,
    pub position_to_open: Option<NewProgrammaticPosition>,
    pub position_to_close: Option<ManagedPosition>,
    pub message_queue: Vec<PoolManagerMessage>,
}

impl PoolManager {
    pub fn new() -> Self {
        let programmatic_wallet_pubkey = match Wallet::get_programmatic_pubkey() {
            Ok(pubkey) => Some(pubkey),
            Err(e) => {
                red!("Failed to get programmatic wallet pubkey: {:?}", e);
                None
            }
        };
        let defi_wallet: Option<Pubkey> = match Wallet::get_stored_local_wallet_pubkey() {
            Ok(pubkey) => Some(pubkey),
            Err(e) => {
                red!("Failed to get stored local wallet pubkey: {:?}", e);
                None
            }
        };
        let mode = std::env::var("MODE").unwrap_or("passive".to_string());
        let active = mode == "active";
        PoolManager {
            created: Utc::now(),
            updated: Utc::now(),
            active,
            managed_positions: Vec::new(),
            local_wallet_pubkey: defi_wallet,
            programmatic_wallet_pubkey,
            position_to_open: None,
            message_queue: Vec::new(),
            position_to_close: None,
        }
    }

    pub async fn start(tx: tokio::sync::mpsc::Sender<PoolManagerMessage>) -> anyhow::Result<()> {
        let pool_manager = Arc::new(Mutex::new(PoolManager::new()));
        POOL_MANAGER.set(pool_manager);
        Token::initiate_token_store();
        TickerState::init();
        PriceChecker::init();
        NewPositionData::init();
        
        tokio::spawn(async {
            match CoinbaseWebsocket::start().await {
                Ok(_) => (),
                Err(e) => red!("Failed to start Coinbase websocket: {:?}", e),
            }
        });

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(1));

            loop {
                interval.tick().await;
                let mut pool_manager = POOL_MANAGER.get().lock().await;
                let messages: Vec<_> = pool_manager.message_queue.drain(..).collect(); // Drain the queue

                drop(pool_manager);
                for message in messages {
                    if let Err(e) = tx.clone().send(message).await {
                        red!("Failed to send message: {:?}", e);
                    }
                }
            }
          
        });

        tokio::spawn(async move {
            let frequency_seconds = 30;
            let mut interval = interval(Duration::from_secs(frequency_seconds));

            loop {
                interval.tick().await;

                match PoolManager::fetch_and_update_managed_positions(frequency_seconds).await {
                    Ok(_) => (),
                    Err(e) => red!("Failed to update managed positions: {:?}", e),
                }
            }
            
        });

        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(1));

            loop {
                interval.tick().await;

                match Self::analyze_managed_positions().await {
                    Ok(_) => (),
                    Err(e) => red!("Failed to analyze managed positions: {:?}", e),
                }
            }
            
        });
       
        let mut interval = interval(Duration::from_secs(1));
    
        // Main loop
        loop {

            interval.tick().await;

            {
                let pool_manager_lock = POOL_MANAGER.get().lock().await;
                let pool_manager = pool_manager_lock.clone();
                drop(pool_manager_lock);
                
                if !pool_manager.active {
                    continue;
                }

                if let Some(position_to_close) = pool_manager.position_to_close.clone() {
                    match position_to_close.close().await {
                        Ok(_) => {
                            let mut pool_manager_lock = POOL_MANAGER.get().lock().await;
                            pool_manager_lock.position_to_close = None;
                            drop(pool_manager_lock);
                            println!("removed position from close queue");
                            let new_position = NewProgrammaticPosition::from_managed_position(&position_to_close)?;
                            PoolManager::queue_programmatic_open(new_position.clone()).await?;
                            println!("added position to open queue");
                            
                        },
                        Err(e) => {
                            let error_message = format!("{:?}", e);
                            if error_message.contains("AccountNotFound") {
                                blue!("Skipping iteration due to AccountNotFound error.");
                                continue; // Skip the rest of this loop iteration
                            } else {
                                red!("Failed to close position: {:?}", e);
                                // Handle other errors as needed
                            }
                        },
                    }
                }
            }

            {
                let pool_manager_lock = POOL_MANAGER.get().lock().await;
                let pool_manager = pool_manager_lock.clone();
                drop(pool_manager_lock);

                if !pool_manager.active {
                    continue;
                }

                if let Some(position_to_open) = pool_manager.position_to_open.clone() {
                    NewPositionData::set_token_amounts(&position_to_open).await?;
                    let position = position_to_open.clone();
                    tokio::spawn(async move {
                        NewPositionData::pool_price_loop(&position).await;
                    });

                    match position_to_open.open().await {
                        Ok(_) => {
                            let mut pool_manager_lock = POOL_MANAGER.get().lock().await;
                            pool_manager_lock.position_to_open = None;
                        },
                        Err(e) => red!("Failed to open programmatic position: {:?}", e),
                    }
                }
            }
          
        }
    }

    pub async fn fetch_and_update_managed_positions(frequency_seconds: u64) -> anyhow::Result<()> {
         // Start timestamp for performance measurement
        let start_timestamp = Utc::now();
        blue!("\nChecking for new positions...\n");
    
        // Clone necessary data under a scoped lock
        let (local_wallet_pubkey, programmatic_wallet_pubkey, mut managed_positions) = {
            let pool_manager_lock = POOL_MANAGER.get().lock().await;
            let pool_manager = pool_manager_lock.clone();

            drop(pool_manager_lock);
            
            (
                pool_manager.local_wallet_pubkey,
                pool_manager.programmatic_wallet_pubkey,
                pool_manager.managed_positions.clone(),
            )
        };
    
        // Fetch positions for wallets
        let mut orca_positions: Vec<OrcaPositionInfo> = vec![];
    
        if let Some(pubkey) = local_wallet_pubkey {
            let local_orca_positions = Orca::get_positions_for_wallet(pubkey.to_string()).await?;
            orca_positions.extend(local_orca_positions);
        }
    
        if let Some(pubkey) = programmatic_wallet_pubkey {
            let programmatic_orca_positions = Orca::get_positions_for_wallet(pubkey.to_string()).await?;
            orca_positions.extend(programmatic_orca_positions);
        }
    
        let mut message_queue = vec![];
    
        // Retain and process existing positions
        managed_positions.retain(|position| {
            if position.pool_type == PoolType::Orca {
                let exists_in_orca = orca_positions.iter().any(|orca_position| orca_position.address == position.address);
    
                if !exists_in_orca {
                    message_queue.push(PoolManagerMessage {
                        message_type: MessageType::RemovePosition,
                        data: Some(json!(position.clone())),
                        frequency_seconds,
                    });
                    false // Remove this position
                } else {
                    true // Retain this position
                }
            } else {
                true // Retain non-Orca positions
            }
        });
    
        // Process new or updated positions
        for position in orca_positions {
            if let Some(existing_position) = managed_positions.iter_mut().find(|p| p.address == position.address) {
                // println!("Position exists in PoolManager");
                let pool = Orca::get_clp_pool(
                    RpcMode::conservative(),
                    &existing_position.token_a.as_ref().unwrap().address,
                    &existing_position.token_b.as_ref().unwrap().address,
                    existing_position.tick_spacing,
                )
                .await?;
                existing_position.update_prices(pool, position.clone()).await?;
    
                message_queue.push(PoolManagerMessage {
                    message_type: MessageType::UpdatePosition,
                    data: Some(json!(existing_position.clone())),
                    frequency_seconds,
                });
            } else {
                // New position
                let tokens_and_tick = Orca::get_tokens_and_tick(RpcMode::conservative(), &position.whirlpool_address).await?;
                let pool = Orca::get_clp_pool(
                    RpcMode::conservative(),
                    &tokens_and_tick.token_a,
                    &tokens_and_tick.token_b,
                    tokens_and_tick.tick_spacing,
                )
                .await?;

                let created_at = Rpc::get_account_creation_date(
                    RpcMode::conservative(), 
                    &position.address, 
                    Some(10000)
                ).await.unwrap_or(Utc::now());
    
                let mut managed_position = ManagedPosition::from_orca_position_info(position.clone(), created_at);
                managed_position = managed_position.update_prices(pool, position.clone()).await?;

                managed_positions.push(managed_position.clone());
    
                message_queue.push(PoolManagerMessage {
                    message_type: MessageType::UpdatePosition,
                    data: Some(json!(managed_position.clone())),
                    frequency_seconds,
                });
            }
        }
    
        // Update the pool manager state and message queue under a scoped lock
        {
            let mut pool_manager = POOL_MANAGER.get().lock().await;
            pool_manager.managed_positions = managed_positions;
            pool_manager.message_queue.extend(message_queue);
            pool_manager.updated = Utc::now();

            drop(pool_manager);
        }
    
        let end_timestamp = Utc::now();
        blue!("\nManaged positions updated in {} seconds\n", (end_timestamp - start_timestamp).num_seconds());
    
        Ok(())
    }

    pub async fn is_rebalancing() -> bool {
        let pool_manager_lock = POOL_MANAGER.get().lock().await;
        let pool_manager = pool_manager_lock.clone();
        drop(pool_manager_lock);
        if pool_manager.position_to_close.is_some() || pool_manager.position_to_open.is_some() {
            return true;
        } else {
            return false
        }
    }
    
    pub async fn add_to_message_queue(message: PoolManagerMessage) -> anyhow::Result<()> {
        let mut pool_manager = POOL_MANAGER.get().lock().await;

        pool_manager.message_queue.push(message);

        drop(pool_manager);

        Ok(())
    }

    pub async fn clear_message_queue() -> anyhow::Result<()> {
        let mut pool_manager = POOL_MANAGER.get().lock().await;
        pool_manager.message_queue.clear();

        drop(pool_manager);

        Ok(())
    }
    
    pub async fn get_managed_positions() -> anyhow::Result<Vec<ManagedPosition>> {
        let pool_manager_lock = POOL_MANAGER.get().lock().await;
        let pool_manager = pool_manager_lock.clone();
        drop(pool_manager_lock);
        
        Ok(pool_manager.managed_positions)
    }

    pub async fn get_positions_for_wallet(wallet_key: &str) -> anyhow::Result<Vec<ManagedPosition>> {
        let orca_positions = Orca::get_positions_for_wallet(wallet_key.to_string()).await?;
        
        let mut managed_positions = vec![];
        
        for position in orca_positions {
            let tokens_and_tick = Orca::get_tokens_and_tick(RpcMode::conservative(), &position.whirlpool_address).await?;
            let pool = Orca::get_clp_pool(
                RpcMode::conservative(),
                &tokens_and_tick.token_a,
                &tokens_and_tick.token_b,
                tokens_and_tick.tick_spacing,
            )
            .await?;

            let created_at = Rpc::get_account_creation_date(
                RpcMode::conservative(), 
                &position.address, 
                Some(10000)
            ).await.unwrap_or(Utc::now());

            let mut managed_position = ManagedPosition::from_orca_position_info(position.clone(), created_at);
            managed_position = managed_position.update_prices(pool, position.clone()).await?;
            managed_positions.push(managed_position);
        }

        Ok(managed_positions)
    }

    pub async fn set_local_wallet_pubkey(wallet_key: String) -> anyhow::Result<Vec<ManagedPosition>> {
        let wallet_key = Pubkey::from_str(&wallet_key).map_err(|e| anyhow::anyhow!("Error parsing wallet key: {:?}", e))?;
        let managed_positions = Self::get_positions_for_wallet(&wallet_key.to_string()).await?;

        let mut pool_manager = POOL_MANAGER.get().lock().await;

        pool_manager.local_wallet_pubkey = Some(wallet_key);

        for position in managed_positions.clone() {
            if !pool_manager.managed_positions.iter().any(|p| p.address == position.address) {
                pool_manager.managed_positions.push(position);
            }
        }

        println!("Managed positions length: {}", pool_manager.managed_positions.len());

        drop(pool_manager);

        Ok(managed_positions)
    }

    pub async fn analyze_managed_positions() -> anyhow::Result<()> {
        if PoolManager::is_rebalancing().await {
            return Ok(());
        }
        let managed_positions = PoolManager::get_managed_positions().await?;
        
        for mut position in managed_positions {
            if Wallet::is_programmatic_wallet(&position.wallet_key.clone())? {
                if position.should_rebalance().await? {
                    println!("Closing position for wallet: {}", position.wallet_key);
                    PoolManager::queue_programmatic_close(position.clone()).await?;
                    // let new_position = NewProgrammaticPosition::from_managed_position(&position)?;
                    // PoolManager::queue_programmatic_open(new_position).await?;
                    // PoolManager::queue_programmatic_close(position.clone()).await?;
                    
                    // position.rebalance().await?;

                    // PoolManager::close_position(position.clone()).await?;
                    // position.close().await?;

                    // println!("Rebalancing position for wallet: {}", position.wallet_key);
                    // position.rebalance().await?;
                }
            }
        }

        Ok(())
    }
    pub async fn unset_local_wallet_pubkey() -> anyhow::Result<Vec<ManagedPosition>> {
        let mut pool_manager = POOL_MANAGER.get().lock().await;

        let wallet_key = pool_manager.local_wallet_pubkey.clone().ok_or_else(|| anyhow::anyhow!("Local wallet pubkey not set"))?.to_string();
        pool_manager.local_wallet_pubkey = None;
        let positions_to_remove = pool_manager.managed_positions.iter().filter(|p| p.wallet_key == wallet_key).cloned().collect();

        pool_manager.managed_positions.retain(|p| p.wallet_key != wallet_key);

        drop(pool_manager);

        Ok(positions_to_remove)
    }

    pub async fn open_position(new_position: NewPosition) -> anyhow::Result<OrcaOpenPositionInstruction> {
        blue!("Opening position with data: {:?}", new_position);

        let open_position_instruction = Orca::get_open_position_instructions(new_position).await?;

        Ok(open_position_instruction)
    }

    pub async fn queue_programmatic_open(new_position: NewProgrammaticPosition) -> anyhow::Result<()> {
        blue!("queuing new programmatic position ");
        
        let mut pool_manager = POOL_MANAGER.get().lock().await;

        pool_manager.position_to_open = Some(new_position.clone());

        drop(pool_manager);

        Ok(())
    }

    pub async fn queue_programmatic_close(managed_position: ManagedPosition) -> anyhow::Result<()> {
        // println!("closing prog position ");
        
        // managed_position.close().await?;

        blue!("adding to close queue");
        let mut pool_manager = POOL_MANAGER.get().lock().await;

        pool_manager.position_to_close = Some(managed_position.clone());

        drop(pool_manager);

        Ok(())
    }

    pub async fn close_position(managed_position: ManagedPosition) -> anyhow::Result<OrcaClosePositionInstruction> {
        blue!("Closing position with data: {:?}", managed_position);

        let close_position_instruction = Orca::get_close_position_instructions(
            RpcMode::fast(),
            managed_position.address.clone(),
            managed_position.wallet_key.clone(),
            None
        ).await?;

        Ok(close_position_instruction)
    }

    pub async fn swap_tokens(token_swap: TokenSwap) -> anyhow::Result<OrcaSwapInstructions> {
        blue!("Swapping tokens with data: {:?}", token_swap);

        let swap_instructions = Orca::get_swap_instructions(token_swap).await?;

        Ok(swap_instructions)
    }
    
}

