mod util;
mod domain_models;
mod pseudo_strategies;
mod stats;

use rayon::prelude::*;
use std::collections::HashMap;
use crate::domain_models::bros::{get_bros};
use crate::domain_models::stock::get_stock;
use crate::pseudo_strategies::execute_strategy::execute_strategy;
use crate::stats::containers::AgentRunStats;

const NUMBER_OF_SIMULATIONS: usize = 20;
const NUMBER_OF_DAYS: usize = 1000;
const NUMBER_OF_HOURS: usize = 24 * NUMBER_OF_DAYS;


fn main() {
    let result: Vec<AgentRunStats> = (0..NUMBER_OF_SIMULATIONS)
        .into_par_iter()
        .map(|_| {
            let mut stocks = get_stock();
            let mut bros = get_bros();

            for h in 1..NUMBER_OF_HOURS {
                for stock in stocks.iter_mut() {
                    stock.mv();
                    for bro in bros.iter_mut() {
                        execute_strategy(stock, bro, h);
                    }
                }
            }
            let stats: Vec<AgentRunStats> = bros
                .iter()
                .map(|bro| AgentRunStats {
                    name: bro.name,
                    trade_count: bro.trades.len(),
                    net_worth: bro.get_net_worth(&stocks),
                })
                .collect();
            return stats;
        })
        .flatten()
        .collect();

    println!("Stats: {:?}", result);
}
