use serde::{Deserialize, Serialize};

pub const OUTPUT_FILE_NAME: &str = "generated/mc.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentRunStats {
    pub trade_count: usize,
    pub net_worth: f64,
}
