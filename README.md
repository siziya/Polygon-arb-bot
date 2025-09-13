# Polygon Arbitrage Opportunity Detector Bot

This Rust bot periodically checks the prices of a specific token pair (e.g., WETH/USDC) on two different DEXes on Polygon. If a significant price difference is detected, it logs the simulated arbitrage opportunity and profit to a SQLite database.

## Features

- Fetches prices from two DEXes using standard UniswapV2 routers
- Simulates arbitrage profit for a fixed trade size and gas cost
- Logs profitable opportunities (above a threshold) to SQLite
- Configurable via `.env` file

## Technology Stack

- **Rust** (async, type-safe, high-performance)
- **ethers-rs** for blockchain interaction
- **rusqlite** for database logging
- **dotenv** for simple configuration

## Setup

### 1. Clone the repository

```
git clone https://github.com/siziya/Polygon-arb-bot.git
cd Polygon-arb-bot
```

### 2. Install Rust toolchain

https://rustup.rs/

### 3. Set up configuration

Copy `.env.example` to `.env` and edit as needed:

```
cp .env.example .env
```

- Fill in Polygon RPC URL, DEX router/token addresses, etc.

### 4. Build and run

```
cargo run --release
```

## Database

Profitable opportunities are logged in SQLite (`opportunities.db` by default).  
You can view them using any SQLite client.

## Project Structure

- `src/main.rs` - Entrypoint and main loop
- `src/config.rs` - Loads environment config
- `src/dex.rs` - DEX price fetch logic
- `src/arbitrage.rs` - Arbitrage detection logic
- `src/profit.rs` - Simulated profit calculation
- `src/db.rs` - SQLite logging
- `abi/UniswapV2Router.json` - ABI for price fetching

## Extending

- Add more DEXes by providing their router addresses
- Swap in different token pairs (update addresses in `.env`)
- Add more logging or notification logic as needed

---

**For any issues or feature requests, open a GitHub issue. PRs welcome!**