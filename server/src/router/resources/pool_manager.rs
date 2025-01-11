use cnctd_server::{
    bad_request, internal_server_error, not_found, unauthorized, success_data, success_msg,
    router::{error::{ErrorCode, ErrorResponse}, response::SuccessResponse, HttpMethod},
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use solana::pool_manager::PoolManager;
use crate::router::rest::Resource;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DataIn {
    id: Option<String>, 
    wallet_key: Option<String>,
}

enum Operation {
    AllPositions,
    OpenPosition,
    ClosePosition,
    ConnectBrowserWallet,
    DisconnectBrowserWallet,
    Unrecognized,
}

impl Operation {
    fn from_str(s: &str) -> Self {
        match s {
            "all-positions" => Operation::AllPositions,
            "open-position" => Operation::OpenPosition,
            "close-position" => Operation::ClosePosition,
            "connect-browser-wallet" => Operation::ConnectBrowserWallet,
            "disconnect-browser-wallet" => Operation::DisconnectBrowserWallet,
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
        !matches!(
            self, 
            Operation::Unrecognized 
            | Operation::AllPositions 
            | Operation::OpenPosition
            | Operation::ClosePosition
            | Operation::ConnectBrowserWallet
            | Operation::DisconnectBrowserWallet
        )
    }
}

pub async fn route_pool_manager(
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
            Operation::AllPositions => {
                let positions = PoolManager::get_managed_positions().map_err(|e| internal_server_error!(e))?;
                println!("Positions: {:?}", positions);
                Ok(success_data!(json!(positions)))
            }
            _ => Err(bad_request!("Invalid operation for GET")),
        },
        HttpMethod::POST => match operation {
            Operation::OpenPosition => {
                println!("Opening position with data: {:?}", data_val);


                Ok(success_data!(json!(data_val)))
            }
            _ => Err(bad_request!("Invalid operation for POST")),
        },
        HttpMethod::PUT => match operation {
            Operation::ClosePosition => {
                println!("Closing position with data: {:?}", data_val);

                Ok(success_msg!("Position closed"))
            }
            Operation::ConnectBrowserWallet => {
                let wallet_key_string = data.wallet_key.ok_or_else(|| bad_request!("Missing wallet key"))?;
                println!("Connecting browser wallet with key: {:?}", wallet_key_string);
                let wallet_positions = PoolManager::set_browser_wallet_pubkey(wallet_key_string).await.map_err(|e| internal_server_error!(e))?;

                Ok(success_data!(json!(wallet_positions)))
            }
            Operation::DisconnectBrowserWallet => {
                let removed_positions = PoolManager::unset_browser_wallet_pubkey().map_err(|e| internal_server_error!(e))?;

                Ok(success_data!(json!(removed_positions)))
            }
            _ => Err(bad_request!("Invalid operation for PUT")),
        },
        HttpMethod::DELETE => match operation {
            _ => Err(bad_request!("Invalid operation for DELETE")),
        },
        _ => Err(bad_request!("Invalid HTTP method")),
    }
}
