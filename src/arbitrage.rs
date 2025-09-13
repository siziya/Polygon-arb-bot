use crate::{config::Config, dex, profit};
use ethers::prelude::*;
use std::sync::Arc;

#[derive(Debug)]
pub struct ArbitrageOpportunity {
    pub buy_dex: String,
    pub sell_dex: String,
    pub buy_price: f64,
    pub sell_price: f64,
    pub simulated_profit: f64,
}

pub async fn check_for_arbitrage(config: &Config) -> anyhow::Result<Option<ArbitrageOpportunity>> {
    let provider = Arc::new(Provider::<Http>::try_from(config.polygon_rpc_url.as_str())?);
    let weth = config.weth.parse()?;
    let usdc = config.usdc.parse()?;
    let amount_in = ethers::utils::parse_units(config.trade_amount_weth, 18)?;

    // Get price on dex1 (buy WETH for USDC)
    let out1 = dex::get_amount_out(
        provider.clone(),
        config.dex1_router.parse()?,
        amount_in,
        vec![weth, usdc],
    ).await?;
    // Get price on dex2 (sell WETH for USDC)
    let out2 = dex::get_amount_out(
        provider.clone(),
        config.dex2_router.parse()?,
        amount_in,
        vec![weth, usdc],
    ).await?;

    let price1 = out1.as_u128() as f64 / 1e6; // USDC has 6 decimals
    let price2 = out2.as_u128() as f64 / 1e6;

    let profit = profit::simulate_profit(price1, price2, config);

    if profit >= config.min_profit_usdc {
        Ok(Some(ArbitrageOpportunity {
            buy_dex: config.dex1_router.clone(),
            sell_dex: config.dex2_router.clone(),
            buy_price: price1,
            sell_price: price2,
            simulated_profit: profit,
        }))
    } else {
        Ok(None)
    }
}