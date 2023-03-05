use std::collections::{HashMap, HashSet};
use crate::domain_models::agent::{Agent, Strategies};

const BRO_1_NAME: &str = "Chad The Crypto King";
const BRO_2_NAME: &str = "Ben The Wall Street Intern";
const BRO_3_NAME: &str = "Kyle The Reddit Wise";
const DEFAULT_STARTING_CASH: f64 = 10000.0;


pub fn get_bros() -> Vec<Agent<'static>> {
    vec![
        Agent {
            name: BRO_1_NAME,
            portfolio: HashMap::new(),
            cash: DEFAULT_STARTING_CASH,
            strategies: HashSet::from([Strategies::HeadAndShoulders, Strategies::ReverseHeadAndShoulders]),
            trades: Vec::new(),
        },
        Agent {
            name: BRO_2_NAME,
            portfolio: HashMap::new(),
            cash: DEFAULT_STARTING_CASH,
            strategies: HashSet::new(),
            trades: Vec::new(),
        },
        Agent {
            name: BRO_3_NAME,
            portfolio: HashMap::new(),
            cash: DEFAULT_STARTING_CASH,
            strategies: HashSet::new(),
            trades: Vec::new(),
        },
    ]
}
