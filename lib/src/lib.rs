use serde::{Deserialize, Serialize};

// Market definition
pub const NUMBER_OF_STOCKS: usize = 5;
pub const ABS_NOISE: f64 = 0.001;
pub const RELATIVE_NOISE: f64 = 0.01;
pub const TRADING_FEE: f64 = 0.005;

// Agents definitions
pub const DEFAULT_STARTING_CASH: f64 = 10000.0;
pub const CHAD_NAME: &str = "Chad The Crypto King";
pub const BEN_NAME: &str = "Ben The Wall Street Intern";

// Trading strategies definitions
const SCALE_FACTOR: usize = 36;
pub const SMALL_WINDOW: usize = SCALE_FACTOR;
pub const MEDIUM_WINDOW: usize = SMALL_WINDOW * SCALE_FACTOR;
pub const LARGE_WINDOW: usize = MEDIUM_WINDOW * SCALE_FACTOR;

pub const ALLOWED_RELATIVE_DIFF: f64 = 0.3;
pub const TRADE_FRACTION: f64 = 0.4;

// Simulation definition
pub const NUMBER_OF_SIMULATIONS: usize = 5000;
const NUMBER_OF_DAYS: usize = 1000;
pub const NUMBER_OF_HOURS: usize = 24 * NUMBER_OF_DAYS;

// Results definition
pub const OUTPUT_FILE_NAME: &str = "generated/mc.json";
pub const NETWORTH_PLOT_FILE_NAME: &str = "generated/networth.png";
pub const TRADE_COUNT_PLOT_FILE_NAME: &str = "generated/trade_count.png";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AgentRunStats {
    pub trade_count: usize,
    pub net_worth: f64,
}
