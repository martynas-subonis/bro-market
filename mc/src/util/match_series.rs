use crate::util::partitions::create_partitions;
use crate::util::windows::create_available_windows;
use lib::{ALLOWED_RELATIVE_DIFF, NUMBER_OF_HOURS};
use linreg::linear_regression;

pub fn match_series(
    price_series: &Vec<f64>,
    partitions_nums: usize,
    timeline: &[f64; NUMBER_OF_HOURS],
    matcher: fn(Vec<f64>) -> bool,
) -> bool {
    let n = price_series.len();
    let available_windows = create_available_windows(n);
    if available_windows.is_empty() {
        return false;
    }

    for window in available_windows {
        let partitions = create_partitions(window, price_series.len(), partitions_nums);
        if matcher(compute_partitions_slopes(
            price_series,
            timeline,
            partitions,
        )) {
            return true;
        }
    }
    false
}

fn compute_partitions_slopes(
    price_series: &[f64],
    timeline: &[f64; NUMBER_OF_HOURS],
    p: Vec<usize>,
) -> Vec<f64> {
    let mut slopes: Vec<f64> = Vec::new();
    for i in 0..p.len() - 1 {
        let fit: (f64, f64) =
            match linear_regression(&timeline[p[i]..p[i + 1]], &price_series[p[i]..p[i + 1]]) {
                Ok(num) => num,
                Err(e) => {
                    panic!(
                        "{}",
                        format!("Failed to perform linear regression. Error message: {}", e)
                    );
                }
            };
        slopes.push(fit.0);
    }
    slopes
}

pub fn exceeds_allowed_rel_diff(k0: f64, k1: f64) -> bool {
    ((k0.abs() - k1.abs()).abs() / k0.abs()) > ALLOWED_RELATIVE_DIFF
}
