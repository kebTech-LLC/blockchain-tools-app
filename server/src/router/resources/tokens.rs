use cnctd_server::{
    bad_request, internal_server_error, not_found, unauthorized, success_data, success_msg,
    router::{error::{ErrorCode, ErrorResponse}, response::SuccessResponse, HttpMethod},
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::router::rest::Resource;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DataIn {
    id: Option<String>, // Changed to String for simplicity (no UUID dependency)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct IncomingTokenSwap {
    pub wallet_address: String,
    pub pool_address: Option<String>,
    pub amount: u64,
    pub amount_is_in: bool,
    pub mint_out_address: String,
    pub slippage_tolerance: Option<u16>,
}

enum Operation {
    Swap,
    Unrecognized,
}

impl Operation {
    fn from_str(s: &str) -> Self {
        match s {
            "swap" => Operation::Swap,
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
        !matches!(self, Operation::Unrecognized)
    }
}

pub async fn route_token(
    method: HttpMethod,
    operation: Option<String>,
    data_val: Value,
    auth_token: Option<String>,
    _client_id: Option<String>, // Client ID is unused in this simplified version
) -> Result<SuccessResponse, ErrorResponse> {
    let operation = Operation::from_option(operation);
    let data: DataIn = serde_json::from_value(data_val.clone()).map_err(|e| bad_request!(e))?;

    // if operation.requires_auth() {
    //     Resource::authenticate(auth_token.clone()).map_err(|e| unauthorized!(e))?;
    // }

    match method {
        HttpMethod::GET => match operation {
            _ => Err(bad_request!("Invalid operation for GET")),
        },
        HttpMethod::POST => match operation {
            // Operation::Swap => {
            //     let incoming_token_swap: IncomingTokenSwap = serde_json::from_value(data_val.clone()).map_err(|e| bad_request!(e))?;

            //     let swap_instructions = 
            // }
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
