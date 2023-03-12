use crate::domain_models::agent::Agent;
use crate::domain_models::agent::Strategies::{
    DoubleBottom, DoubleTop, HeadAndShoulders, ReverseHeadAndShoulders,
};
use lib::{BEN_NAME, CHAD_NAME};
use std::collections::{HashMap, HashSet};

const DEFAULT_STARTING_CASH: f64 = 10000.0;

pub fn get_bros() -> Vec<Agent<'static>> {
    let chad = Agent {
        name: CHAD_NAME,
        portfolio: HashMap::new(),
        cash: DEFAULT_STARTING_CASH,
        strategies: HashSet::from([HeadAndShoulders, ReverseHeadAndShoulders]),
        trades: Vec::new(),
    };
    let ben = Agent {
        name: BEN_NAME,
        portfolio: HashMap::new(),
        cash: DEFAULT_STARTING_CASH,
        strategies: HashSet::from([DoubleTop, DoubleBottom]),
        trades: Vec::new(),
    };
    return Vec::from([chad, ben]);
}