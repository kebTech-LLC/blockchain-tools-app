use cnctd_server::{
    bad_request, internal_server_error, not_found, unauthorized, success_data, success_msg,
    router::{error::{ErrorCode, ErrorResponse}, response::SuccessResponse, HttpMethod},
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use solana::pool_manager::POOL_MANAGER;
use crate::router::rest::Resource;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DataIn {
    id: Option<String>, // Changed to String for simplicity (no UUID dependency)
    name: Option<String>,
    order_number: Option<i32>,
    public_flag: Option<bool>,
    token: Option<String>,
}

enum Operation {
    Unrecognized,
    AllPositions,
}

impl Operation {
    fn from_str(s: &str) -> Self {
        match s {
            "all-positions" => Operation::AllPositions,
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
        !matches!(self, Operation::Unrecognized | Operation::AllPositions)
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
                let pool_manager = match POOL_MANAGER.get().lock() {
                    Ok(pool_manager) => pool_manager.clone(),
                    Err(e) => return Err(internal_server_error!(e)),
                };
                println!("Pool Manager: {:?}", pool_manager);
                let positions = pool_manager.managed_positions.clone();
                println!("Positions: {:?}", positions);
                Ok(success_data!(json!(positions)))
            }
            _ => Err(bad_request!("Invalid operation for GET")),
        },
        HttpMethod::POST => match operation {
            _ => Err(bad_request!("Invalid operation for POST")),
        },
        HttpMethod::PUT => match operation {
            _ => Err(bad_request!("Invalid operation for PUT")),
        },
        HttpMethod::DELETE => match operation {
            _ => Err(bad_request!("Invalid operation for DELETE")),
        },
        _ => Err(bad_request!("Invalid HTTP method")),
    }
}
