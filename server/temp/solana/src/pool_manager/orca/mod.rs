pub mod token_swap;

use std::{str::FromStr, time::Instant};

use chrono::Utc;
use helius::types::PriorityLevel;
use kebtech_utils::*;
use base64::{prelude::BASE64_STANDARD, Engine};
use orca_pools_ipc_types::{request::{close_position_request::{ClosePositionRequest, PriceTickInfo}, new_position_request::NewPositionRequest, swap_request::{SwapAmount, SwapRequest}, Request, TokenAmount}, response::{close_position_instruction::OrcaClosePositionInstruction, open_position_instruction::OrcaOpenPositionInstruction, orca_pool_info::{OrcaPoolInfo, OrcaPoolTokensAndTick}, orca_position_info::OrcaPositionInfo, orca_swap_instructions::OrcaSwapInstructions, Response}, solana::SolanaInstruction};
use solana_sdk::{instruction::{AccountMeta, Instruction}, pubkey::Pubkey, signature::{Keypair, Signature}, signer::Signer};
use token_swap::TokenSwap;

use crate::{rpc::{Rpc, RpcMode}, wallet::{programmatic_transaction::ProgrammaticTransaction, Wallet}, };

use super::new_position::NewPosition;

pub struct Orca;

impl Orca {
    pub async fn get_open_position_instructions(new_position: NewPosition) -> anyhow::Result<OrcaOpenPositionInstruction> {
        let token_amount = if new_position.amount_a > new_position.amount_b {
            TokenAmount::TokenA(new_position.amount_a as u64)
        } else {
            TokenAmount::TokenB(new_position.amount_b as u64)
        };

        let new_position_clone = new_position.clone();
        let response = Rpc::call_orca(
            RpcMode::fast(),
            move |url| {
                let new_position = new_position_clone.clone();
                let new_position_request = NewPositionRequest::new(
                    url,
                    new_position.wallet.pubkey.to_string(),
                    new_position.pool_address.clone(),
                    token_amount.clone(),
                    Some(250),
                    new_position.range_lower,
                    new_position.range_upper,
                );
                Request::GetOpenPositionInstruction { new_position_request }
            },
            Some(20000)
        ).await?;

        let open_position_instruction = match response {
            Response::OpenPositionInstruction(instruction) => instruction,
            _ => return Err(anyhow::anyhow!("Unexpected response: {:?}", response)),
        };

        Ok(open_position_instruction)
    }

    pub async fn get_prog_open_position_instructions(
        pool_address: &str,
        token_amount_b: u64,
        slippage: u16,
        range_lower: f64,
        range_upper: f64,
    ) -> anyhow::Result<OrcaOpenPositionInstruction> {
        let token_amount = TokenAmount::TokenB(token_amount_b);
        println!("token_amount: {:?}", token_amount);
        let wallet_key = Wallet::get_programmatic_pubkey()?.to_string();
        let pool_address = pool_address.to_string();
        let response = Rpc::call_orca(
            RpcMode::fast(),
            move |url| {
                let new_position_request = NewPositionRequest::new(
                    url,
                    wallet_key.clone(),
                    pool_address.clone(),
                    token_amount.clone(),
                    Some(slippage),
                    range_lower,
                    range_upper,
                );
                Request::GetOpenPositionInstruction { new_position_request }
            },
            Some(20000)
        ).await?;

        let open_position_instruction = match response {
            Response::OpenPositionInstruction(instruction) => instruction,
            _ => return Err(anyhow::anyhow!("Unexpected response: {:?}", response)),
        };

        Ok(open_position_instruction)
    }

    pub async fn get_close_position_instructions(
        mode: RpcMode,
        position_mint: String,
        wallet_key: String,
        price_tick_info: Option<PriceTickInfo>,
    ) -> anyhow::Result<OrcaClosePositionInstruction> {

        let response = Rpc::call_orca(
            mode,
            move |url| {
                let close_position_request = ClosePositionRequest::new(
                    url,
                    position_mint.clone(),
                    wallet_key.clone(),
                    price_tick_info.clone(),
                    Some(1000),
                );
                Request::GetClosePositionInstruction {
                    close_position_request,
                }
            },
            Some(20000)
        )
        .await?;
    
        match response {
            Response::ClosePositionInstruction(instruction) => Ok(instruction),
            _ => Err(anyhow::anyhow!("Unexpected response: {:?}", response)),
        }
    }

    pub async fn get_positions_for_wallet(wallet_key_str: String) -> anyhow::Result<Vec<OrcaPositionInfo>> {
        let response = Rpc::call_orca(
            RpcMode::conservative(),
            move |url| {
                Request::GetPositionsForWallet {
                    rpc_url: url,
                    wallet_key: wallet_key_str.clone(),
                }
            },
            Some(20000)
        ).await?;

        match response {
            Response::Positions(position_infos) => Ok(position_infos),
            _ => Err(anyhow::anyhow!("Unexpected response")),
        }
        
    }

    pub async fn get_swap_instructions(token_swap: TokenSwap) -> anyhow::Result<OrcaSwapInstructions> {
        let wallet_key = token_swap.wallet_key.clone();
        let pool_address = token_swap.pool_address.clone();
        let amount = token_swap.amount;
        let amount_is_in = token_swap.amount_is_in;
        let mint_out_address = token_swap.mint_out_address.clone();
        let slippage_tolerance = token_swap.slippage_tolerance;

        let response = Rpc::call_orca(
            RpcMode::fast(),
            move |url| {
                let swap_request = SwapRequest::new(
                    url,
                    wallet_key.clone(),
                    pool_address.clone(),
                    if amount_is_in {
                        SwapAmount::ExactIn(amount)
                    } else {
                        SwapAmount::ExactOut(amount)
                    },
                    mint_out_address.clone(),
                    slippage_tolerance,
                );
                    Request::GetSwapInstructions { swap_request }
                }, 
                Some(20000)).await?;

            match response {
                Response::SwapInstructions(instructions) => Ok(instructions),
                _ => return Err(anyhow::anyhow!("Unexpected response: {:?}", response)),
            }
    }

    pub async fn get_clp_pool(rpc_mode: RpcMode, token_a: &str, token_b: &str, tick_spacing: u16) -> anyhow::Result<OrcaPoolInfo> {
        let token_a = token_a.to_string();
        let token_b = token_b.to_string();
    
        let response = Rpc::call_orca(
            rpc_mode,
            move |url| {
                Request::GetClpPool { rpc_url: url, token_a: token_a.clone(), token_b: token_b.clone(), tick_spacing }
            },
            Some(20000)
        ).await?;
    
        match response {
            Response::PoolInfo(response) => Ok(response),
            _ => return Err(anyhow::anyhow!("Unexpected response"))
        }
    }

    pub async fn get_tokens_and_tick(rpc_mode: RpcMode,address: &str) -> anyhow::Result<OrcaPoolTokensAndTick> {
        let address = address.to_string();
    
        let response = Rpc::call_orca(
            rpc_mode,
            move |url| {
                Request::GetPoolTokensAndTick { rpc_url: url, whirlpool_address: address.clone() }
            },
            Some(20000)
        ).await?;
    
        match response {
            Response::PoolTokensAndTick(response) => Ok(response),
            _ => return Err(anyhow::anyhow!("Unexpected response"))
        }
    }

    pub fn convert_to_instruction(sol_instr: &SolanaInstruction) -> anyhow::Result<Instruction> {
        let program_id = Pubkey::from_str(&sol_instr.program_id)
            .map_err(|_| anyhow::anyhow!("Invalid program_id: {}", sol_instr.program_id))?;
    
        let accounts: Vec<AccountMeta> = sol_instr
            .accounts
            .iter()
            .map(|account| {
                Ok::<AccountMeta, anyhow::Error>(AccountMeta {
                    pubkey: Pubkey::from_str(&account.pubkey)
                        .map_err(|_| anyhow::anyhow!("Invalid pubkey: {}", account.pubkey))?,
                    is_signer: account.is_signer,
                    is_writable: account.is_writable,
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
    
        Ok(Instruction {
            program_id,
            accounts,
            data: sol_instr.data.clone(),
        })
    }
    
    pub fn solana_instructions_to_instructions(solana_instructions: &Vec<SolanaInstruction>) -> anyhow::Result<Vec<Instruction>> {
        let mut instructions = vec![];
    
        for solana_instruction in solana_instructions {
            let instruction = Orca::convert_to_instruction(solana_instruction)?;
            instructions.push(instruction);
        }
    
        Ok(instructions)
    }

    
    
    pub async fn get_pool_price(rpc_mode: RpcMode, pool_address: &str) -> anyhow::Result<f64> {
        let tokens_and_tick = Orca::get_tokens_and_tick(rpc_mode.clone(), &pool_address).await?;
            let pool = Orca::get_clp_pool(
                rpc_mode,
                &tokens_and_tick.token_a,
                &tokens_and_tick.token_b,
                tokens_and_tick.tick_spacing,
            )
            .await?;

        Ok(pool.price)
    }

    pub async fn perform_orca_transaction(
        instructions: Vec<SolanaInstruction>,
        additional_signer_strings: Vec<String>,
        priority_level: Option<PriorityLevel>,
        // pool_address: &str,
    ) -> anyhow::Result<Signature> {
        let instructions = Orca::solana_instructions_to_instructions(&instructions)?;
        let signers = ProgrammaticTransaction::get_all_signers(additional_signer_strings.clone())?;
        let transaction = ProgrammaticTransaction::new(instructions.clone(), signers).await?;
        let new_instructions = transaction.simulate_and_update_instructions(Some(20000), priority_level).await?;
        
        println!("simulated and adjusted instructions");
        let signers = ProgrammaticTransaction::get_all_signers(additional_signer_strings.clone())?;

        let start = Instant::now();
        blue!("sending transaction");
        let signature = Rpc::send_and_confirm_transaction(
            RpcMode::fast(),
            new_instructions,
            signers,
            Some(60000),
        ).await?;

        green!("sent transaction in {}ms", start.elapsed().as_millis());

        Ok(signature)
    }

    // pub async fn handle_open_position_instructions(open_position_instruction: OrcaOpenPositionInstruction) -> anyhow::Result<()> {
    //     let instructions: Vec<Instruction> = open_position_instruction
    //         .instructions
    //         .iter()
    //         .map(|sol_instr| Orca::convert_to_instruction(sol_instr))
    //         .collect::<Result<Vec<_>, _>>()?;
    
    //     let recent_blockhash = Rpc::get_latest_blockhash(RpcMode::fast()).await?;
    
    //     let payer = Pubkey::from_str(&open_position_instruction.payer)?;
    
    //     let message = solana_sdk::message::Message::new_with_blockhash(&instructions, Some(&payer), &recent_blockhash);
    
    //     let wallet_keypair = solana_sdk::signature::Keypair::from_base58_string(&open_position_instruction.wallet_key);
    
    //     let mut signers: Vec<&dyn solana_sdk::signer::Signer> = vec![&wallet_keypair];
    
    //     let additional_keypairs: Vec<solana_sdk::signature::Keypair> = open_position_instruction
    //         .additional_signers
    //         .iter()
    //         .map(|s| {
    //             let decoded = base64::engine::general_purpose::STANDARD.decode(s).map_err(|e| {
    //                 anyhow::anyhow!("Failed to decode signer: {:?}, error: {}", s, e)
    //             })?;
    //             solana_sdk::signature::Keypair::from_bytes(&decoded).map_err(|e| {
    //                 anyhow::anyhow!("Failed to create keypair from bytes: {:?}, error: {}", decoded, e)
    //             })
    //         })
    //         .collect::<Result<Vec<_>, _>>()?;
    
    //     signers.extend(additional_keypairs.iter().map(|k| k as &dyn solana_sdk::signer::Signer));
    
    //     let transaction = solana_sdk::transaction::Transaction::new(&signers, message, recent_blockhash);
    
    //     Rpc::send_and_confirm_transaction(transaction, RpcMode::fast()).await?;
    
    //     Ok(())
    // }
    
}