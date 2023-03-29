use crate::domain_models::agent::Agent;
use crate::domain_models::agent::Strategies::{
    DoubleBottom, DoubleTop, HeadAndShoulders, ReverseHeadAndShoulders,
};
use crate::domain_models::stock::Stock;
use crate::pseudo_strategies::double_bottom::is_double_bottom;
use crate::pseudo_strategies::double_top::is_double_top;
use crate::pseudo_strategies::head_and_shoulders::is_head_and_shoulders;
use crate::pseudo_strategies::reverse_head_and_shoulders::is_reverse_head_and_shoulders;
use lib::NUMBER_OF_HOURS;

pub fn execute_strategy(
    stock: &Stock,
    agent: &mut Agent,
    h: usize,
    timeline: &[f64; NUMBER_OF_HOURS],
) -> () {
    if agent.strategies.contains(&HeadAndShoulders) {
        if is_head_and_shoulders(&stock.history, timeline) {
            agent.sell(stock, h);
        }
    }
    if agent.strategies.contains(&ReverseHeadAndShoulders) {
        if is_reverse_head_and_shoulders(&stock.history, timeline) {
            agent.buy(stock, h);
        }
    }
    if agent.strategies.contains(&DoubleTop) {
        if is_double_top(&stock.history, timeline) {
            agent.sell(stock, h);
        }
    }
    if agent.strategies.contains(&DoubleBottom) {
        if is_double_bottom(&stock.history, timeline) {
            agent.buy(stock, h);
        }
    }
}
