use crate::stats::statistics::{mean, std};

#[derive(Debug)]
pub struct AgentRunStats {
    pub trade_count: usize,
    pub net_worth: f64,
}
