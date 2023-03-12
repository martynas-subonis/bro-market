use serde::{Deserialize, Serialize};

pub const CHAD_NAME: &str = "Chad The Crypto King";
pub const BEN_NAME: &str = "Ben The Wall Street Intern";

pub const OUTPUT_FILE_NAME: &str = "generated/mc.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentRunStats {
    pub trade_count: usize,
    pub net_worth: f64,
}
