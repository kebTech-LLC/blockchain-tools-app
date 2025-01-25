use crate::router::resources::{
    new_resource::route_new_resource, pool_manager::route_pool_manager, public_key::route_public_key, sessions::route_sessions
  
};
use std::{future::Future, pin::Pin};

// use crate::{auth::Authentication, services::resource_service::ResourceService};
use cnctd_server::{auth::CnctdAuth, router::{error::{ErrorCode, ErrorResponse}, response::SuccessResponse, HttpMethod, RestRouterFunction}, server::CnctdServer};
use google_oauth::AsyncClient;
use serde_json::Value;
use state::InitCell;
use anyhow::anyhow;
use uuid::Uuid;

pub static JWT_SECRET: InitCell<Vec<u8>> = InitCell::new();
pub static GOOGLE_CLIENT_ID: InitCell<String> = InitCell::new();

#[derive(Debug)]
pub enum Resource {
    NewResource,
    PublicKey,
    Sessions,
    PoolManager,
    Unrecognized,
}

impl Resource {
    pub fn from_str(s: &str) -> Self {
        match s {
            "new_resource" => Resource::NewResource,
            "public_key" => Resource::PublicKey,
            "sessions" => Resource::Sessions,
            "pool_manager" => Resource::PoolManager,
            _ => Resource::Unrecognized,
        }
    }

    pub async fn authenticate(auth_token: Option<String>) -> anyhow::Result<String> {
        // let secret = JWT_SECRET.try_get().ok_or(anyhow!("JWT secret not set"))?;
        let client_id = GOOGLE_CLIENT_ID.try_get().ok_or(anyhow!("Google client ID not set"))?;
        match auth_token {
            Some(id_token) => {
                let client = AsyncClient::new(client_id);
                // let user_id = CnctdAuth::verify_auth_token(secret.to_owned(), &jwt)?;
                let payload = client.validate_id_token(id_token).await?;
                let user_id = payload.sub;
                println!("User ID: {}", user_id);
                // Ok(Uuid::parse_str(&user_id)?)
                Ok(user_id)
            }
            None => return Err(anyhow!("auth token required"))
        }
    }
}


#[derive(Clone, Copy)]
pub struct RestRouter;

impl RestRouterFunction for RestRouter {
    fn route(&self, method: HttpMethod, path: String, data: Value, auth_token: Option<String>, client_id: Option<String>, ip_address: Option<String>) -> Pin<Box<dyn Future<Output = Result<SuccessResponse, ErrorResponse>> + Send + 'static>> {
        Box::pin(async move {
            route(method, path, data, auth_token, client_id, ip_address).await
        })
    }

    fn route_redirect(&self, path: String, data: Value, auth_token: Option<String>, client_id: Option<String>) -> Pin<Box<dyn Future<Output = String> + Send>> {
        Box::pin(async move {
            let response = ErrorResponse::new(
                Some(ErrorCode::NotFound),
                Some("Unrecognized resource".into()),
            );
            response.to_string()
        })
    }
}

async fn route(method: HttpMethod, path: String, data: Value, auth_token: Option<String>, client_id: Option<String>, ip_address: Option<String>) -> Result<SuccessResponse, ErrorResponse> {
    let (resource, operation) = CnctdServer::path_to_resource_and_operation(&path);
    println!(
        "Routing request...method: {:?}, path: {}, resource: {}, operation: {:?}, data: {:?}",
        method, path, resource, operation, !data.is_null()
    );

    let resource = Resource::from_str(&resource);

    match resource {
        Resource::NewResource => {
            Ok(route_new_resource(method, operation, data, auth_token, client_id).await?)
        }
        Resource::PublicKey => {
            Ok(route_public_key(method, operation, data, auth_token, None).await?)
        }
        Resource::Sessions => {
            Ok(route_sessions(method, operation, data, auth_token, client_id, ip_address).await?)
        }
        Resource::PoolManager => {
            Ok(route_pool_manager(method, operation, data, auth_token, client_id).await?)
        }
        _ => {
            let response = ErrorResponse::new(
                Some(ErrorCode::NotFound),
                Some("Unrecognized resource".into()),
            );
            Err(response)
        }
    }
}

fn parse_path(path: &str) -> (String, Option<String>) {
    let parts: Vec<&str> = path.trim_start_matches('/').split('/').collect();
    let resource = parts.get(0).unwrap_or(&"").to_string();
    let operation = parts.get(1).map(|s| s.to_string());
    (resource, operation)
}

