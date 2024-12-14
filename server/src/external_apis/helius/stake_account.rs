use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StakeAccount {
    pub pubkey: String, // The public key of the stake account
    pub lamports: u64,  // Amount of lamports in the stake account
    // Add more fields if necessary, based on the API response
}

#[derive(Debug, Deserialize)]
pub struct StakeAccountsResponse {
    pub accounts: Vec<StakeAccount>, // List of stake accounts
}
