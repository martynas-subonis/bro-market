use crate::domain_models::agent::{Agent, Strategy};
use crate::domain_models::stock::Stock;
use crate::pseudo_strategies::double_bottom::is_double_bottom;
use crate::pseudo_strategies::double_top::is_double_top;
use crate::pseudo_strategies::head_and_shoulders_bottom::is_head_and_shoulders_bottom;
use crate::pseudo_strategies::head_and_shoulders_top::is_head_and_shoulders_top;
use lib::{BEN_NAME, CHAD_NAME, DEFAULT_STARTING_CASH, NUMBER_OF_HOURS};
use std::collections::HashMap;

pub struct HeadAndShoulders {}
impl Strategy for HeadAndShoulders {
    fn should_buy(&self, stock: &Stock, timeline: &[f64; NUMBER_OF_HOURS]) -> bool {
        is_head_and_shoulders_bottom(&stock.history, timeline)
    }

    fn should_sell(&self, stock: &Stock, timeline: &[f64; NUMBER_OF_HOURS]) -> bool {
        is_head_and_shoulders_top(&stock.history, timeline)
    }
}

pub struct Double {}

impl Strategy for Double {
    fn should_buy(&self, stock: &Stock, timeline: &[f64; NUMBER_OF_HOURS]) -> bool {
        is_double_bottom(&stock.history, timeline)
    }

    fn should_sell(&self, stock: &Stock, timeline: &[f64; NUMBER_OF_HOURS]) -> bool {
        is_double_top(&stock.history, timeline)
    }
}

pub fn create_bros() -> Vec<Agent<'static>> {
    let chad = Agent {
        name: CHAD_NAME,
        portfolio: HashMap::new(),
        cash: DEFAULT_STARTING_CASH,
        trades: Vec::new(),
        strategy: Box::new(HeadAndShoulders {}),
    };
    let ben = Agent {
        name: BEN_NAME,
        portfolio: HashMap::new(),
        cash: DEFAULT_STARTING_CASH,
        trades: Vec::new(),
        strategy: Box::new(Double {}),
    };
    Vec::from([chad, ben])
}
