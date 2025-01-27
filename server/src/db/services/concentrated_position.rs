use sea_orm::Set;
use solana::pool_manager::position_manager::managed_position::ManagedPosition;

use crate::db::{entities::concentrated_position::{ActiveModel, Model}, DB};


pub struct ConcentratedPosition;

impl ConcentratedPosition {
    // pub async fn from_managed_position(managed_position: &ManagedPosition) -> anyhow::Result<Model> {
    //     let db= DB::get_connection().await?;

    //     let active_model = ActiveModel {
    //         address: Set(managed_position.address),
    //         wallet_address: Set(managed_position.wallet_key),
    //         pool_address: Set(managed_position.pool_address),
    //         pool_type: Set(managed_position.pool_type.to_string()),
    //         token_mint_a: Set(managed_position.token_a.unwrap().address),
    //         token_mint_b: Set(managed_position.token_b.unwrap().address),
    //         token_a_decimals: todo!(),
    //         token_b_decimals: todo!(),
    //         start_price_token_a_per_b: todo!(),
    //         start_price_token_b_per_a: todo!(),
    //         start_price_sqrt: todo!(),
    //         end_price_token_a_per_b: todo!(),
    //         end_price_token_b_per_a: todo!(),
    //         end_price_sqrt: todo!(),
    //         start_balance_token_a: todo!(),
    //         start_balance_token_b: todo!(),
    //         end_balance_token_a: todo!(),
    //         end_balance_token_b: todo!(),
    //         liquidity: todo!(),
    //         tick_lower_index: todo!(),
    //         tick_upper_index: todo!(),
    //         range_lower: todo!(),
    //         range_upper: todo!(),
    //         fees_earned_token_a: todo!(),
    //         fees_earned_token_b: todo!(),
    //         fees_paid_open_sol: todo!(),
    //         fees_paid_open_usd: todo!(),
    //         fees_paid_close_sol: todo!(),
    //         fees_paid_close_usd: todo!(),
    //         sol_costs_start: todo!(),
    //         sol_costs_end: todo!(),
    //         rewards_amount: todo!(),
    //         rewards_mint: todo!(),
    //         rent_exempt_paid_sol: todo!(),
    //         rent_exempt_refunded_sol: todo!(),
    //         opened_at: todo!(),
    //         closed_at: todo!(),
    //         position_mint: todo!(),
           
    //     };


    //     let model = active_model
    //         .insert(&*db)
    //         .await?;

    //     Ok(model)

    // }
}