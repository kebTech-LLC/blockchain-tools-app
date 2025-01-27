use serde_json::json;
use tokio::time::{interval, Duration};
use anyhow::Result;

use crate::pool_manager::message::{MessageType, PoolManagerMessage};

use super::coinbase::ticker::{TickerState, TimePeriod};

pub struct PriceChecker;

impl PriceChecker {
    pub fn new() -> Self {
        PriceChecker
    }

    pub fn init() {
        let price_checker = PriceChecker::new();

        tokio::spawn(async move {
            match price_checker.start().await {
                Ok(_) => (),
                Err(e) => eprintln!("Price checker error: {:?}", e),
            }
        });

        // Use `LocalSet` to ensure `Signer` does not require `Send`
        // tokio::spawn(async move {
        //     local.run_until(async move {
        //         match price_checker.start().await {
        //             Ok(_) => (),
        //             Err(e) => eprintln!("Price checker error: {:?}", e),
        //         }
        //     }).await;
        // });
    }

    pub async fn start(&self) -> Result<()> {
        let mut interval = interval(Duration::from_secs(1));

        loop {
            interval.tick().await;

            // Retrieve history and calculate volumes for each time period
            let stats = self.calculate_stats().await?;

            // // Print the results
            let mut data: Vec<String> = Vec::new();
            for (time_period, (_total_volume, per_second_volume, average_price)) in stats {
                data.push(format!(
                    "{:?}: {:.2} updates/sec, Average Price: {:.2}",
                    time_period, per_second_volume, average_price
                ));
            }
            let message = PoolManagerMessage {
                message_type: MessageType::Stats,
                data: Some(json!(data)),
                frequency_seconds: 1,
            };
            match message.add_to_queue().await {
                Ok(_) => {},
                Err(e) => println!("Failed to add stats to queue: {:?}", e),
            }

            

            // Process positions sequentially on the same thread
            // for mut position in managed_positions {
            //     if Wallet::is_programmatic_wallet(&position.wallet_key.clone())? {
            //         println!("Programmatic wallet detected: {}", position.wallet_key);

            //         if position.should_rebalance()? {
            //             println!("Closing position for wallet: {}", position.wallet_key);
            //             position.close().await?;

            //             println!("Rebalancing position for wallet: {}", position.wallet_key);
            //             position.rebalance().await?;
            //         }
            //     }
            // }
        }
    }

    async fn calculate_stats(&self) -> Result<Vec<(TimePeriod, (usize, f64, f64))>> {
        let time_periods = vec![
            TimePeriod::OneSecond,
            TimePeriod::FiveSeconds,
            TimePeriod::TenSeconds,
            TimePeriod::FifteenSeconds,
            TimePeriod::ThirtySeconds,
            TimePeriod::OneMinute,
            TimePeriod::FiveMinutes,
            TimePeriod::FifteenMinutes,
            TimePeriod::OneHour,
            TimePeriod::TwoHours,
            TimePeriod::FourHours,
            TimePeriod::SixHours,
            TimePeriod::TwelveHours,
            TimePeriod::EighteenHours,
            TimePeriod::TwentyFourHours,
        ];

        let mut results = Vec::new();
        for period in time_periods {
            let history = TickerState::get_history(period.clone())?;
            let total_volume = history.len(); // Number of updates in this time period
            let period_seconds = self.get_duration_seconds(&period); // Duration in seconds
            let per_second_volume = if period_seconds > 0 {
                total_volume as f64 / period_seconds as f64
            } else {
                0.0
            };

            // Calculate the average price
            let total_price: f64 = history.iter().map(|state| state.price).sum();
            let average_price = if total_volume > 0 {
                total_price / total_volume as f64
            } else {
                0.0
            };

            results.push((period, (total_volume, per_second_volume, average_price)));
        }

        Ok(results)
    }

    fn get_duration_seconds(&self, time_period: &TimePeriod) -> usize {
        match time_period {
            TimePeriod::OneSecond => 1,
            TimePeriod::FiveSeconds => 5,
            TimePeriod::TenSeconds => 10,
            TimePeriod::FifteenSeconds => 15,
            TimePeriod::ThirtySeconds => 30,
            TimePeriod::OneMinute => 60,
            TimePeriod::FiveMinutes => 300,
            TimePeriod::FifteenMinutes => 900,
            TimePeriod::OneHour => 3600,
            TimePeriod::TwoHours => 7200,
            TimePeriod::FourHours => 14400,
            TimePeriod::SixHours => 21600,
            TimePeriod::TwelveHours => 43200,
            TimePeriod::EighteenHours => 64800,
            TimePeriod::TwentyFourHours => 86400,
        }
    }
}
