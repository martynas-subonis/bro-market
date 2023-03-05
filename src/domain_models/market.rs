use crate::domain_models::stock::Stock;

const NUMBER_OF_STOCKS: usize = 5;

#[derive(Debug)]
pub struct Market {
    pub stocks: Vec<Stock>,
}

impl Market {
    pub fn new() -> Market {
        let mut stocks: Vec<Stock> = Vec::new();
        for _ in 0..NUMBER_OF_STOCKS {
            stocks.push(Stock::default())
        }
        Market { stocks }
    }
}
