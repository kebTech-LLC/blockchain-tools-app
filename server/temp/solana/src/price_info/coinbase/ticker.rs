use std::sync::{Arc, RwLock};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use state::InitCell;

pub static TICKER_STATE: InitCell<Arc<RwLock<TickerState>>> = InitCell::new();
pub static TICKER_HISTORY: InitCell<Arc<RwLock<Vec<TickerState>>>> = InitCell::new();

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum TimePeriod {
    OneSecond,
    FiveSeconds,
    TenSeconds,
    FifteenSeconds,
    ThirtySeconds,
    OneMinute,
    FiveMinutes,
    FifteenMinutes,
    OneHour,
    TwoHours,
    FourHours,
    SixHours,
    TwelveHours,
    EighteenHours,
    TwentyFourHours,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TickerState {
    pub price: f64,
    pub time: i64, // Store Unix timestamp for efficient queries
}

impl TickerState {
    pub fn init() {
        TICKER_STATE.set(Arc::new(RwLock::new(TickerState::new(0.0, 0))));
        TICKER_HISTORY.set(Arc::new(RwLock::new(Vec::new())));
    }

    pub fn new(price: f64, time: i64) -> Self {
        Self { price, time }
    }

    /// Parse an ISO 8601 time string into a Unix timestamp
    pub fn from_iso8601(price: f64, time_str: &str) -> anyhow::Result<Self> {
        let parsed_time: DateTime<Utc> = time_str
            .parse()
            .map_err(|_| anyhow::anyhow!("Failed to parse time string"))?;
        Ok(Self {
            price,
            time: parsed_time.timestamp(),
        })
    }

    pub fn add_to_current(&self) -> anyhow::Result<()> {
        let mut state = TICKER_STATE.get().write().map_err(|_| anyhow::anyhow!("Ticker state is poisoned"))?;
        state.price = self.price;
        state.time = self.time;

        Ok(())
    }

    pub fn add_to_history(&self) -> anyhow::Result<()> {
        let mut history = TICKER_HISTORY.get().write().map_err(|_| anyhow::anyhow!("Ticker history is poisoned"))?;
        history.push(self.clone());

        Ok(())
    }

    pub fn update(&self) -> anyhow::Result<()> {
        self.add_to_current()?;
        self.add_to_history()?;

        Ok(())
    }

    pub fn get_history(time_period: TimePeriod) -> anyhow::Result<Vec<Self>> {
        let history = TICKER_HISTORY.get().read().map_err(|_| anyhow::anyhow!("Ticker history is poisoned"))?;
        let now = Utc::now().timestamp();

        let threshold = match time_period {
            TimePeriod::OneSecond => now - 1,
            TimePeriod::FiveSeconds => now - 5,
            TimePeriod::TenSeconds => now - 10,
            TimePeriod::FifteenSeconds => now - 15,
            TimePeriod::ThirtySeconds => now - 30,
            TimePeriod::OneMinute => now - 60,
            TimePeriod::FiveMinutes => now - 300,
            TimePeriod::FifteenMinutes => now - 900,
            TimePeriod::OneHour => now - 3600,
            TimePeriod::TwoHours => now - 7200,
            TimePeriod::FourHours => now - 14400,
            TimePeriod::SixHours => now - 21600,
            TimePeriod::TwelveHours => now - 43200,
            TimePeriod::EighteenHours => now - 64800,
            TimePeriod::TwentyFourHours => now - 86400,
        };

        let filtered_history: Vec<Self> = history.iter().cloned().filter(|state| state.time >= threshold).collect();
        Ok(filtered_history)
    }

    pub fn get_average_price(time_period: TimePeriod) -> anyhow::Result<f64> {
        let history = Self::get_history(time_period)?;
        let total_price: f64 = history.iter().map(|state| state.price).sum();
        let count = history.len();
        if count == 0 {
            return Err(anyhow::anyhow!("No data available for the given time period"));
        }
        Ok(total_price / count as f64)
    }

    pub fn get_average_price_per_second(time_period: TimePeriod) -> anyhow::Result<f64> {
        let history = Self::get_history(time_period.clone())?;
        let total_price: f64 = history.iter().map(|state| state.price).sum();
        let count = history.len();
        if count == 0 {
            return Err(anyhow::anyhow!("No data available for the given time period"));
        }
        let duration_seconds = Self::get_duration_seconds(&time_period);
        Ok(total_price / count as f64 / duration_seconds as f64)
    }

    pub fn get_duration_seconds(time_period: &TimePeriod) -> usize {
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

    pub fn get_total_volume(time_period: TimePeriod) -> anyhow::Result<usize> {
        let history = Self::get_history(time_period)?;
        Ok(history.len())
    }

    pub fn get_current_state() -> anyhow::Result<Self> {
        let state = TICKER_STATE.get().read().map_err(|_| anyhow::anyhow!("Ticker state is poisoned"))?;
        Ok(state.clone())
    }

    pub fn get_current_price() -> anyhow::Result<f64> {
        let state = Self::get_current_state()?;
        Ok(state.price)
    }
}
