use crate::domain_models::agent::Agent;
use crate::domain_models::agent::Strategies::{
    DoubleBottom, DoubleTop, HeadAndShouldersBottom, HeadAndShouldersTop,
};
use crate::domain_models::stock::Stock;
use crate::pseudo_strategies::double_bottom::is_double_bottom;
use crate::pseudo_strategies::double_top::is_double_top;
use crate::pseudo_strategies::head_and_shoulders_bottom::is_head_and_shoulders_bottom;
use crate::pseudo_strategies::head_and_shoulders_top::is_head_and_shoulders_top;
use lib::NUMBER_OF_HOURS;

pub fn execute_strategy(
    stock: &Stock,
    agent: &mut Agent,
    h: usize,
    timeline: &[f64; NUMBER_OF_HOURS],
) {
    if agent.strategies.contains(&HeadAndShouldersTop) && is_head_and_shoulders_top(&stock.history, timeline) {
        agent.sell(stock, h);
    }
    if agent.strategies.contains(&HeadAndShouldersBottom) && is_head_and_shoulders_bottom(&stock.history, timeline) {
        agent.buy(stock, h);
    }
    if agent.strategies.contains(&DoubleTop) && is_double_top(&stock.history, timeline) {
        agent.sell(stock, h);
    }
    if agent.strategies.contains(&DoubleBottom) && is_double_bottom(&stock.history, timeline) {
        agent.buy(stock, h);
    }
}
