use std::collections::HashMap;
use crate::domain_models::stock::Stock;

const TRADE_FRACTION: f64 = 0.4;
const TRADING_FEE: f64 = 0.005;

#[derive(Debug, PartialEq)]
pub enum Strategies {
    HeadAndShoulders,
    ReverseHeadAndShoulders,
}

#[derive(Debug, PartialEq)]
pub struct Trade {
    pub id: String,
    pub amount: f64,
    pub time: usize,
}


#[derive(Debug)]
pub struct Agent<'a> {
    pub name: &'a str,
    pub portfolio: HashMap<String, f64>,
    pub cash: f64,
    pub strategies: Vec<Strategies>,
    pub trades: Vec<Trade>,
}

impl Agent<'_> {
    pub fn buy(&mut self, stock: &Stock, time: usize) -> () {
        let trade_amount = TRADE_FRACTION * self.cash;
        let fee_amount = TRADING_FEE * trade_amount;
        self.cash -= trade_amount;

        let buy_amount = (trade_amount - fee_amount) / stock.price;
        let stock_id = stock.id.clone();
        let stock_id_cl = stock_id.clone();

        if self.portfolio.contains_key(&stock_id) {
            if let Some(val) = self.portfolio.get_mut(&stock_id) {
                *val = *val + buy_amount;
            }
        } else {
            self.portfolio.insert(stock_id, buy_amount);
        }

        self.trades.push(Trade {
            id: stock_id_cl,
            amount: buy_amount,
            time,
        });
    }

    pub fn sell(&mut self, stock: &Stock, time: usize) -> () {
        let stock_id = stock.id.clone();
        let stock_id_clone = stock_id.clone();

        if self.portfolio.contains_key(&stock_id) {
            if let Some(val) = self.portfolio.get_mut(&stock_id) {
                let sell_amount = *val * stock.price;
                let fee_amount = TRADING_FEE * sell_amount;
                self.cash += sell_amount - fee_amount;
                self.trades.push(Trade {
                    id: stock_id,
                    amount: -sell_amount,
                    time,
                });
                self.portfolio.remove(&stock_id_clone);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::hash_map::Entry;
    use super::*;

    #[test]
    fn buys_first_stock() {
        let initial_cash = 5000.0;
        let mut agent = Agent {
            name: "test",
            portfolio: Default::default(),
            cash: initial_cash,
            strategies: vec![],
            trades: vec![],
        };
        let stock = Stock {
            id: "test".to_string(),
            price: 2.0,
            history: vec![],
        };
        agent.buy(&stock, 1);
        assert_eq!(agent.portfolio[&stock.id], 995.0);
        assert_eq!(agent.cash, 3000.0);
        assert!(agent.trades.iter().eq(
            vec![Trade {
                id: "test".to_string(),
                amount: 995.0,
                time: 1,
            }].iter()
        ));
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
            strategies: vec![],
            trades: vec![],
        };
        let time = 1 as usize;
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
            strategies: vec![],
            trades: vec![],
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
            strategies: vec![],
            trades: vec![],
        };
        agent.sell(&stock_two, 1);
        assert_eq!(agent.cash, initial_cash);
    }
}
