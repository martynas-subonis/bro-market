use crate::domain_models::stock::Stock;
use lib::{NUMBER_OF_HOURS, TRADE_FRACTION, TRADING_FEE};
use std::collections::hash_map::Entry::Vacant;
use std::collections::HashMap;

pub trait Strategy {
    fn should_buy(&self, _stock: &Stock, _timeline: &[f64; NUMBER_OF_HOURS]) -> bool {
        false
    }
    fn should_sell(&self, _stock: &Stock, _timeline: &[f64; NUMBER_OF_HOURS]) -> bool {
        false
    }
}

#[derive(Debug, PartialEq)]
pub struct Trade {
    pub id: String,
    pub units: f64,
    pub time: usize,
}

pub struct Agent<'a> {
    pub name: &'a str,
    pub portfolio: HashMap<String, f64>,
    pub cash: f64,
    pub trades: Vec<Trade>,
    pub strategy: Box<dyn Strategy>,
}

impl Agent<'_> {
    pub fn buy(&mut self, stock: &Stock, time: usize) {
        let trade_amount = TRADE_FRACTION * self.cash;
        let fee_amount = TRADING_FEE * trade_amount;
        self.cash -= trade_amount;

        let buy_units = (trade_amount - fee_amount) / stock.price;
        let stock_id = stock.id.clone();
        let stock_id_cl = stock_id.clone();

        if let Vacant(e) = self.portfolio.entry(stock_id) {
            e.insert(buy_units);
        } else if let Some(val) = self.portfolio.get_mut(&stock_id_cl) {
            *val += buy_units;
        }

        self.trades.push(Trade {
            id: stock_id_cl,
            units: buy_units,
            time,
        });
    }

    pub fn sell(&mut self, stock: &Stock, time: usize) {
        let stock_id = stock.id.clone();
        let stock_id_clone = stock_id.clone();

        if self.portfolio.contains_key(&stock_id) {
            let sell_units = self.portfolio.get_mut(&stock_id).unwrap();
            let sell_amount = *sell_units * stock.price;
            let fee_amount = TRADING_FEE * sell_amount;
            self.cash += sell_amount - fee_amount;
            self.trades.push(Trade {
                id: stock_id,
                units: -*sell_units,
                time,
            });
            self.portfolio.remove(&stock_id_clone);
        }
    }

    pub fn get_net_worth(&self, stocks: &[Stock]) -> f64 {
        let mut net_worth = self.cash;
        let stock_price_map: HashMap<String, f64> =
            stocks.iter().map(|s| (s.id.clone(), s.price)).collect();

        for k in self.portfolio.keys() {
            let amount = self.portfolio.get(k).unwrap();
            let price = stock_price_map.get(k).unwrap();
            net_worth += amount * price;
        }

        net_worth
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct MockStrategy;
    impl Strategy for MockStrategy {}

    #[test]
    fn buys_first_stock() {
        let initial_cash = 5000.0;
        let mut agent = Agent {
            name: "test",
            portfolio: Default::default(),
            cash: initial_cash,
            trades: Vec::new(),
            strategy: Box::new(MockStrategy),
        };
        let stock = Stock {
            id: "test".to_string(),
            price: 2.0,
            history: Vec::new(),
        };
        agent.buy(&stock, 1);
        assert_eq!(agent.portfolio[&stock.id], 995.0);
        assert_eq!(agent.cash, 3000.0);
        assert!(agent.trades.iter().eq(vec![Trade {
            id: "test".to_string(),
            units: 995.0,
            time: 1,
        }]
        .iter()));
    }

    #[test]
    fn buys_when_stock_already_in_portfolio() {
        let stock = Stock {
            id: "test".to_string(),
            price: 2.0,
            history: vec![],
        };
        let initial_cash = 5000.0;
        let initial_amount = 1000.0;
        let mut agent = Agent {
            name: "test",
            portfolio: HashMap::from([(stock.id.clone(), initial_amount)]),
            cash: initial_cash,
            trades: Vec::new(),
            strategy: Box::new(MockStrategy),
        };
        agent.buy(&stock, 1);
        assert_eq!(agent.portfolio[&stock.id], 1995.0);
        assert_eq!(agent.cash, 3000.0);
    }

    #[test]
    fn sells_full_stock_amount() {
        let stock = Stock {
            id: "test".to_string(),
            price: 2.0,
            history: vec![],
        };
        let initial_cash = 500.0;
        let initial_amount = 1000.0;
        let mut agent = Agent {
            name: "test",
            portfolio: HashMap::from([(stock.id.clone(), initial_amount)]),
            cash: initial_cash,
            trades: Vec::new(),
            strategy: Box::new(MockStrategy),
        };
        agent.sell(&stock, 1);
        assert!(!agent.portfolio.contains_key(&stock.id));
        assert_eq!(agent.cash, 2490.0);
    }

    #[test]
    fn sell_does_not_happen_if_stock_is_not_in_portfolio() {
        let stock_one = Stock {
            id: "test_1".to_string(),
            price: 2.0,
            history: vec![],
        };
        let stock_two = Stock {
            id: "test_2".to_string(),
            price: 2.0,
            history: vec![],
        };
        let initial_cash = 0.0;
        let initial_amount = 100.0;
        let mut agent = Agent {
            name: "test",
            portfolio: HashMap::from([(stock_one.id, initial_amount)]),
            cash: initial_cash,
            trades: Vec::new(),
            strategy: Box::new(MockStrategy),
        };
        agent.sell(&stock_two, 1);
        assert_eq!(agent.cash, initial_cash);
    }

    #[test]
    fn get_net_worth_calculates_net_worth_correctly() {
        let stock_one = Stock {
            id: "test_1".to_string(),
            price: 2.0,
            history: vec![],
        };
        let stock_two = Stock {
            id: "test_2".to_string(),
            price: 1.0,
            history: vec![],
        };
        let stocks = Vec::from([stock_one.clone(), stock_two.clone()]);
        let initial_cash = 1000.0;
        let initial_amount = 100.0;
        let agent = Agent {
            name: "test",
            portfolio: HashMap::from([
                (stock_one.id, initial_amount),
                (stock_two.id, initial_amount),
            ]),
            cash: initial_cash,
            trades: Vec::new(),
            strategy: Box::new(MockStrategy),
        };

        let actual_net_worth = agent.get_net_worth(&stocks);

        assert_eq!(actual_net_worth, 1300.0)
    }
}
