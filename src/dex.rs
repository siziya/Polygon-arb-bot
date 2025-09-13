use ethers::{
    prelude::*,
    contract::abigen,
    types::{Address, U256},
};
use std::sync::Arc;

abigen!(
    UniswapV2Router,
    "./abi/UniswapV2Router.json"
);

pub async fn get_amount_out(
    client: Arc<Provider<Http>>, 
    router_address: Address,
    amount_in: U256,
    path: Vec<Address>,
) -> anyhow::Result<U256> {
    let router = UniswapV2Router::new(router_address, client);
    let amounts = router.get_amounts_out(amount_in, path).call().await?;
    // amounts[amounts.len() - 1] is the final output
    Ok(*amounts.last().unwrap())
}