use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountInfo {
    pub pubkey: String,    // Public key of the account
    pub lamports: u64,     // Balance in lamports
    pub owner: String,     // Program that owns the account (e.g., Stake Program)
    pub executable: bool,  // Whether the account is executable (for programs)
    pub rent_epoch: u64,   // Rent epoch for the account
}

#[derive(Debug, Deserialize)]
pub struct AccountInfoResponse {
    pub accounts: Vec<AccountInfo>, // List of accounts owned by the wallet
}
