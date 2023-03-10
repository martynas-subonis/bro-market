use crate::stats::statistics::{mean, std};

#[derive(Debug)]
pub struct AgentRunStats<'a> {
    pub name: &'a str,
    pub trade_count: usize,
    pub net_worth: f64,
}
