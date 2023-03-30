use lib::DEFAULT_STARTING_CASH;
use ndarray::{Array, Ix1};

pub fn round_to_precision(num: u32, p: u32) -> u32 {
    let rounded = (num / p) * p;
    return rounded;
}

pub fn calc_stats(array: &Array<f64, Ix1>) -> (f64, f64) {
    let mean = array.mean().unwrap();
    let std = array.std(0.0);
    (mean, std)
}

pub fn calculate_networth_probability(net_worth_array: &Array<f64, Ix1>, multiplier: f64) -> f64 {
    net_worth_array
        .iter()
        .filter(|&x| *x > multiplier * DEFAULT_STARTING_CASH)
        .count() as f64
        / net_worth_array.len() as f64
        * 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_to_precision_rounds_correctly_for_500() {
        let cases = [(0, 0), (1, 0), (999, 500), (1000, 1000), (1001, 1000)];
        for case in cases {
            assert_eq!(round_to_precision(case.0, 500), case.1)
        }
    }
}
