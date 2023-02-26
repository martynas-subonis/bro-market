use crate::domain_models::stock::Stock;

const NUMBER_OF_STOCKS: usize = 5;

#[derive(Debug)]
pub struct Market {
    pub stocks: Vec<Stock>,
}

impl Market {
    pub fn new() -> Market {
        let stocks = vec![Stock::default(); NUMBER_OF_STOCKS];
        Market { stocks }
    }
}
