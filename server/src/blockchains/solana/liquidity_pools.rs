use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PoolInfoResponse {
    pub id: String,
    pub success: bool,
    pub data: Vec<PoolData>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PoolData {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String, // Handle reserved keyword
    pub program_id: String,
    pub mint_a: TokenInfo,
    pub mint_b: TokenInfo,
    pub tvl: f64,
    pub price: f64,
    pub fee_rate: f64,
    pub burn_percent: f64,
    pub config: PoolConfig,
    pub day: Performance,
    pub week: Performance,
    pub month: Performance,
    pub reward_default_infos: Vec<RewardInfo>,
}

#[derive(Debug, Deserialize)]
pub struct TokenInfo {
    pub address: String,
    #[serde(rename = "chainId")]
    pub chain_id: u64,
    pub decimals: u8,
    pub symbol: String,
    pub name: String,
    #[serde(rename = "logoURI")] // Explicitly map "logoURI"
    pub logo_uri: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PoolConfig {
    pub id: String,
    pub index: u64,
    pub protocol_fee_rate: u64,
    pub trade_fee_rate: u64,
    pub tick_spacing: u64,
    pub default_range: f64,
    pub default_range_point: Vec<f64>,
    pub fund_fee_rate: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Performance {
    pub apr: f64,
    pub fee_apr: f64,
    pub volume: f64,
    pub volume_quote: f64,
    pub volume_fee: f64,
    pub price_min: f64,
    pub price_max: f64,
    pub reward_apr: Vec<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RewardInfo {
    pub mint: TokenInfo,
    #[serde(rename = "perSecond")]
    pub per_second: String, // Handle case-sensitive field
    #[serde(rename = "startTime")]
    pub start_time: String, // Handle case-sensitive field
    #[serde(rename = "endTime")]
    pub end_time: String,   // Handle case-sensitive field
}