use chrono::{TimeZone, Utc};
use cnctd_server::{
    bad_request, internal_server_error, not_found, unauthorized, success_data, success_msg,
    router::{error::{ErrorCode, ErrorResponse}, response::SuccessResponse, HttpMethod},
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::{blockchains::solana::{account_snapshot::{self, AccountSnapshot}, Solana}, external_apis::helius::Helius, router::rest::Resource};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DataIn {
    id: Option<String>, // Changed to String for simplicity (no UUID dependency)
    name: Option<String>,
    order_number: Option<i32>,
    public_flag: Option<bool>,
    token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PublicKeyRequest {
    public_key: String,
    date: Option<String>,
}

enum Operation {
    ById,
    Create,
    UpdateName,
    Unrecognized,
}

impl Operation {
    fn from_str(s: &str) -> Self {
        match s {
            "by-id" => Operation::ById,
            "create" => Operation::Create,
            "update-name" => Operation::UpdateName,

            _ => Operation::Unrecognized,
        }
    }

    fn from_option(s: Option<String>) -> Self {
        match s {
            Some(op) => Self::from_str(&op),
            None => Operation::Unrecognized,
        }
    }

    fn requires_auth(&self) -> bool {
        !matches!(self, Operation::Unrecognized,)
    }
}

pub async fn route_public_key(
    method: HttpMethod,
    operation: Option<String>,
    data_val: Value,
    auth_token: Option<String>,
    _client_id: Option<String>, // Client ID is unused in this simplified version
) -> Result<SuccessResponse, ErrorResponse> {
    let operation = Operation::from_option(operation);
    let data: DataIn = serde_json::from_value(data_val.clone()).map_err(|e| bad_request!(e))?;

    if operation.requires_auth() {
        Resource::authenticate(auth_token.clone()).map_err(|e| unauthorized!(e))?;
    }

    match method {
        HttpMethod::GET => match operation {
            Operation::ById => {
                let id = data.id.ok_or_else(|| bad_request!("id required"))?;
                let resource = json!({ "id": id, "name": "Example Resource" }); // Simulated resource
                Ok(success_data!(resource))
            }
            _ => Err(bad_request!("Invalid operation for GET")),
        },
        HttpMethod::POST => {
            // Parse the input data to extract the public key
            let request: PublicKeyRequest = serde_json::from_value(data_val)
                .map_err(|e| bad_request!(format!("Invalid request: {}", e)))?;

            let account_balance = Solana::get_balance(&request.public_key).await
                .map_err(|e| internal_server_error!(format!("Failed to fetch balance: {}", e)))?;

            let stake_accounts = Solana::get_stake_accounts(&request.public_key).await
                .map_err(|e| internal_server_error!(format!("Failed to fetch stake accounts: {}", e)))?;
            
            // let date = request.date.unwrap();
            // let target_date = chrono::DateTime::parse_from_str(&date, "%Y-%m-%d %H:%M:%S")
            //     .map_err(|e| bad_request!(format!("Invalid date format: {}", e)))?
            //     .with_timezone(&Utc);

            // let mut account_snapshots: Vec<AccountSnapshot> = Vec::new();
            // for (pub_key, stake_account) in stake_accounts {
            //     let pub_key_str = &pub_key.to_string();
            //     let snapshot = AccountSnapshot::get_account_snapshot_at_date(pub_key_str, target_date)
            //         .await
            //         .map_err(|e| internal_server_error!(format!("Failed to fetch account snapshot: {}", e)))?;
            //     account_snapshots.push(snapshot);
            // }
            let helius = Helius::new()
                .map_err(|e| internal_server_error!(format!("Failed to initialize Helius client: {}", e)))?;

            // let transaction_history = helius.get_transaction_history(&request.public_key).await
            //     .map_err(|e| internal_server_error!(format!("Failed to fetch transaction history: {}", e)))?;

            let stake_accounts = helius.get_stake_accounts_from_history(&request.public_key).await
                .map_err(|e| internal_server_error!(format!("Failed to fetch stake accounts: {}", e)))?;

            // Return the public key in a response object
            let response = json!({
                "message": "Public key received successfully",
                "account_balance": account_balance,
                // "transaction_history": transaction_history,
                // "account_snapshots": account_snapshots,
                "stake_accounts": stake_accounts,
            });

            Ok(success_data!(response))
        }
        HttpMethod::PUT => match operation {
            Operation::UpdateName => {
                let id = data.id.ok_or_else(|| bad_request!("id required"))?;
                let new_name = data.name.ok_or_else(|| bad_request!("name required"))?;
                let updated_resource = json!({ "id": id, "name": new_name }); // Simulated update
                Ok(success_data!(updated_resource))
            }
            _ => Err(bad_request!("Invalid operation for PUT")),
        },
        HttpMethod::DELETE => match operation {
            Operation::ById => {
                let id = data.id.ok_or_else(|| bad_request!("id required"))?;
                Ok(success_msg!(format!("Resource with id {} deleted", id)))
            }
            _ => Err(bad_request!("Invalid operation for DELETE")),
        },
        _ => Err(bad_request!("Invalid HTTP method")),
    }
}

