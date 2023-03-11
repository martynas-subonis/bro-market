use rand::distributions::{Distribution, Uniform};
use uuid::Uuid;

const NUMBER_OF_STOCKS: usize = 5;
const ABS_NOISE: f64 = 0.001;
const RELATIVE_NOISE: f64 = 0.01;

pub fn get_stocks() -> Vec<Stock> {
    return (0..NUMBER_OF_STOCKS).map(|_| Stock::default()).collect();
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
    fn delta(&self) -> f64 {
        let mut rng = rand::thread_rng();
        let rel_range = Uniform::new(-RELATIVE_NOISE, RELATIVE_NOISE);
        let rel_delta = rel_range.sample(&mut rng);

        let abs_range = Uniform::new(-ABS_NOISE, ABS_NOISE);
        let abs_delta = abs_range.sample(&mut rng);

        return self.price * rel_delta + abs_delta;
    }

    pub(crate) fn mv(&mut self) -> () {
        self.price += self.delta();
        self.history.push(self.price);
    }
}
