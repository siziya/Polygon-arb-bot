use crate::config::Config;

pub fn simulate_profit(buy_price: f64, sell_price: f64, config: &Config) -> f64 {
    let gross_profit = sell_price - buy_price;
    let net_profit = gross_profit - config.gas_cost_usdc;
    net_profit
}