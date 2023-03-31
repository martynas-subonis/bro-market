use crate::domain_models::stochastic::StochasticProcess;
use uuid::Uuid;

const NUMBER_OF_STOCKS: usize = 5;

pub fn create_stocks() -> Vec<Stock> {
    (0..NUMBER_OF_STOCKS).map(|_| Stock::default()).collect()
}

#[derive(Debug, Clone)]
pub struct Stock {
    pub id: String,
    pub price: f64,
    pub history: Vec<f64>,
}

impl Default for Stock {
    fn default() -> Stock {
        Stock {
            id: Uuid::new_v4().to_string(),
            price: 1.0,
            history: vec![1.0],
        }
    }
}

impl Stock {
    pub(crate) fn mv(&mut self, sp: &mut StochasticProcess) {
        let s = sp.sample();
        let delta = self.price * s.rel + s.abs;
        self.price += delta;
        self.history.push(self.price);
    }
}
