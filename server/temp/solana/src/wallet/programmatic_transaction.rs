use std::{str::FromStr, sync::Arc};

use helius::{types::{GetPriorityFeeEstimateOptions, GetPriorityFeeEstimateRequest, PriorityLevel, UiTransactionEncoding}, Helius};
use kebtech_utils::*;
use base64::{prelude::BASE64_STANDARD, Engine};
use serde::{Deserialize, Serialize};
use solana_client::{client_error::reqwest, nonblocking::rpc_client::RpcClient};
use solana_sdk::{compute_budget::ComputeBudgetInstruction, instruction::Instruction, message::Message, pubkey::Pubkey, signature::Keypair, signer::Signer, transaction::Transaction};

use crate::{rpc::{rpc_url::RpcUrl, Rpc, RpcMode}, utils::serialize_transaction_to_base58};

use super::Wallet;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProgrammaticTransaction {
    pub transaction: Transaction,
}

impl ProgrammaticTransaction {
    pub async fn new(instructions: Vec<Instruction>, signers: Vec<Box<dyn Signer>>) -> anyhow::Result<Self> {
        let wallet = Wallet::get_programmatic_keypair()?;
        let mut all_instructions = vec![];

        all_instructions.extend(instructions);
    
        let recent_blockhash = Rpc::get_latest_blockhash(RpcMode::fast(), None).await?;

        let message = Message::new_with_blockhash(&all_instructions, Some(&wallet.pubkey()), &recent_blockhash);
    
        let transaction = Transaction::new(&signers, message, recent_blockhash);
        
        Ok(ProgrammaticTransaction {
            transaction,
        })
    }

    pub async fn simulate_and_update_instructions(
        &self,
        timeout_ms: Option<u64>,
        // pool_address: &str,
        priority_level: Option<PriorityLevel>,
    ) -> anyhow::Result<Vec<Instruction>> {
        // Simulate the current transaction
        let start = std::time::Instant::now();
        blue!("simulating transaction");
        let (_context, simulated_transaction) = Rpc::simulate_transaction(
            RpcMode::fast(),
            &self.transaction,
            timeout_ms,
        )
        .await?;
        green!("simulated transaction in {}ms", start.elapsed().as_millis());
    
        // Resolve the pool address to a Pubkey
        // let address_pubkey = Pubkey::from_str(pool_address)?;
        let mut additional_instructions = vec![];
    
        // Adjust limits if units were consumed
        if let Some(units_consumed) = simulated_transaction.units_consumed {
            let units_consumed_safe = units_consumed as u32 + 100_000;
            let compute_limit_instruction = ComputeBudgetInstruction::set_compute_unit_limit(units_consumed_safe);
            additional_instructions.push(compute_limit_instruction);

            // Helius priority fee logic
            
            let api_key = std::env::var("HELIUS_API_KEY")?;
            let config = helius::config::Config::new(&api_key, helius::types::Cluster::MainnetBeta)?;
            let request_client = reqwest::Client::new();
            let client = helius::rpc_client::RpcClient::new(Arc::new(request_client), Arc::new(config))?;   

            let recommended = if priority_level.is_none() {
                Some(true)
            } else {
                None
            };
            let request = GetPriorityFeeEstimateRequest {
                transaction: Some(serialize_transaction_to_base58(&self.transaction)?), // Provide the serialized transaction
                account_keys: None,                  // Use account keys if you prefer
                options: Some(GetPriorityFeeEstimateOptions {
                    priority_level: priority_level, // Adjust priority level as needed
                    include_all_priority_fee_levels: None,
                    transaction_encoding: None,
                    lookback_slots: None,
                    recommended,
                    include_vote: None,
                }),
            };
            let priority_fee_estimate_response = client.get_priority_fee_estimate(request).await.map_err(|e| {
                anyhow::anyhow!("Helius API call failed: {:?}", e)
            })?;
            
            if let Some(fee_estimate) = priority_fee_estimate_response.priority_fee_estimate {
                println!("Priority fee estimate: {}", fee_estimate);
                let priority_fee_instruction = ComputeBudgetInstruction::set_compute_unit_price((fee_estimate * 1.5) as u64);
                additional_instructions.push(priority_fee_instruction);
            } else {
                println!("No priority fee estimate available from Helius API.");
            }


            // Fetch recent prioritization fees
            // let start = std::time::Instant::now();
            // blue!("fetching prioritization fees");
            // let prioritization_fees = Rpc::call(
            //     move |client| {
            //         Box::pin(async move {
            //             client
            //                 .get_recent_prioritization_fees(&[address_pubkey])
            //                 .await
            //                 .map_err(|e| e.into())
            //         })
            //     },
            //     timeout_ms,
            //     RpcMode::fast(),
            // )
            // .await?;
            // green!("fetched prioritization fees in {}ms", start.elapsed().as_millis());

            // // Extract and sort prioritization fees
            // let mut prioritization_fees_array: Vec<u64> = prioritization_fees
            //     .iter()
            //     .map(|fee| fee.prioritization_fee)
            //     .collect();
            // prioritization_fees_array.sort_unstable();

            // // Calculate the median and add 25%
            // if let Some(prioritization_fee) = prioritization_fees_array.get(prioritization_fees_array.len() / 2) {
            //     let adjusted_fee = ((*prioritization_fee as f64) * 1.75).ceil() as u64; // Add 75% and round up
            //     let minimum_fee = 1_000_000; // Minimum fee in lamports (0.001 SOL)
            //     let final_fee = adjusted_fee.max(minimum_fee); // Ensure the fee is at least 0.001 SOL
            
            //     let priority_fee_instruction = ComputeBudgetInstruction::set_compute_unit_price(final_fee);
            //     additional_instructions.push(priority_fee_instruction);
            // }
            
        }
    
        // Reconstruct the original instructions
        let mut updated_instructions: Vec<Instruction> = self
            .transaction
            .message
            .instructions
            .iter()
            .map(|compiled| {
                let program_id = self.transaction.message.account_keys[compiled.program_id_index as usize];
                let accounts = compiled
                    .accounts
                    .iter()
                    .map(|&index| {
                        let pubkey = self.transaction.message.account_keys[index as usize];
                        let is_signer = self.transaction.message.is_signer(index as usize);
                        let is_writable = self.transaction.message.is_maybe_writable(index as usize, None);
                        solana_sdk::instruction::AccountMeta {
                            pubkey,
                            is_signer,
                            is_writable,
                        }
                    })
                    .collect();
                solana_sdk::instruction::Instruction {
                    program_id,
                    accounts,
                    data: compiled.data.clone(),
                }
            })
            .collect();
    
        // Append the additional instructions
        updated_instructions.extend(additional_instructions);
    
        // Return the combined instructions
        Ok(updated_instructions)
    }
    
    pub fn get_all_signers(additional_signer_strings: Vec<String>) -> anyhow::Result<Vec<Box<dyn Signer>>> {
        // Get the programmatic wallet keypair
        let wallet_keypair = Wallet::get_programmatic_keypair()?;
        let mut signers: Vec<Box<dyn Signer>> = vec![Box::new(wallet_keypair)];
    
        // Decode and create additional keypairs
        let additional_keypairs = additional_signer_strings
            .iter()
            .map(|s| {
                let decoded = BASE64_STANDARD.decode(s).map_err(|e| {
                    anyhow::anyhow!("Failed to decode signer: {:?}, error: {}", s, e)
                })?;
                Keypair::from_bytes(&decoded).map_err(|e| {
                    anyhow::anyhow!("Failed to create Keypair from bytes: {:?}", e)
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
    
        // Convert additional keypairs into Box<dyn Signer>
        signers.extend(
            additional_keypairs.into_iter().map(|kp| Box::new(kp) as Box<dyn Signer>)
        );
    
        Ok(signers)
    }
}