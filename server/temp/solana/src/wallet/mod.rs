pub mod programmatic_transaction;

use std::str::FromStr;

use serde::{Deserialize, Serialize};
use solana_client::rpc_request::TokenAccountsFilter;
use solana_sdk::{native_token::lamports_to_sol, pubkey::Pubkey, signer::{keypair::Keypair, Signer}};

use crate::{rpc::{Rpc, RpcMode}, token::Token};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Wallet {
    pub pubkey: String,
    pub name: String,
}

impl Wallet {
    pub fn get_programmatic_keypair() -> anyhow::Result<Keypair> {
        dotenv::dotenv().ok();
        let private_key = std::env::var("SOLANA_WALLET_PRIVATE_KEY")?;
        let wallet = Keypair::from_base58_string(&private_key);

        Ok(wallet)
    }

    pub fn get_programmatic_pubkey() -> anyhow::Result<Pubkey> {
        let keypair = Wallet::get_programmatic_keypair()?;

        Ok(keypair.pubkey())
    }

    pub fn is_programmatic_wallet(wallet_key: &str) -> anyhow::Result<bool> {
        let programmatic_pubkey = Wallet::get_programmatic_pubkey()?;
        let wallet_pubkey = Pubkey::from_str(wallet_key)?;

        Ok(wallet_pubkey == programmatic_pubkey)
    }

    pub async fn get_token_balance(wallet_key: &str, token_mint: &str, rpc_mode: RpcMode) -> anyhow::Result<(u64, f64)> {
        let is_sol = token_mint == Token::solana().address;
        if is_sol {
            let lamports = Self::get_account_lamports(wallet_key, rpc_mode).await?;
            let sol = lamports_to_sol(lamports);
            return Ok((lamports, sol));
        } else {
            let token_accounts = Rpc::call(
                {
                    let wallet_key = wallet_key.to_string();
                    let token_mint = token_mint.to_string();
                    move |client| {
                        let wallet_key = wallet_key.clone();
                        let token_mint = token_mint.clone();
                        Box::pin(async move {
                            client.get_token_accounts_by_owner(
                                &Pubkey::from_str(&wallet_key).unwrap(),
                                TokenAccountsFilter::Mint(Pubkey::from_str(&token_mint).unwrap()),
                            ).await.map_err(|e| e.into())
                        })
                    }
                },
                None,
                rpc_mode.clone(),
            ).await?;

            let token_account = token_accounts.first().ok_or(anyhow::anyhow!("No token accounts found"))?;
            let pubkey_string = token_account.pubkey.clone();
            let pubkey = Pubkey::from_str(&pubkey_string).map_err(|e| anyhow::anyhow!(e.to_string()))?;
            // Fetch account balance
            let balance_response = Rpc::call(
                move |client| {
                    
                    Box::pin(async move {
                        client
                            .get_token_account_balance(&pubkey)
                            .await
                            .map_err(|e| e.into())
                    })
                },
                Some(5000),
                rpc_mode,
            )
            .await?;
    
            let ui_amount = balance_response.ui_amount.ok_or(anyhow::anyhow!("UI amount missing"))?;
            let amount = balance_response.amount.parse::<u64>()?;
    
            return Ok((amount, ui_amount));
            
        }


    }

    pub async fn get_account_lamports(account_str: &str, rpc_mode: RpcMode) -> anyhow::Result<u64> {
        let account_pubkey = Pubkey::from_str(account_str).map_err(|e| anyhow::anyhow!(e.to_string()))?;
        let account_data = Rpc::call(
            move |client| {
                Box::pin(async move {
                    client.get_account(&account_pubkey).await.map_err(|e| e.into())
                })
            },
            None,
            rpc_mode,
        ).await?;

        let balance = account_data.lamports;

        Ok(balance)

    }

    pub async fn get_sol_balance(wallet_key: &str, rpc_mode: RpcMode) -> anyhow::Result<f64> {
        let lamports = Self::get_account_lamports(wallet_key, rpc_mode).await?;
        let sol = lamports_to_sol(lamports);

        Ok(sol)
    }

    pub fn get_stored_local_wallet_pubkey() -> anyhow::Result<Pubkey> {
        dotenv::dotenv().ok();
        let wallet_key = std::env::var("SOLANA_DEFI_WALLET_PUBLIC_KEY")?;
        let wallet_pubkey = Pubkey::from_str(&wallet_key)?;
        
        Ok(wallet_pubkey)
    }

    // pub async fn get_programmatic_transaction(
    //     instructions: Vec<Instruction>,
    //     signers: Vec<Box<dyn Signer>>,
    //     simulation: bool,
    // ) -> anyhow::Result<Transaction> {
    //     let wallet = Wallet::get_programmatic_keypair()?;
    //     let recent_blockhash = Rpc::get_latest_blockhash(RpcMode::fast(), None).await?;
    
    //     // Initialize a vector for instructions, starting with prioritization instructions if provided
    //     let mut all_instructions = vec![];
    
    //     // Add prioritization fee instructions if specified
    //     if let Some(limit) = compute_unit_limit {
    //         let limit_instruction = ComputeBudgetInstruction::set_compute_unit_limit(limit);
    //         all_instructions.push(limit_instruction);
    //     }
    
    //     if let Some(price) = compute_unit_price {
    //         let price_instruction = ComputeBudgetInstruction::set_compute_unit_price(price);
    //         all_instructions.push(price_instruction);
    //     }
    
    //     // Append the provided instructions
    //     all_instructions.extend(instructions);
    
    //     // Create the transaction message
    //     let message = Message::new_with_blockhash(&all_instructions, Some(&wallet.pubkey()), &recent_blockhash);
    
    //     // Create the transaction with the signers
    //     let transaction = Transaction::new(&signers, message, recent_blockhash);
    
    //     Ok(transaction)
    // }
    

}