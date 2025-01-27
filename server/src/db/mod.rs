pub mod entities;
pub mod services;

use std::{env, sync::Arc};

use sea_orm::{Database, DatabaseConnection};
use state::InitCell;

static CONNECTION: InitCell<Arc<DatabaseConnection>> = InitCell::new();

pub struct DB;

impl DB {
    pub async fn start() -> anyhow::Result<()> {
        let url = env::var("DATABASE_URL")?;
        let connection = Database::connect(&url).await?;

        CONNECTION.set(Arc::new(connection));

        Ok(())
    }
    
    pub async fn get_connection() -> anyhow::Result<Arc<DatabaseConnection>> {
        Ok(CONNECTION.try_get().ok_or_else(|| anyhow::anyhow!("No connection"))?.clone())
    }
}