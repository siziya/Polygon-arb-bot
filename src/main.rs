use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub polygon_rpc_url: String,
    pub dex1_router: String,
    pub dex2_router: String,
    pub weth: String,
    pub usdc: String,
    pub min_profit_usdc: f64,
    pub trade_amount_weth: f64,
    pub gas_cost_usdc: f64,
    pub db_path: String,
    pub poll_interval_secs: u64,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            polygon_rpc_url: env::var("POLYGON_RPC_URL")?,
            dex1_router: env::var("DEX1_ROUTER")?,
            dex2_router: env::var("DEX2_ROUTER")?,
            weth: env::var("WETH")?,
            usdc: env::var("USDC")?,
            min_profit_usdc: env::var("MIN_PROFIT_USDC")?.parse()?,
            trade_amount_weth: env::var("TRADE_AMOUNT_WETH")?.parse()?,
            gas_cost_usdc: env::var("GAS_COST_USDC")?.parse()?,
            db_path: env::var("DB_PATH")?,
            poll_interval_secs: env::var("POLL_INTERVAL_SECS")?.parse()?,
        })
    }
}