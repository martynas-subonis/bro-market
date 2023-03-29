mod domain_models;
mod pseudo_strategies;
mod util;

use crate::domain_models::bros::get_bros;
use crate::domain_models::stochastic::StochasticProcess;
use crate::domain_models::stock::get_stocks;
use crate::pseudo_strategies::execute_strategy::execute_strategy;
use lib::{AgentRunStats, NUMBER_OF_HOURS, NUMBER_OF_SIMULATIONS, OUTPUT_FILE_NAME};
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    println!("Starting bro market simulation...");
    let timeline = lib::create_timeline();
    let result: HashMap<&str, Vec<AgentRunStats>> = (0..NUMBER_OF_SIMULATIONS)
        .into_par_iter()
        .map(|_| {
            let mut sp = StochasticProcess::default();
            let mut stocks = get_stocks();
            let mut bros = get_bros();

            for h in 1..NUMBER_OF_HOURS {
                for stock in stocks.iter_mut() {
                    stock.mv(&mut sp);
                    for bro in bros.iter_mut() {
                        execute_strategy(stock, bro, h, &timeline);
                    }
                }
            }
            return bros
                .iter()
                .map(|bro| {
                    (
                        bro.name,
                        Vec::from([AgentRunStats {
                            trade_count: bro.trades.len(),
                            net_worth: bro.get_net_worth(&stocks),
                        }]),
                    )
                })
                .collect::<HashMap<&str, Vec<AgentRunStats>>>();
        })
        .reduce(
            || HashMap::new(),
            |mut acc, map| {
                for (key, mut value) in map.into_iter() {
                    let stats_vec = acc.entry(key).or_insert_with(Vec::new);
                    stats_vec.append(&mut value);
                }
                return acc;
            },
        );

    println!("Saving results...");
    let serialized = serde_json::to_string(&result).unwrap();
    let mut file = File::create(OUTPUT_FILE_NAME).unwrap();
    file.write_all(serialized.as_bytes()).unwrap();
    println!("Results saved to: {}", OUTPUT_FILE_NAME);
}
