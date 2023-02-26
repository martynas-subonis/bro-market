mod domain_models;
mod pseudo_strategies;

use crate::domain_models::agent::Agent;
use crate::domain_models::agent::Strategies::HeadAndShoulders;
use crate::domain_models::market::Market;
use crate::domain_models::bros::get_bros;
use crate::pseudo_strategies::head_and_shoulders::{is_head_and_shoulders};

const NUMBER_OF_DAYS: usize = 1000;
const NUMBER_OF_HOURS: usize = 24 * NUMBER_OF_DAYS;


fn main() {
    let mut market = Market::new();
    let mut bros: Vec<Agent> = get_bros();
    let mut hours: Vec<f64> = vec![0.0];

    let mut times_stocks_matched: usize = 0;
    for h in 1..NUMBER_OF_HOURS {
        hours.push(h as f64);
        for stock in market.stocks.iter_mut() {
            stock.mv();
            match is_head_and_shoulders(&hours, &stock.history) {
                Ok(is) => match is {
                    true => {
                        for bro in bros.iter_mut() {
                            if bro.strategies.contains(&HeadAndShoulders) {
                                bro.buy(stock, h);
                            }
                        }
                        times_stocks_matched += 1;
                    }
                    false => ()
                }
                Err(err) => {
                    println!("An error occurred: {}", err)
                }
            }
        }
    }
    println!("Times stocks matched: {}", times_stocks_matched);
    for bro in bros {
        println!("Bro name: {:?}", bro.name);
        println!("Bro trades: {:?}", bro.trades);
        println!("Bro portfolio: {:?}", bro.portfolio);
    }
}
