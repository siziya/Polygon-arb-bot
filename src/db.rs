use rusqlite::{params, Connection};
use crate::arbitrage::ArbitrageOpportunity;

pub fn init_db(path: &str) -> anyhow::Result<Connection> {
    let conn = Connection::open(path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS opportunities (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
            buy_dex TEXT,
            sell_dex TEXT,
            buy_price REAL,
            sell_price REAL,
            simulated_profit REAL
        )",
        [],
    )?;
    Ok(conn)
}

pub fn log_opportunity(conn: &Connection, opp: &ArbitrageOpportunity) -> anyhow::Result<()> {
    conn.execute(
        "INSERT INTO opportunities (buy_dex, sell_dex, buy_price, sell_price, simulated_profit) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            opp.buy_dex,
            opp.sell_dex,
            opp.buy_price,
            opp.sell_price,
            opp.simulated_profit
        ],
    )?;
    Ok(())
}