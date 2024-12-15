use std::{str::FromStr, time::Duration};
use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_config::RpcBlockConfig;
use solana_sdk::{account::Account, commitment_config::CommitmentConfig, pubkey::Pubkey};

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountSnapshot {
    pub account: Account,
    pub slot: u64,           // The slot at which this snapshot was taken
    pub date: DateTime<Utc>, // The block time for this snapshot
}

impl AccountSnapshot {
    pub async fn get_account_snapshot_at_date(
        stake_account_pubkey: &str,
        target_date: DateTime<Utc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let rpc_url = "https://api.mainnet-beta.solana.com";
        let client = RpcClient::new_with_timeout(rpc_url.into(), Duration::from_secs(360));

        // Convert the stake account public key to a Solana Pubkey
        let pubkey = Pubkey::from_str(stake_account_pubkey)?;

        // Get the current block height
        let current_block_height = client.get_block_height().await?;

        // Perform a binary search to find the closest block to the target date
        let closest_block_slot = find_closest_block(&client, target_date, 0, current_block_height).await?;

        // Fetch block details for the slot
        let block = client
            .get_block_with_config(closest_block_slot, RpcBlockConfig::default())
            .await?;

        let block_time = block.block_time.ok_or("Block time not found")?;
        let block_date = Utc.timestamp(block_time, 0);

        // Fetch the account details with the correct commitment config
        let account_info = client
            .get_account_with_commitment(
                &pubkey,
                CommitmentConfig::confirmed(),
            )
            .await?
            .value
            .ok_or("Stake account not found at the given slot")?;

        // Build and return the snapshot
        Ok(AccountSnapshot {
            account: account_info,
            slot: closest_block_slot,
            date: block_date,
        })
    }
}

/// Binary search to find the closest block to a target date
async fn find_closest_block(
    client: &RpcClient,
    target_date: DateTime<Utc>,
    mut low: u64,
    mut high: u64,
) -> Result<u64, Box<dyn std::error::Error>> {
    while low <= high {
        let mid = (low + high) / 2;

        // Get the block time for the mid slot
        let block_time = client.get_block_time(mid).await?;


        let block_date = Utc.timestamp(block_time, 0);

        if block_date == target_date {
            return Ok(mid); // Exact match
        } else if block_date < target_date {
            low = mid + 1; // Search in the upper half
        } else {
            high = mid - 1; // Search in the lower half
        }
    }

    // If no exact match, return the closest slot (high will be the best candidate)
    Ok(high)
}
