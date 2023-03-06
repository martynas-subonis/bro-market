mod util;
mod domain_models;
mod pseudo_strategies;
mod stats;

use std::collections::HashMap;
use crate::domain_models::market::Market;
use crate::domain_models::bros::{CHAD_NAME, BEN_NAME, get_bros};
use crate::pseudo_strategies::execute_strategy::execute_strategy;
use crate::stats::containers::AgentStats;

const NUMBER_OF_SIMULATIONS: usize = 5;
const NUMBER_OF_DAYS: usize = 1000;
const NUMBER_OF_HOURS: usize = 24 * NUMBER_OF_DAYS;


fn main() {
    let mut stats: HashMap<&str, AgentStats> = get_bros()
        .iter()
        .map(|b| (b.name, AgentStats { trade_counts: Vec::new(), net_worths: Vec::new() }))
        .collect();

    for _ in 0..NUMBER_OF_SIMULATIONS {
        let mut market = Market::new();
        let mut bros = get_bros();

        for h in 1..NUMBER_OF_HOURS {
            for stock in market.stocks.iter_mut() {
                stock.mv();
                for bro in bros.iter_mut() {
                    execute_strategy(stock, bro, h);
                }
            }
        }

        for bro in bros {
            let bro_stats = stats.get_mut(bro.name).unwrap();
            bro_stats.net_worths.push(bro.get_net_worth(&market.stocks));
            bro_stats.trade_counts.push(bro.trades.len());
        }
    }
    println!("Stats: {:?}", stats);
}
