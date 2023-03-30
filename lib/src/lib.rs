use serde::{Deserialize, Serialize};

pub const CHAD_NAME: &str = "Chad The Crypto King";
pub const BEN_NAME: &str = "Ben The Wall Street Intern";

pub const OUTPUT_FILE_NAME: &str = "generated/mc.json";

const NUMBER_OF_DAYS: usize = 1000;
pub const NUMBER_OF_HOURS: usize = 24 * NUMBER_OF_DAYS;

pub const DEFAULT_STARTING_CASH: f64 = 10000.0;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AgentRunStats {
    pub trade_count: usize,
    pub net_worth: f64,
}
