use std::collections::HashMap;
use crate::domain_models::stock::Stock;

const TRADE_FRACTION: f64 = 0.2;

#[derive(Debug, PartialEq)]
pub enum Strategies {
    HeadAndShoulders
}

#[derive(Debug)]
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
    fn trade(&mut self, stock: &Stock, time: usize, trade_sign: f64) -> () {
        let trade_amount = trade_sign * TRADE_FRACTION * self.cash;
        self.cash -= trade_amount;

        let stock_amount = trade_amount / stock.price;
        let stock_id = stock.id.clone();
        let stock_id_cl = stock_id.clone();

        if self.portfolio.contains_key(&stock_id) {
            let val = self.portfolio.get_mut(&stock_id);
        } else {
            self.portfolio.insert(stock_id, stock_amount);
        }

        self.trades.push( Trade {
            id: stock_id_cl,
            amount: trade_amount,
            time
        })
    }
}

impl Agent<'_> {
    pub fn buy(&mut self, stock: &Stock, time: usize) -> () {
        self.trade(stock, time, 1.0)
    }

    pub fn sell(&mut self, stock: &Stock, time: usize) -> () {
        self.trade(stock, time, -1.0)
    }
}
