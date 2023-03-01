use crate::pseudo_strategies::windows::{LARGE_WINDOW, MEDIUM_WINDOW, SMALL_WINDOW};
use crate::pseudo_strategies::partition::{get_partitions};
use linreg::{linear_regression};

pub fn is_head_and_shoulders(hours: &Vec<f64>, price_series: &Vec<f64>) -> Result<bool, &'static str> {
    let n: usize = price_series.len();
    let available_windows: Vec<usize> = vec![LARGE_WINDOW, MEDIUM_WINDOW, SMALL_WINDOW]
        .into_iter()
        .filter(|x| x <= &n)
        .collect();

    if available_windows.len() == 0 {
        return Ok(false);
    }

    for window in available_windows {
        if window % 6 != 0 {
            return Err("Window must be divisible by 6.");
        }

        if window > n {
            return Err("Window cannot be bigger than the series.");
        }

        let p = get_partitions(window, n);
        let mut slopes = [0.0; 6];

        for i in 0..p.len() - 1 {
            let fit: (f64, f64) = match linear_regression(&hours[p[i]..p[i + 1]], &price_series[p[i]..p[i + 1]]) {
                Ok(num) => num,
                _ => {
                    return Err("Failed to perform linear regression.");
                }
            };
            slopes[i] = fit.0
        }
        if slopes_match(slopes) {
            return Ok(true);
        }
    }
    return Ok(false);
}


pub fn slopes_match(slopes: [f64; 6]) -> bool {
    let k1 = slopes[0];
    let k2 = slopes[1];
    let k3 = slopes[2];
    let k4 = slopes[3];
    let k5 = slopes[4];
    let k6 = slopes[5];

    if k1 < 0.0 || k2 > 0.0 || k3 < 0.0 || k4 > 0.0 || k5 < 0.0 || k6 > 0.0 {
        return false;
    }


    // First shoulder check
    if exceeds_allowed_rel_diff(k1, k2) {
        return false;
    }

    // Head check
    if exceeds_allowed_rel_diff(k3, k4) {
        return false;
    }

    // Second shoulder check
    if exceeds_allowed_rel_diff(k5, k6) {
        return false;
    }

    // Check first shoulder rise with second shoulder rise
    if exceeds_allowed_rel_diff(k1, k5) {
        return false;
    }

    // Check first shoulder decline with second shoulder decline
    if exceeds_allowed_rel_diff(k2, k6) {
        return false;
    }

    return true;
}

const ALLOWED_RELATIVE_DIFF: f64 = 0.3;

pub fn exceeds_allowed_rel_diff(k0: f64, k1: f64) -> bool {
    return (
        (k0.abs() - k1.abs()).abs() / k0.abs()
    ) > ALLOWED_RELATIVE_DIFF;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slopes_match_returns_true_for_basic_true_case() {
        let slopes = [1.0, -1.0, 1.0, -1.0, 1.0, -1.0];

        let result = slopes_match(slopes);

        assert_eq!(result, true);
    }

    #[test]
    fn slopes_match_returns_true_for_true_steeper_head_case() {
        let slopes = [1.0, -1.0, 2.0, -2.0, 1.0, -1.0];

        let result = slopes_match(slopes);

        assert_eq!(result, true);
    }

    #[test]
    fn slopes_match_returns_true_for_true_minor_deviations() {
        let slopes = [1.0, -0.9, 1.9, -2.1, 1.1, -0.9];

        let result = slopes_match(slopes);

        assert_eq!(result, true);
    }

    #[test]
    fn slopes_match_returns_false_for_incorrect_slope_signs() {
        let incorrect_slopes = [
            [1.0, 1.0, 1.0, -1.0, 1.0, -1.0],
            [1.0, -1.0, 1.0, 1.0, 1.0, -1.0],
            [1.0, -1.0, 1.0, -1.0, 1.0, 1.0],
            [-1.0, -1.0, 1.0, -1.0, 1.0, -1.0],
            [1.0, -1.0, -1.0, -1.0, 1.0, -1.0],
            [1.0, -1.0, 1.0, -1.0, -1.0, -1.0]
        ];

        for slopes in incorrect_slopes {
            let result = slopes_match(slopes);
            assert_eq!(result, false);
        }
    }

    #[test]
    fn slopes_match_returns_false_for_higher_deviations_for_close_pairs() {
        let high_deviation_slopes = [
            [1.0, -0.7, 1.0, -1.0, 1.0, -1.0],
            [1.0, -1.0, 0.6, -1.0, 1.0, -1.0],
            [1.0, -1.0, 1.0, -1.0, 0.6, -1.0],
        ];

        for slopes in high_deviation_slopes {
            let result = slopes_match(slopes);
            assert_eq!(result, false);
        }
    }

    #[test]
    fn slopes_match_returns_false_for_higher_deviations_between_shoulders() {
        let high_deviation_slopes = [
            [0.7, -0.7, 1.0, -1.0, 1.0, -1.0],
            [1.0, -1.0, 1.0, -1.0, 0.6, -0.6],
        ];

        for slopes in high_deviation_slopes {
            let result = slopes_match(slopes);
            assert_eq!(result, false);
        }
    }
}
