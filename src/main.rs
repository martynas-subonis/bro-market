mod util;
mod domain_models;
mod pseudo_strategies;

use crate::domain_models::agent::Agent;
use crate::domain_models::agent::Strategies::HeadAndShoulders;
use crate::domain_models::market::Market;
use crate::domain_models::bros::get_bros;
use crate::pseudo_strategies::execute_strategy::execute_strategy;
use crate::pseudo_strategies::head_and_shoulders::{is_head_and_shoulders};

const NUMBER_OF_DAYS: usize = 1000;
const NUMBER_OF_HOURS: usize = 24 * NUMBER_OF_DAYS;


fn main() {
    let mut market = Market::new();
    let mut bros: Vec<Agent> = get_bros();

    for h in 1..NUMBER_OF_HOURS {
        for stock in market.stocks.iter_mut() {
            stock.mv();
            for bro in bros.iter_mut() {
                execute_strategy(stock, bro, h);
            }
        }
    }
    for bro in bros {
        println!("Bro name: {:?}", bro.name);
        println!("Bro trades: {:?}", bro.trades);
        println!("Bro portfolio: {:?}", bro.portfolio);
    }
}
