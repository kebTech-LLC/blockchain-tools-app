use cnctd_server::{bad_request, internal_server_error, not_found, router::{error::{ErrorCode, ErrorResponse}, response::SuccessResponse, HttpMethod}, socket::{client::CnctdClient, CnctdSocket}, success_data, success_msg, unauthorized};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::router::rest::Resource;



#[derive(Serialize, Deserialize, Debug)]
struct DataIn {
    client_id: Option<String>,
    user_id: Option<String>,
    subscriptions: Option<Vec<String>>,
}

enum Operation {
    Register,
    UpdateUserId,
    Authenticate,
    Unrecognized
}

impl Operation {
    fn from_str(s: &str) -> Self {
        match s {
            "register" => Operation::Register,
            "update-user-id" => Operation::UpdateUserId,
            "authenticate" => Operation::Authenticate,
            _ => Operation::Unrecognized
        }
    }

    fn from_option(s: Option<String>) -> Self {
        match s {
            Some(s) => Self::from_str(&s),
            None => Operation::Unrecognized
        }
    }

    fn requires_auth(&self) -> bool {
        match self {
            Operation::Unrecognized | Operation::UpdateUserId | Operation::Register => false,
            _ => true
        }
    }
    
}

pub async fn route_sessions(method: HttpMethod, operation: Option<String>, data: Value, auth_token: Option<String>, client_id: Option<String>, ip_address: Option<String>) -> Result<SuccessResponse, ErrorResponse> {
    let operation = Operation::from_option(operation);
    let data: DataIn = serde_json::from_value(data).map_err(|e| bad_request!(&e.to_string()))?;

    if operation.requires_auth() {
        match Resource::authenticate(auth_token.clone()).await {
            Ok(_user) => {},
            Err(e) => return Err(unauthorized!(e))
        }
    }

    match method {
        HttpMethod::GET =>{
            match operation {
                _ => return Err(not_found!("No matching operation"))
            }
        }
        HttpMethod::POST => {
            match operation {
                Operation::Register => {
                    let subscriptions: Vec<String> = data.subscriptions.unwrap_or_else(|| vec![]);
                    let client_id = CnctdClient::register_client(subscriptions, ip_address).await.map_err(|e| not_found!(e))?;
                
                    Ok(success_data!(client_id.into()))
                }
                
                _ => return Err(not_found!("No matching operation")),
            }
        }
        HttpMethod::PUT => {
            match operation {
                Operation::UpdateUserId => {
                    let client_id = data.client_id.ok_or_else(|| bad_request!("client_id required"))?;
                    let user_id = data.user_id.ok_or_else(|| bad_request!("user_id required"))?;
                    CnctdClient::update_client_user_id(&client_id, &user_id).await.map_err(|e| not_found!(e))?;
                    let client_info = CnctdClient::get_client_info(&client_id).await.map_err(|e| not_found!(e))?;

                    Ok(success_data!(json!(client_info)))
                }
                Operation::Authenticate => {
                    let client_id = data.client_id.ok_or_else(|| bad_request!("client_id required"))?;
                    CnctdClient::update_client_authenticated(&client_id, true).await.map_err(|e| not_found!(e))?;
                    let client_info = CnctdClient::get_client_info(&client_id).await.map_err(|e| not_found!(e))?;

                    Ok(success_data!(json!(client_info)))
                }
                _ => return Err(not_found!("No matching operation"))
            }
        }
        HttpMethod::DELETE => {
            match operation {
                _ => return Err(not_found!("No matching operation"))
            }
        }
    }
}