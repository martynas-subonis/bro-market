use crate::domain_models::agent::{Agent, Strategies};
use crate::domain_models::stock::Stock;
use crate::pseudo_strategies::head_and_shoulders::is_head_and_shoulders;
use crate::pseudo_strategies::reverse_head_and_shoulders::is_reverse_head_and_shoulders;

pub fn execute_strategy(stock: &Stock, agent: &mut Agent, time: usize) -> () {
    if agent.strategies.contains(&Strategies::HeadAndShoulders) {
        if is_head_and_shoulders(&stock.history) {
            agent.sell(stock, time);
        }
    }
    if agent.strategies.contains(&Strategies::ReverseHeadAndShoulders) {
        if is_reverse_head_and_shoulders(&stock.history) {
            agent.buy(stock, time);
        }
    }
}
