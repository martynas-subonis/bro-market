mod domain_models;
mod pseudo_strategies;
mod util;

use crate::domain_models::bros::create_bros;
use crate::domain_models::stochastic::StochasticProcess;
use crate::domain_models::stock::create_stocks;
use lib::{AgentRunStats, NUMBER_OF_HOURS, NUMBER_OF_SIMULATIONS, OUTPUT_FILE_NAME};
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

fn main() {
    println!("Running bro market simulation...");
    let timeline = create_timeline();
    let result = run_simulations(NUMBER_OF_SIMULATIONS, &timeline);
    save_results(&result, OUTPUT_FILE_NAME);
    println!("Results saved to: {}", OUTPUT_FILE_NAME);
}

fn create_timeline() -> [f64; NUMBER_OF_HOURS] {
    let mut hours_array = [0.0; NUMBER_OF_HOURS];
    for (i, hour) in hours_array.iter_mut().enumerate() {
        *hour = i as f64;
    }
    hours_array
}

fn run_simulations(
    num_simulations: usize,
    timeline: &[f64; NUMBER_OF_HOURS],
) -> HashMap<&'static str, Vec<AgentRunStats>> {
    (0..num_simulations)
        .into_par_iter()
        .map(|id| simulate_single_run(id, timeline))
        .reduce(HashMap::new, |mut acc, map| {
            for (key, mut value) in map.into_iter() {
                let stats_vec = acc.entry(key).or_insert_with(Vec::new);
                stats_vec.append(&mut value);
            }
            acc
        })
}

fn simulate_single_run(
    s_id: usize,
    timeline: &[f64; NUMBER_OF_HOURS],
) -> HashMap<&'static str, Vec<AgentRunStats>> {
    let start = Instant::now();
    let mut sp = StochasticProcess::default();
    let mut stocks = create_stocks();
    let mut bros = create_bros();

    for h in 1..NUMBER_OF_HOURS {
        for stock in stocks.iter_mut() {
            stock.mv(&mut sp);
            for bro in bros.iter_mut() {
                if bro.strategy.should_buy(stock, timeline) {
                    bro.buy(stock, h)
                }
                if bro.strategy.should_sell(stock, timeline) {
                    bro.sell(stock, h)
                }
            }
        }
    }

    let result = bros
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
        .collect();

    let duration = start.elapsed();
    println!("Simulation ID-{} completed in {:?}.", s_id, duration);
    result
}

fn save_results(result: &HashMap<&str, Vec<AgentRunStats>>, file_name: &str) {
    let serialized = serde_json::to_string(result).unwrap();
    let mut file = File::create(file_name).unwrap();
    file.write_all(serialized.as_bytes()).unwrap();
}
