use std::collections::{HashMap, HashSet};
use crate::domain_models::agent::{Agent};
use crate::domain_models::agent::Strategies::{HeadAndShoulders, ReverseHeadAndShoulders, DoubleTop, DoubleBottom};

const DEFAULT_STARTING_CASH: f64 = 10000.0;

pub fn get_bros() -> Vec<Agent<'static>> {
    let chad = Agent {
        name: "Chad The Crypto King",
        portfolio: HashMap::new(),
        cash: DEFAULT_STARTING_CASH,
        strategies: HashSet::from([HeadAndShoulders, ReverseHeadAndShoulders]),
        trades: Vec::new(),
    };
    let ben = Agent {
        name: "Ben The Wall Street Intern",
        portfolio: HashMap::new(),
        cash: DEFAULT_STARTING_CASH,
        strategies: HashSet::from([DoubleTop, DoubleBottom]),
        trades: Vec::new(),
    };
    return Vec::from([chad, ben]);
}
