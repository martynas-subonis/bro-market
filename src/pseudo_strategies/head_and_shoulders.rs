use crate::util::match_series::{match_series, exceeds_allowed_rel_diff};

const HEAD_AND_SHOULDERS_PARTITIONS: usize = 6;

pub fn is_head_and_shoulders(price_series: &Vec<f64>) -> bool {
    return match_series(price_series, HEAD_AND_SHOULDERS_PARTITIONS, slopes_match);
}

fn slopes_match(slopes: Vec<f64>) -> bool {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slopes_match_returns_true_for_basic_true_case() {
        let slopes = vec![1.0, -1.0, 1.0, -1.0, 1.0, -1.0];

        let result = slopes_match(slopes);

        assert_eq!(result, true);
    }

    #[test]
    fn slopes_match_returns_true_for_true_steeper_head_case() {
        let slopes = vec![1.0, -1.0, 2.0, -2.0, 1.0, -1.0];

        let result = slopes_match(slopes);

        assert_eq!(result, true);
    }

    #[test]
    fn slopes_match_returns_true_for_true_minor_deviations() {
        let slopes = vec![1.0, -0.9, 1.9, -2.1, 1.1, -0.9];

        let result = slopes_match(slopes);

        assert_eq!(result, true);
    }

    #[test]
    fn slopes_match_returns_false_for_incorrect_slope_signs() {
        let incorrect_slopes = [
            vec![1.0, 1.0, 1.0, -1.0, 1.0, -1.0],
            vec![1.0, -1.0, 1.0, 1.0, 1.0, -1.0],
            vec![1.0, -1.0, 1.0, -1.0, 1.0, 1.0],
            vec![-1.0, -1.0, 1.0, -1.0, 1.0, -1.0],
            vec![1.0, -1.0, -1.0, -1.0, 1.0, -1.0],
            vec![1.0, -1.0, 1.0, -1.0, -1.0, -1.0]
        ];

        for slopes in incorrect_slopes {
            let result = slopes_match(slopes);
            assert_eq!(result, false);
        }
    }

    #[test]
    fn slopes_match_returns_false_for_higher_deviations_for_close_pairs() {
        let high_deviation_slopes = [
            vec![1.0, -0.7, 1.0, -1.0, 1.0, -1.0],
            vec![1.0, -1.0, 0.6, -1.0, 1.0, -1.0],
            vec![1.0, -1.0, 1.0, -1.0, 0.6, -1.0],
        ];

        for slopes in high_deviation_slopes {
            let result = slopes_match(slopes);
            assert_eq!(result, false);
        }
    }

    #[test]
    fn slopes_match_returns_false_for_higher_deviations_between_shoulders() {
        let high_deviation_slopes = [
            vec![0.7, -0.7, 1.0, -1.0, 1.0, -1.0],
            vec![1.0, -1.0, 1.0, -1.0, 0.6, -0.6],
        ];

        for slopes in high_deviation_slopes {
            let result = slopes_match(slopes);
            assert_eq!(result, false);
        }
    }
}
