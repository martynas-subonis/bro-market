use crate::util::partitions::get_partitions;
use crate::util::windows::get_available_windows;
use lib::NUMBER_OF_HOURS;
use linreg::linear_regression;

const ALLOWED_RELATIVE_DIFF: f64 = 0.3;

pub fn match_series(
    price_series: &Vec<f64>,
    partitions_nums: usize,
    timeline: &[f64; NUMBER_OF_HOURS],
    matcher: fn(Vec<f64>) -> bool,
) -> bool {
    let n = price_series.len();
    let available_windows = get_available_windows(n);
    if available_windows.len() == 0 {
        return false;
    }

    for window in available_windows {
        let partitions = get_partitions(window, price_series.len(), partitions_nums);
        if matcher(get_partition_slopes(&price_series, timeline, partitions)) {
            return true;
        }
    }
    return false;
}

fn get_partition_slopes(
    price_series: &Vec<f64>,
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
    return slopes;
}

pub fn exceeds_allowed_rel_diff(k0: f64, k1: f64) -> bool {
    return ((k0.abs() - k1.abs()).abs() / k0.abs()) > ALLOWED_RELATIVE_DIFF;
}
