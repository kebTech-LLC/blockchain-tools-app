use solana::{pool_manager::PoolManager, wallet::Wallet};

pub struct SolanaPools;

impl SolanaPools {
    pub async fn get_sol_usdc_pool() -> anyhow::Result<()> {
        Ok(())
    }

    // pub async fn get_sol_balance() -> anyhow::Result<f64> {
    //     let balance = Wallet::get_sol_balance().await?;

    //     println!("SOL balance: {}", balance);

    //     Ok(balance)
    // }
    
}