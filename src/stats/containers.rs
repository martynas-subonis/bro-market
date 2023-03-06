use crate::stats::statistics::{mean, std};

#[derive(Debug)]
pub struct AgentStats {
    pub trade_counts: Vec<usize>,
    pub net_worths: Vec<f64>,
}

impl AgentStats {
    pub fn mean_net_worth(&self) -> f64 {
        return mean(&self.net_worths);
    }

    pub fn std_net_worth(&self) -> f64 {
        return std(&self.net_worths);
    }

    pub fn mean_trade_counts(&self) -> f64 {
        let counts: Vec<f64> = self.trade_counts.iter().map(|x| *x as f64).collect();
        return mean(&counts);
    }
}
