mod util;
mod domain_models;
mod pseudo_strategies;
mod stats;

use std::sync::{Arc, Mutex};
use std::thread;
use std::collections::HashMap;
use crate::domain_models::bros::{CHAD_NAME, BEN_NAME, get_bros};
use crate::domain_models::stock::get_stock;
use crate::pseudo_strategies::execute_strategy::execute_strategy;
use crate::stats::containers::AgentRunStats;

const NUMBER_OF_SIMULATIONS: usize = 20;
const NUMBER_OF_DAYS: usize = 1000;
const NUMBER_OF_HOURS: usize = 24 * NUMBER_OF_DAYS;


fn main() {
    let mut mutex_stats: HashMap<&str, Vec<AgentRunStats>> = get_bros()
        .iter()
        .map(|b| (b.name, Vec::new()))
        .collect();
    let atomic_stats = Arc::new(Mutex::new(mutex_stats));
    let mut handles = vec![];

    for _ in 0..NUMBER_OF_SIMULATIONS {
        let stats_clone = Arc::clone(&atomic_stats);
        let handle = thread::spawn(move || {
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

            let mut stats = stats_clone.lock().unwrap();
            for bro in bros {
                let run_stats = AgentRunStats {
                    trade_count: bro.trades.len(),
                    net_worth: bro.get_net_worth(&stocks),
                };

                let bro_stats = stats.get_mut(bro.name).unwrap();
                bro_stats.push(run_stats);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Stats: {:?}", *atomic_stats.lock().unwrap());
}
