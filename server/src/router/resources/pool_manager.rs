use cnctd_server::{
    bad_request, internal_server_error, not_found, unauthorized, success_data, success_msg,
    router::{error::{ErrorCode, ErrorResponse}, response::SuccessResponse, HttpMethod},
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use solana::{pool_manager::{new_position::{NewPosition, NewProgrammaticPosition}, position_manager::managed_position::ManagedPosition, PoolManager}, services::position_settings::PositionSettings, wallet::Wallet};

use crate::router::rest::Resource;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DataIn {
    id: Option<String>, 
    address: Option<String>,
    wallet_key: Option<String>,
    name: Option<String>,
    range_factor: Option<f64>,
}

enum Operation {
    AllPositions,
    OpenPosition,
    OpenProgrammaticPosition,
    ClosePosition,
    SwapTokens,
    ConnectLocalWallet,
    DisconnectLocalWallet,
    ProgrammaticWalletPubkey,
    StoredLocalWalletPubkey,
    ToggleAutoRebalance,
    AllPositionSettings,
    PositionSettings,
    Unrecognized,
}

impl Operation {
    fn from_str(s: &str) -> Self {
        match s {
            "all-positions" => Operation::AllPositions,
            "open-position" => Operation::OpenPosition,
            "open-programmatic-position" => Operation::OpenProgrammaticPosition,
            "close-position" => Operation::ClosePosition,
            "swap-tokens" => Operation::SwapTokens,
            "connect-local-wallet" => Operation::ConnectLocalWallet,
            "disconnect-local-wallet" => Operation::DisconnectLocalWallet,
            "programmatic-wallet-pubkey" => Operation::ProgrammaticWalletPubkey,
            "stored-local-wallet-pubkey" => Operation::StoredLocalWalletPubkey,
            "toggle-auto-rebalance" => Operation::ToggleAutoRebalance,
            "all-position-settings" => Operation::AllPositionSettings,
            "position-settings" => Operation::PositionSettings,
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
        match self {
            Operation::ClosePosition 
            | Operation::OpenProgrammaticPosition 
            | Operation::SwapTokens 
            | Operation::OpenPosition
            | Operation::ToggleAutoRebalance => true,
            _ => false,
        }
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

    // if operation.requires_auth() {
    //     Resource::authenticate(auth_token.clone()).await.map_err(|e| unauthorized!(e))?;
    // }

    match method {
        HttpMethod::GET => match operation {
            Operation::AllPositions => {
                let positions = PoolManager::get_managed_positions().await.map_err(|e| internal_server_error!(e))?;
                println!("Positions: {:?}", positions);
                Ok(success_data!(json!(positions)))
            }
            Operation::ProgrammaticWalletPubkey => {
                let wallet_pubkey = Wallet::get_programmatic_pubkey().map_err(|e| internal_server_error!(e))?;

                // Wallet::get_sol_balance(&RpcUrl::solana_mainnet(), wallet_pubkey).await.map_err(|e| internal_server_error!(e))?;

                Ok(success_data!(json!(wallet_pubkey.to_string())))
            }
            Operation::StoredLocalWalletPubkey => {
                let wallet_pubkey = Wallet::get_stored_local_wallet_pubkey().map_err(|e| internal_server_error!(e))?;

                // Wallet::get_sol_balance(&RpcUrl::solana_mainnet(), wallet_pubkey).await.map_err(|e| internal_server_error!(e))?;

                Ok(success_data!(json!(wallet_pubkey.to_string())))
            }
            Operation::AllPositionSettings => {
                let position_settings = PositionSettings::get_all().await.map_err(|e| internal_server_error!(e))?;

                Ok(success_data!(json!(position_settings)))
            }
            Operation::PositionSettings => {
                let name = data.name.ok_or_else(|| bad_request!("Missing name"))?;

                let position_settings = PositionSettings::get(name).await.map_err(|e| internal_server_error!(e))?;

                Ok(success_data!(json!(position_settings)))
            }
            _ => Err(bad_request!("Invalid operation for GET")),
        },
        HttpMethod::POST => match operation {
            Operation::OpenPosition => {
                println!("incoming data: {:?}", data_val);
                let new_position: NewPosition = serde_json::from_value(data_val).map_err(|e| bad_request!(e))?;

                println!("Opening position with new_position: {:?}", new_position);
                let open_position_instructions = PoolManager::open_position(new_position).await.map_err(|e| internal_server_error!(e))?;


                Ok(success_data!(json!(open_position_instructions)))
            }
            Operation::OpenProgrammaticPosition => {
                let new_position: NewProgrammaticPosition = serde_json::from_value(data_val).map_err(|e| bad_request!(e))?;

                PoolManager::queue_programmatic_open(new_position).await.map_err(|e| internal_server_error!(e))?;

                Ok(success_msg!("Ok"))
            }
            Operation::SwapTokens => {
                println!("Swapping tokens with data: {:?}", data_val);
                let token_swap = serde_json::from_value(data_val).map_err(|e| bad_request!(e))?;
                let swap_instructions = PoolManager::swap_tokens(token_swap).await.map_err(|e| internal_server_error!(e))?;

                Ok(success_data!(json!(swap_instructions)))
            }
            Operation::PositionSettings => {
                let name = data.name.ok_or_else(|| bad_request!("Missing name"))?;
                let range_factor = data.range_factor.ok_or_else(|| bad_request!("Missing range factor"))?;

                let position_settings = PositionSettings::new(name, range_factor).await.map_err(|e| internal_server_error!(e))?;

                Ok(success_data!(json!(position_settings)))
            }
            _ => Err(bad_request!("Invalid operation for POST")),
        },
        HttpMethod::PUT => match operation {
            Operation::ClosePosition => {
                println!("Closing position with data: {:?}", data_val);
                let address = data.address.ok_or_else(|| bad_request!("Missing address"))?;    

                PoolManager::queue_programmatic_close(&address).await.map_err(|e| internal_server_error!(e))?;

                Ok(success_msg!("Queued for close"))
            }
            Operation::ConnectLocalWallet => {
                let wallet_key_string = data.wallet_key.ok_or_else(|| bad_request!("Missing wallet key"))?;
                println!("Connecting local wallet with key: {:?}", wallet_key_string);
                let wallet_positions = PoolManager::set_local_wallet_pubkey(wallet_key_string).await.map_err(|e| internal_server_error!(e))?;

                Ok(success_data!(json!(wallet_positions)))
            }
            Operation::DisconnectLocalWallet => {
                let removed_positions = PoolManager::unset_local_wallet_pubkey().await.map_err(|e| internal_server_error!(e))?;

                Ok(success_data!(json!(removed_positions)))
            }
            Operation::ToggleAutoRebalance => {
                let mut managed_position: ManagedPosition = serde_json::from_value(data_val).map_err(|e| bad_request!(e))?;
                managed_position.toggle_auto_rebalance().await.map_err(|e| internal_server_error!(e))?;

                Ok(success_msg!("Ok"))
            }
            Operation::PositionSettings => {
                let name = data.name.ok_or_else(|| bad_request!("Missing name"))?;
                let range_factor = data.range_factor.ok_or_else(|| bad_request!("Missing range factor"))?;

                let position_settings = PositionSettings::update(name, range_factor).await.map_err(|e| internal_server_error!(e))?;

                Ok(success_data!(json!(position_settings)))
            }
            _ => Err(bad_request!("Invalid operation for PUT")),
        },
        HttpMethod::DELETE => match operation {
            Operation::PositionSettings => {
                let name = data.name.ok_or_else(|| bad_request!("Missing name"))?;

                let _result = PositionSettings::delete(name).await.map_err(|e| internal_server_error!(e))?;

                Ok(success_msg!("Deleted"))
            }
            _ => Err(bad_request!("Invalid operation for DELETE")),
        },
        _ => Err(bad_request!("Invalid HTTP method")),
    }
}
